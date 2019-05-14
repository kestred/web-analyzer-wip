use code_analysis::{FileId, PackageGraph, SourceChange, SourceRootId};
use vue_analysis::Analysis;
use ra_vfs::{Vfs, VfsChange, RootEntry, Filter, RelativePath};
use std::path::{Path, PathBuf};

pub fn load(entrypoint: PathBuf) -> (Analysis, Vfs) {
    let mut change = SourceChange::new();

    let root = find_package_json(&entrypoint).parent().unwrap().to_path_buf();
    let roots = vec![IncludeFiles::root(root.clone())];
    let (mut vfs, vfs_roots) = Vfs::new(roots);
    for r in vfs_roots {
        let vfs_root_path = vfs.root2path(r);
        let is_local = vfs_root_path.starts_with(&root);
        change.add_root(SourceRootId(r.0.into()), is_local);
    }

    // Create package graph
    let mut package_graph = PackageGraph::default();
    let mut load = |path: &std::path::Path| {
        let vfs_file = vfs.load(path);
        vfs_file.map(|f| FileId(f.0.into()))
    };
    if let Some(file_id) = load(&entrypoint) {
        package_graph.add_package_root(file_id);
    }
    change.set_package_graph(package_graph);

    let mut analysis = Analysis::default();
    analysis.apply_change(change);

    // Wait until vfs has loaded all roots
    let receiver = vfs.task_receiver().clone();
    for task in receiver {
        vfs.handle_task(task);
        let fs_changes = process_changes(&mut vfs);
        analysis.apply_change(fs_changes);

        // TODO: Support more than 1 root
        break;
    }

    (analysis, vfs)
}

fn process_changes(vfs: &mut Vfs) -> SourceChange {
    let changes = vfs.commit_changes();
    let mut change = SourceChange::new();
    if changes.is_empty() {
        return change;
    }

    for c in changes {
        match c {
            VfsChange::AddRoot { root, files } => {
                for (file, path, text) in files {
                    change.add_file(
                        SourceRootId(root.0.into()),
                        FileId(file.0.into()),
                        path,
                        text,
                    );
                }
            }
            VfsChange::AddFile { root, file, path, text } => {
                change.add_file(SourceRootId(root.0.into()), FileId(file.0.into()), path, text);
            }
            VfsChange::RemoveFile { root, file, path } => {
                change.remove_file(SourceRootId(root.0.into()), FileId(file.0.into()), path)
            }
            VfsChange::ChangeFile { file, text } => {
                change.change_file(FileId(file.0.into()), text);
            }
        }
    }
    change
}
/// `IncludeFiles` is used to create a `RootEntry` for VFS
pub struct IncludeFiles {
    path: PathBuf,
}

impl IncludeFiles {
    pub fn root(path: PathBuf) -> RootEntry {
        RootEntry::from(IncludeFiles { path })
    }
}

impl Filter for IncludeFiles {
    fn include_dir(&self, dir_path: &RelativePath) -> bool {
        const COMMON_IGNORED_DIRS: &[&str] = &["node_modules", "target", ".git"];
        let is_ignored = dir_path.components().any(|c| COMMON_IGNORED_DIRS.contains(&c.as_str()));
        let hidden = dir_path.components().any(|c| c.as_str().starts_with("."));
        !is_ignored && !hidden
    }

    fn include_file(&self, file_path: &RelativePath) -> bool {
        match file_path.extension() {
            Some("js") |
            Some("ts") |
            Some("vue") => true,
            _ => false
        }
    }
}

impl std::convert::From<IncludeFiles> for RootEntry {
    fn from(v: IncludeFiles) -> RootEntry {
        RootEntry::new(v.path.clone(), Box::new(v))
    }
}

fn find_package_json(path: &Path) -> PathBuf {
    if path.ends_with("package.json") {
        return path.to_path_buf();
    }
    let mut curr = Some(path);
    while let Some(path) = curr {
        let candidate = path.join("package.json");
        if candidate.exists() {
            return candidate;
        }
        curr = path.parent();
    }
    panic!("can't find package.json at {}", path.display())
}
