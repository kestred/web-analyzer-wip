use code_analysis::{AstId, AstIdMap, SourceDatabase, SourceId, impl_intern_key};
use code_grammar::{AstNode, TreeArc};
use javascript_grammar::ast as js;
use std::sync::Arc;

#[salsa::query_group(AstDatabaseStorage)]
pub trait AstDatabase: SourceDatabase {
    /// Parses the source as a javascript program.
    fn javascript_ast(&self, file_id: SourceId) -> TreeArc<js::Program>;
    fn javascript_source_map(&self, file_id: SourceId) -> Arc<AstIdMap>;

    #[salsa::interned]
    fn intern_class(&self, loc: DeclLoc<js::Class>) -> ClassId;
    #[salsa::interned]
    fn intern_function(&self, loc: DeclLoc<js::Function>) -> FunctionId;
}

pub fn javascript_ast(db: &impl AstDatabase, file_id: SourceId) -> TreeArc<js::Program> {
    let text = db.source_text(file_id);
    let (ast, _) = js::Program::parse(text.as_str());
    ast
}

pub fn javascript_source_map(db: &impl AstDatabase, file_id: SourceId) -> Arc<AstIdMap> {
    let program = db.javascript_ast(file_id);
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
pub struct DeclLoc<N: AstNode> {
    pub ast_id: AstId<N>,
}

impl<N: AstNode> Clone for DeclLoc<N> {
    fn clone(&self) -> DeclLoc<N> {
        DeclLoc { ast_id: self.ast_id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId(salsa::InternId);
impl_intern_key!(FunctionId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClassId(salsa::InternId);
impl_intern_key!(ClassId);
