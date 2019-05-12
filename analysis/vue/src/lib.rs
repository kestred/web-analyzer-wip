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
    code_analysis::SourceDatabase +
    html_analysis::AstDatabase +
    javascript_analysis::AstDatabase
{
}

impl<T> VueDatabase for T
where
    T: crate::AstDatabase +
       code_analysis::SourceDatabase +
       html_analysis::AstDatabase +
       javascript_analysis::AstDatabase
{
}
