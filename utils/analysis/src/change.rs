use crate::source::{SourceDatabase, FileId, PackageGraph, SourceRoot, SourceRootId};
use relative_path::RelativePathBuf;
use rustc_hash::FxHashMap;
use std::{fmt, sync::Arc};

#[derive(Default)]
pub struct SourceChange {
    new_roots: Vec<(SourceRootId, bool)>,
    roots_changed: FxHashMap<SourceRootId, RootChange>,
    files_changed: Vec<(FileId, Arc<String>)>,
    dependencies_added: Vec<DependencyData>,
    package_graph: Option<PackageGraph>,
}

impl SourceChange {
    pub fn new() -> SourceChange {
        SourceChange::default()
    }

    pub fn add_root(&mut self, root_id: SourceRootId, is_local: bool) {
        self.new_roots.push((root_id, is_local));
    }

    pub fn add_file(
        &mut self,
        root_id: SourceRootId,
        file_id: FileId,
        path: RelativePathBuf,
        text: Arc<String>,
    ) {
        let file = AddFile { file_id, path, text };
        self.roots_changed.entry(root_id).or_default().added.push(file);
    }

    pub fn set_package_graph(&mut self, graph: PackageGraph) {
        self.package_graph = Some(graph);
    }
}

#[derive(Debug)]
struct AddFile {
    file_id: FileId,
    path: RelativePathBuf,
    text: Arc<String>,
}

#[derive(Debug)]
struct RemoveFile {
    file_id: FileId,
    path: RelativePathBuf,
}

#[derive(Default)]
struct RootChange {
    added: Vec<AddFile>,
    removed: Vec<RemoveFile>,
}

impl fmt::Debug for RootChange {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("AnalysisChange")
            .field("added", &self.added.len())
            .field("removed", &self.removed.len())
            .finish()
    }
}

#[derive(Debug)]
pub struct DependencyData {
    root_id: SourceRootId,
    root_change: RootChange,
}

impl DependencyData {
    pub fn prepare(
        root_id: SourceRootId,
        files: Vec<(FileId, RelativePathBuf, Arc<String>)>,
    ) -> DependencyData {
        let mut root_change = RootChange::default();
        root_change.added = files
            .into_iter()
            .map(|(file_id, path, text)| AddFile { file_id, path, text })
            .collect();
        DependencyData { root_id, root_change }
    }
}

impl SourceChange {
    pub fn apply_to(self, db: &mut dyn SourceDatabase) {
        if !self.new_roots.is_empty() {
            let mut local_roots = Vec::clone(&db.local_roots());
            for (root_id, is_local) in self.new_roots {
                db.set_source_root(root_id, Default::default());
                if is_local {
                    local_roots.push(root_id);
                }
            }
            db.set_local_roots(Arc::new(local_roots));
        }

        for (root_id, root_change) in self.roots_changed {
            apply_root_change(db, root_id, root_change);
        }
        for (file_id, text) in self.files_changed {
            db.set_file_text(file_id, text)
        }
        if !self.dependencies_added.is_empty() {
            let mut dependencies = Vec::clone(&db.foreign_roots());
            for dependency in self.dependencies_added {
                dependencies.push(dependency.root_id);
                db.set_source_root(dependency.root_id, Default::default());
                // db.set_constant_dependency_symbols(dependency.root_id, Arc::new(dependency.symbol_index));
                apply_root_change(db, dependency.root_id, dependency.root_change);
            }
            db.set_foreign_roots(Arc::new(dependencies));
        }
        if let Some(package_graph) = self.package_graph {
            db.set_package_graph(Arc::new(package_graph))
        }
    }
}

fn apply_root_change(db: &mut dyn SourceDatabase, root_id: SourceRootId, root_change: RootChange) {
    let mut source_root = SourceRoot::clone(&db.source_root(root_id));
    for add_file in root_change.added {
        db.set_file_text(add_file.file_id, add_file.text);
        db.set_file_relative_path(add_file.file_id, add_file.path.clone());
        db.set_file_source_root(add_file.file_id, root_id);
        source_root.files.insert(add_file.path, add_file.file_id);
    }
    for remove_file in root_change.removed {
        db.set_file_text(remove_file.file_id, Default::default());
        source_root.files.remove(&remove_file.path);
    }
    db.set_source_root(root_id, Arc::new(source_root));
}
