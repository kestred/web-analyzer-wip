mod app;
mod ast;
mod config;
mod diagnostics;

pub use self::app::{AppDatabase, AppDatabaseStorage};
pub use self::ast::{AstDatabase, AstDatabaseStorage};
pub use self::config::{Config, ConfigDatabase, ConfigDatabaseStorage};

#[cfg(feature = "runtime")]
mod debug;
#[cfg(feature = "runtime")]
mod runtime;
#[cfg(feature = "runtime")]
pub use runtime::Analysis;

pub trait VueDatabase:
    crate::AppDatabase +
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
    T: crate::AppDatabase +
       crate::AstDatabase +
       crate::ConfigDatabase +
       code_analysis::SourceDatabase +
       html_analysis::AstDatabase +
       javascript_analysis::AstDatabase +
       typescript_analysis::AstDatabase
{
}
