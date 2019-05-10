use crate::parse::{AstId, AstIdMap, FileLikeId, ParseDatabase};
use analysis_utils::impl_intern_key;
use grammar_utils::AstNode;
use javascript_grammar::ast as js;
use std::sync::Arc;

#[salsa::query_group(DefinitionDatabaseStorage)]
pub(crate) trait DefinitionDatabase: ParseDatabase {
    fn decl_map_javascript(&self, file_id: FileLikeId) -> Arc<AstIdMap>;

    #[salsa::interned]
    fn intern_class(&self, loc: ItemLoc<js::Class>) -> ClassId;
    #[salsa::interned]
    fn intern_function(&self, loc: ItemLoc<js::Function>) -> FunctionId;
}

pub(crate) fn decl_map_javascript(db: &dyn ParseDatabase, file_id: FileLikeId) -> Arc<AstIdMap> {
    let program = db.parse_javascript(file_id);
    Arc::new(AstIdMap::from_root(&program.syntax, |node| {
        if let Some(node) = js::ClassDeclaration::cast(node) {
            Some(&node.syntax)
        } else if let Some(node) = js::ClassExpression::cast(node) {
            Some(&node.syntax)
        } else if let Some(node) = js::FunctionDeclaration::cast(node) {
            Some(&node.syntax)
        } else if let Some(node) = js::FunctionExpression::cast(node) {
            Some(&node.syntax)
        } else {
            None
        }
    }))
}


#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct ItemLoc<N: AstNode> {
    pub(crate) ast_id: AstId<N>,
}

impl<N: AstNode> Clone for ItemLoc<N> {
    fn clone(&self) -> ItemLoc<N> {
        ItemLoc { ast_id: self.ast_id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId(salsa::InternId);
impl_intern_key!(FunctionId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClassId(salsa::InternId);
impl_intern_key!(ClassId);
