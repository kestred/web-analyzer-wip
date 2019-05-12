use crate::{impl_arena_id, Arena, SourceId};
use code_grammar::{AstNode, SyntaxElement, SyntaxKind, SyntaxNode, TextRange};
use std::{marker::PhantomData, hash::{Hash, Hasher}};

/// `AstId` points to an AST node in any file.
///
/// It is stable across reparses, and can be used as salsa key/value.
#[derive(Debug)]
pub struct AstId<N: AstNode> {
    file_id: SourceId,
    local_ast_id: LocalAstId<N>,
}

impl<N: AstNode> Clone for AstId<N> {
    fn clone(&self) -> AstId<N> {
        *self
    }
}
impl<N: AstNode> Copy for AstId<N> {}

impl<N: AstNode> PartialEq for AstId<N> {
    fn eq(&self, other: &Self) -> bool {
        (self.file_id, self.local_ast_id) == (other.file_id, other.local_ast_id)
    }
}
impl<N: AstNode> Eq for AstId<N> {}
impl<N: AstNode> Hash for AstId<N> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        (self.file_id, self.local_ast_id).hash(hasher);
    }
}

impl<N: AstNode> AstId<N> {
    pub fn file_id(&self) -> SourceId {
        self.file_id
    }
}

/// `AstId` points to an AST node in a specific file.
#[derive(Debug)]
pub struct LocalAstId<N: AstNode> {
    raw: AnonymousAstId,
    _ty: PhantomData<N>,
}

impl<N: AstNode> Clone for LocalAstId<N> {
    fn clone(&self) -> LocalAstId<N> {
        *self
    }
}
impl<N: AstNode> Copy for LocalAstId<N> {}

impl<N: AstNode> PartialEq for LocalAstId<N> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}
impl<N: AstNode> Eq for LocalAstId<N> {}
impl<N: AstNode> Hash for LocalAstId<N> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.raw.hash(hasher);
    }
}

impl<N: AstNode> LocalAstId<N> {
    pub fn with_file_id(self, file_id: SourceId) -> AstId<N> {
        AstId { file_id, local_ast_id: self }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnonymousAstId(u32);
impl_arena_id!(AnonymousAstId);

/// Maps items' `SyntaxNode`s to `AnonymousAstId`s and back.
#[derive(Debug, PartialEq, Eq)]
pub struct AstIdMap {
    arena: Arena<AnonymousAstId, SyntaxNodePtr>,
}

impl AstIdMap {
    pub fn from_root<V>(root: &SyntaxNode, visit: V) -> AstIdMap
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

    pub fn ast_id<N: AstNode>(&self, item: &N) -> LocalAstId<N> {
        let ptr = SyntaxNodePtr::new(item.syntax());
        let raw = match self.arena.iter().find(|(_id, i)| **i == ptr) {
            Some((it, _)) => it,
            None => panic!(
                "Can't find {:?} in AstIdMap:\n{:?}",
                item.syntax(),
                self.arena.iter().map(|(_id, i)| i).collect::<Vec<_>>(),
            ),
        };

        LocalAstId { raw, _ty: PhantomData }
    }

    pub fn find_in_root<'r, T: AstNode>(&self, root: &'r SyntaxNode, id: AstId<T>) -> &'r T {
        let ptr = self.arena[id.local_ast_id.raw];
        let node = match root.covering_node(ptr.range) {
            SyntaxElement::Node(node) => node,
            SyntaxElement::Token(token) => token.parent(),
        };
        assert_eq!(node.kind(), ptr.kind);
        T::cast(node).unwrap()
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
