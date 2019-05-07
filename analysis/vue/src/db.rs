use analysis_utils::FileDatabase;

#[salsa::database(
    analysis_utils::FileDatabaseStorage,
    crate::parse::ParseDatabaseStorage,
)]
#[derive(Debug)]
pub(crate) struct VueDatabase {
    runtime: salsa::Runtime<VueDatabase>,
}

impl salsa::Database for VueDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<VueDatabase> {
        &self.runtime
    }
}

impl Default for VueDatabase {
    fn default() -> VueDatabase {
        let mut db = VueDatabase {
            runtime: salsa::Runtime::default(),
        };
        db.set_package_graph(Default::default());
        db
    }
}
