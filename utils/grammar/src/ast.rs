use crate::syntax_error::SyntaxError;
use rowan::{SyntaxElement, SyntaxKind, SyntaxNode, TransparentNewType, TreeArc, WalkEvent};
use std::fmt::Write;

/// The main trait to go from untyped `SyntaxNode`  to a typed ast. The
/// conversion itself has zero runtime cost: ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode: TransparentNewType<Repr = SyntaxNode> + ToOwned<Owned = TreeArc<Self>> {
    fn cast(syntax: &SyntaxNode) -> Option<&Self> where Self: Sized;
    fn syntax(&self) -> &SyntaxNode;

    #[inline]
    fn downcast(ast: &impl AstNode) -> Option<&Self> where Self: Sized;
}

pub fn debug_dump<'a, Fmt>(node: &SyntaxNode, mut errors: Vec<SyntaxError>, fmt_debug: Fmt) -> String
where
    Fmt: Fn(SyntaxKind) -> &'a str
{
    errors.sort_by_key(|err| err.offset());
    let mut err_pos = 0;
    let mut level = 0;
    let mut buf = String::new();
    macro_rules! indent {
        () => {
            for _ in 0..level {
                buf.push_str("  ");
            }
        };
    }

    for event in node.preorder_with_tokens() {
        match event {
            WalkEvent::Enter(element) => {
                indent!();
                match element {
                    SyntaxElement::Node(node) => writeln!(buf, "{}@{:?}", fmt_debug(node.kind()), node.range()).unwrap(),
                    SyntaxElement::Token(token) => {
                        if token.text().is_heap_allocated() || token.text().trim().is_empty() {
                            writeln!(buf, "{}@{:?}", fmt_debug(token.kind()), token.range()).unwrap();
                        } else {
                            writeln!(buf, "{}@{:?}  {:?}", fmt_debug(token.kind()), token.range(), token.text().as_str()).unwrap();
                        }
                        let off = token.range().end();
                        while err_pos < errors.len() && errors[err_pos].offset() <= off {
                            indent!();
                            writeln!(buf, "err: `{}`", errors[err_pos].message).unwrap();
                            err_pos += 1;
                        }
                    }
                }
                level += 1;
            }
            WalkEvent::Leave(_) => level -= 1,
        }
    }

    assert_eq!(level, 0);
    for err in errors[err_pos..].iter() {
        writeln!(buf, "err: `{:?}`", err.message).unwrap();
    }

    buf
}