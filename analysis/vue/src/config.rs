use code_analysis::SourceRootId;
use std::sync::Arc;

#[salsa::query_group(ConfigDatabaseStorage)]
pub trait ConfigDatabase {
    #[salsa::input]
    fn vue_config(&self, root: SourceRootId) -> Arc<Config>;
}

#[derive(Debug, Default)]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub(crate) global: ConfigGlobals
}

#[derive(Debug, Default)]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConfigGlobals {
    #[serde(default)]
    pub(crate) components: Vec<String>,
    #[serde(default)]
    pub(crate) filters: Vec<String>,
}