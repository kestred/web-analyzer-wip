mod ty;

use crate::definitions::DefinitionDatabase;

pub use ty::*;

#[salsa::query_group(TypeDatabaseStorage)]
pub(crate) trait TypeDatabase: DefinitionDatabase {
    #[salsa::input]
    fn __placeholder__(&self, none: ()) -> ();
}
