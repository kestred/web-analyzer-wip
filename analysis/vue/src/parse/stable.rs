use crate::parse::InputId;
use analysis_utils::{Arena, impl_arena_id};
use grammar_utils::{AstNode, SyntaxKind, SyntaxNode, TextRange, TreeArc};
use std::{marker::PhantomData, hash::{Hash, Hasher}};

/// `AstId` points to an AST node in any file.
///
/// It is stable across reparses, and can be used as salsa key/value.
#[derive(Debug)]
pub(crate) struct AstId<N: AstNode> {
    input_id: InputId,
    input_ast_id: InputAstId<N>,
}

impl<N: AstNode> Clone for AstId<N> {
    fn clone(&self) -> AstId<N> {
        *self
    }
}
impl<N: AstNode> Copy for AstId<N> {}

impl<N: AstNode> PartialEq for AstId<N> {
    fn eq(&self, other: &Self) -> bool {
        (self.input_id, self.input_ast_id) == (other.input_id, other.input_ast_id)
    }
}
impl<N: AstNode> Eq for AstId<N> {}
impl<N: AstNode> Hash for AstId<N> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        (self.input_id, self.input_ast_id).hash(hasher);
    }
}

impl<N: AstNode> AstId<N> {
    pub(crate) fn input_id(&self) -> InputId {
        self.input_id
    }
}

/// `AstId` points to an AST node in a specific file.
#[derive(Debug)]
pub(crate) struct InputAstId<N: AstNode> {
    raw: AnonymousAstId,
    _ty: PhantomData<N>,
}

impl<N: AstNode> Clone for InputAstId<N> {
    fn clone(&self) -> InputAstId<N> {
        *self
    }
}
impl<N: AstNode> Copy for InputAstId<N> {}

impl<N: AstNode> PartialEq for InputAstId<N> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}
impl<N: AstNode> Eq for InputAstId<N> {}
impl<N: AstNode> Hash for InputAstId<N> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.raw.hash(hasher);
    }
}

impl<N: AstNode> InputAstId<N> {
    pub(crate) fn with_input_id(self, input_id: InputId) -> AstId<N> {
        AstId { input_id, input_ast_id: self }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct AnonymousAstId(u32);
impl_arena_id!(AnonymousAstId);

/// Maps items' `SyntaxNode`s to `AnonymousAstId`s and back.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AstIdMap {
    arena: Arena<AnonymousAstId, SyntaxNodePtr>,
}

impl AstIdMap {
    pub(crate) fn ast_id<N: AstNode>(&self, item: &N) -> InputAstId<N> {
        let ptr = SyntaxNodePtr::new(item.syntax());
        let raw = match self.arena.iter().find(|(_id, i)| **i == ptr) {
            Some((it, _)) => it,
            None => panic!(
                "Can't find {:?} in AstIdMap:\n{:?}",
                item.syntax(),
                self.arena.iter().map(|(_id, i)| i).collect::<Vec<_>>(),
            ),
        };

        InputAstId { raw, _ty: PhantomData }
    }

    pub(crate) fn from_root<V>(root: &SyntaxNode, visit: V) -> AstIdMap
    where
        V: Fn(&SyntaxNode) -> Option<&SyntaxNode>
    {
        let mut map = AstIdMap { arena: Arena::default() };
        // By walking the tree in bread-first order we make sure that parents
        // get lower ids then children. That is, adding a new child does not
        // change parent's id. This means that, say, adding a new function to a
        // trait does not change ids of top-level items, which helps caching.
        bfs(root, |it| {
            if let Some(node) = visit(it) {
                map.alloc(node);
            }
        });
        map
    }

    fn alloc(&mut self, item: &SyntaxNode) -> AnonymousAstId {
        self.arena.alloc(SyntaxNodePtr::new(item))
    }
}

/// A pointer to a syntax node inside a file. It can be used to remember a
/// specific node across reparses of the same file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SyntaxNodePtr {
    range: TextRange,
    kind: SyntaxKind,
}

impl SyntaxNodePtr {
    pub fn new(node: &SyntaxNode) -> SyntaxNodePtr {
        SyntaxNodePtr { range: node.range(), kind: node.kind() }
    }
}

/// Walks the subtree in bfs order, calling `f` for each node.
fn bfs(node: &SyntaxNode, mut f: impl FnMut(&SyntaxNode)) {
    let mut curr_layer = vec![node];
    let mut next_layer = vec![];
    while !curr_layer.is_empty() {
        curr_layer.drain(..).for_each(|node| {
            next_layer.extend(node.children());
            f(node);
        });
        std::mem::swap(&mut curr_layer, &mut next_layer);
    }
}
