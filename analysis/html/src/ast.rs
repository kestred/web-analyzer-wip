use code_analysis::{AstId, AstIdMap, SourceDatabase, SourceId, impl_intern_key, impl_source_key};
use code_grammar::{AstNode, TreeArc};
use html_grammar::ast as html;
use std::sync::Arc;

#[salsa::query_group(AstDatabaseStorage)]
pub trait AstDatabase: SourceDatabase {
    /// Parses the source as an HTML document
    fn html_ast(&self, file_id: SourceId) -> TreeArc<html::Document>;
    fn html_source_map(&self, file_id: SourceId) -> Arc<AstIdMap>;

    #[salsa::interned]
    fn intern_script_tag(&self, script: ScriptTag) -> ScriptTagId;
}

pub fn html_ast(db: &impl AstDatabase, file_id: SourceId) -> TreeArc<html::Document> {
    let text = db.source_text(file_id);
    let (ast, _) = html::Document::parse(text.as_str());
    ast
}

pub fn html_source_map(db: &impl AstDatabase, file_id: SourceId) -> Arc<AstIdMap> {
    let document = db.html_ast(file_id);
    Arc::new(AstIdMap::from_root(&document.syntax, |node| {
        if let Some(node) = html::Script::cast(node) {
            Some(&node.syntax)
        } else if let Some(node) = html::Style::cast(node) {
            Some(&node.syntax)
        } else {
            None
        }
    }))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ScriptTagId(salsa::InternId);
impl_intern_key!(ScriptTagId);
impl_source_key!(ScriptTagId);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ScriptTag {
    /// The syntax node of the script tag.
    pub ast_id: AstId<html::Script>,
    /// The type of the script tag (e.g. `"application/javascript"`)
    pub type_: &'static str,
}
