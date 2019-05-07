//! The `analysis/sources` module defines `salsa` inputs and queries for processing source graphs.
//! It almost exactly copied from `rust-analyzer/packages/ra_db`, but is less rust specific.

use relative_path::RelativePathBuf;
use rowan::{SmolStr, TextRange, TextUnit};
use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::Arc;

/// Database which stores all significant input facts: source code and
/// dependencies. Everything else is derived from these queries.
#[salsa::query_group(FileDatabaseStorage)]
pub trait FileDatabase: std::fmt::Debug {
    /// Text of the file.
    #[salsa::input]
    fn file_text(&self, file_id: FileId) -> Arc<String>;
    /// Path to a file, relative to the root of its source root.
    #[salsa::input]
    fn file_relative_path(&self, file_id: FileId) -> RelativePathBuf;
    fn file_extension(&self, file_id: FileId) -> Option<SmolStr>;
    /// Source root of the file.
    #[salsa::input]
    fn file_source_root(&self, file_id: FileId) -> SourceRootId;
    /// Contents of the source root.
    #[salsa::input]
    fn source_root(&self, id: SourceRootId) -> Arc<SourceRoot>;
    fn source_root_libraries(&self, id: SourceRootId) -> Arc<Vec<PackageId>>;
    /// The package graph.
    #[salsa::input]
    fn package_graph(&self) -> Arc<PackageGraph>;
}

fn file_extension(db: &impl FileDatabase, file_id: FileId) -> Option<SmolStr> {
    db.file_relative_path(file_id).extension().map(|ext| ext.into())
}

fn source_root_libraries(db: &impl FileDatabase, id: SourceRootId) -> Arc<Vec<PackageId>> {
    let root = db.source_root(id);
    let graph = db.package_graph();
    let res = root.files
        .values()
        .filter_map(|&it| graph.package_id_for_package_root(it))
        .collect::<Vec<_>>();
    Arc::new(res)
}

/// `FileId` is an integer which uniquely identifies a file. File paths are
/// messy and system-dependent, so most of the code should work directly with
/// `FileId`, without inspecting the path. The mapping between `FileId` and path
/// and `SourceRoot` is constant. A file rename is represented as a pair of
/// deletion/creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct FilePosition {
    pub file_id: FileId,
    pub offset: TextUnit,
}

#[derive(Clone, Copy, Debug)]
pub struct FileRange {
    pub file_id: FileId,
    pub range: TextRange,
}

/// Files are grouped into source roots. A source root is a directory on the
/// file systems which is watched for changes. Typically it corresponds to a
/// single project/package/library/crate/module etc.
///
/// Source roots *might* be nested: in this case, a file belongs to
/// the nearest enclosing source root. Paths to files are always relative to a
/// source root, and the analyzer does not know the root path of the source root at
/// all. So, a file from one source root can't refer to a file in another source
/// root by path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceRootId(pub u32);

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct SourceRoot {
    pub files: FxHashMap<RelativePathBuf, FileId>,
}

/// `PackageGraph` is a bit of information which turns a set of text files into a
/// number of projects/libraries. Each package is defined by the `FileId` of its
/// root module, contextual information and the set of its dependencies.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageGraph {
    arena: FxHashMap<PackageId, PackageData>,
}

#[derive(Debug)]
pub struct CyclicDependencies;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
struct PackageData {
    file_id: FileId,
    dependencies: Vec<Dependency>,
}

impl PackageData {
    fn new(file_id: FileId) -> PackageData {
        PackageData { file_id, dependencies: Vec::new() }
    }

    fn add_dep(&mut self, name: SmolStr, package_id: PackageId) {
        self.dependencies.push(Dependency { name, package_id })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub package_id: PackageId,
    pub name: SmolStr,
}

impl Dependency {
    pub fn package_id(&self) -> PackageId {
        self.package_id
    }
}

impl PackageGraph {
    pub fn add_package_root(&mut self, file_id: FileId) -> PackageId {
        let package_id = PackageId(self.arena.len() as u32);
        let prev = self.arena.insert(package_id, PackageData::new(file_id));
        assert!(prev.is_none());
        package_id
    }

    pub fn add_dep(
        &mut self,
        from: PackageId,
        name: SmolStr,
        to: PackageId,
    ) -> Result<(), CyclicDependencies> {
        if self.dfs_find(from, to, &mut FxHashSet::default()) {
            return Err(CyclicDependencies);
        }
        Ok(self.arena.get_mut(&from).unwrap().add_dep(name, to))
    }

    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = PackageId> + 'a {
        self.arena.keys().map(|it| *it)
    }

    pub fn package_root(&self, package_id: PackageId) -> FileId {
        self.arena[&package_id].file_id
    }

    pub fn package_id_for_package_root(&self, file_id: FileId) -> Option<PackageId> {
        let (&package_id, _) = self.arena.iter().find(|(_package_id, data)| data.file_id == file_id)?;
        Some(package_id)
    }

    pub fn dependencies<'a>(
        &'a self,
        package_id: PackageId,
    ) -> impl Iterator<Item = &'a Dependency> + 'a {
        self.arena[&package_id].dependencies.iter()
    }

    /// Extends this package graph by adding a complete disjoint second package
    /// graph.
    pub fn extend(&mut self, other: PackageGraph) {
        let start = self.arena.len() as u32;
        self.arena.extend(other.arena.into_iter().map(|(id, mut data)| {
            let new_id = PackageId(id.0 + start);
            for dep in &mut data.dependencies {
                dep.package_id = PackageId(dep.package_id.0 + start);
            }
            (new_id, data)
        }));
    }

    fn dfs_find(&self, target: PackageId, from: PackageId, visited: &mut FxHashSet<PackageId>) -> bool {
        if !visited.insert(from) {
            return false;
        }

        for dep in self.dependencies(from) {
            let package_id = dep.package_id();
            if package_id == target {
                return true;
            }

            if self.dfs_find(target, package_id, visited) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::{FileId, PackageGraph, SmolStr};

    #[test]
    fn it_should_panic_because_of_cycle_dependencies() {
        let mut graph = PackageGraph::default();
        let package1 = graph.add_package_root(FileId(1u32));
        let package2 = graph.add_package_root(FileId(2u32));
        let package3 = graph.add_package_root(FileId(3u32));
        assert!(graph.add_dep(package1, SmolStr::new("package2"), package2).is_ok());
        assert!(graph.add_dep(package2, SmolStr::new("package3"), package3).is_ok());
        assert!(graph.add_dep(package3, SmolStr::new("package1"), package1).is_err());
    }

    #[test]
    fn it_works() {
        let mut graph = PackageGraph::default();
        let package1 = graph.add_package_root(FileId(1u32));
        let package2 = graph.add_package_root(FileId(2u32));
        let package3 = graph.add_package_root(FileId(3u32));
        assert!(graph.add_dep(package1, SmolStr::new("package2"), package2).is_ok());
        assert!(graph.add_dep(package2, SmolStr::new("package3"), package3).is_ok());
    }
}
