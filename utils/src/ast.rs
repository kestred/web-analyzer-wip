pub use rowan::{SyntaxNode, TransparentNewType, TreeArc};

/// The main trait to go from untyped `SyntaxNode`  to a typed ast. The
/// conversion itself has zero runtime cost: ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode: TransparentNewType<Repr = SyntaxNode> + ToOwned<Owned = TreeArc<Self>>
{
    fn cast(syntax: &SyntaxNode) -> Option<&Self>
    where
        Self: Sized;
    fn syntax(&self) -> &SyntaxNode;
}
