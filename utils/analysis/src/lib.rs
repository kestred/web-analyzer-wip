mod arena;
mod change;
mod line_index;
mod source;

pub use arena::{ArenaId, Arena, Interner};
pub use change::{SourceChange, DependencyData};
pub use line_index::{LineIndex, LineCol};
pub use relative_path::{RelativePath, RelativePathBuf};
pub use source::*;

#[cfg(feature = "ast")]
mod ast;
#[cfg(feature = "ast")]
pub use ast::{AstId, AstIdMap};

#[macro_export]
macro_rules! impl_intern_key {
    ($name:ident) => {
        impl salsa::InternKey for $name {
            fn from_intern_id(v: salsa::InternId) -> Self {
                $name(v)
            }
            fn as_intern_id(&self) -> salsa::InternId {
                self.0
            }
        }
    }
}
