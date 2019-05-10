mod infer;
mod ty;

use crate::definitions::DefinitionDatabase;

pub(crate) use infer::*;
pub use ty::*;

#[salsa::query_group(TypeDatabaseStorage)]
pub(crate) trait TypeDatabase: DefinitionDatabase {
    #[salsa::input]
    fn __placeholder__(&self, none: ()) -> ();
}
