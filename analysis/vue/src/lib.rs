mod ast;
mod diagnostics;

pub use self::ast::{AstDatabase, AstDatabaseStorage};

#[cfg(feature = "runtime")]
mod debug;
#[cfg(feature = "runtime")]
mod runtime;
#[cfg(feature = "runtime")]
pub use runtime::Analysis;

pub trait VueDatabase:
    crate::AstDatabase +
    crate::ConfigDatabase +
    code_analysis::SourceDatabase +
    html_analysis::AstDatabase +
    javascript_analysis::AstDatabase +
    typescript_analysis::AstDatabase
{
}

impl<T> VueDatabase for T
where
    T: crate::AstDatabase +
       crate::ConfigDatabase +
       code_analysis::SourceDatabase +
       html_analysis::AstDatabase +
       javascript_analysis::AstDatabase +
       typescript_analysis::AstDatabase
{
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

#[salsa::query_group(ConfigDatabaseStorage)]
pub trait ConfigDatabase {
    #[salsa::input]
    fn vue_config(&self, root: code_analysis::SourceRootId) -> std::sync::Arc<Config>;
}
