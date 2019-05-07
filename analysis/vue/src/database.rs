use analysis_utils::SourceDatabase;

#[salsa::database(
    analysis_utils::SourceDatabaseStorage,
    crate::parse::ParseDatabaseStorage,
)]
#[derive(Debug)]
pub(crate) struct RootDatabase {
    runtime: salsa::Runtime<RootDatabase>,
}

impl salsa::Database for RootDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<RootDatabase> {
        &self.runtime
    }
}

impl Default for RootDatabase {
    fn default() -> RootDatabase {
        let mut db = RootDatabase {
            runtime: salsa::Runtime::default(),
        };
        db.set_package_graph(Default::default());
        db
    }
}
