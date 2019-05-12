use code_analysis::{AstId, AstIdMap, SourceDatabase, Source, SourceId, impl_intern_key, impl_source_key};
use code_grammar::{AstNode, TreeArc};
use vue_grammar::ast as vue;
use std::sync::Arc;

#[salsa::query_group(AstDatabaseStorage)]
pub trait AstDatabase: SourceDatabase {
    /// Parses the source as a vue single file component.
    fn vue_ast(&self, file_id: SourceId) -> TreeArc<vue::Component>;
    fn vue_source_map(&self, file_id: SourceId) -> Arc<AstIdMap>;

    fn component_script(&self, file_id: SourceId) -> Option<(SourceId, &'static str)>;
    #[salsa::interned]
    fn intern_component_script(&self, script: ComponentScript) -> ComponentScriptId;
}

pub fn vue_ast(db: &impl AstDatabase, file_id: SourceId) -> TreeArc<vue::Component> {
    let text = db.source_text(file_id);
    let (ast, _) = vue::Component::parse(&*text);
    ast
}

pub fn vue_source_map(db: &impl AstDatabase, file_id: SourceId) -> Arc<AstIdMap> {
    let component = db.vue_ast(file_id);
    Arc::new(AstIdMap::from_root(&component.syntax, |node| {
        if let Some(node) = vue::Script::cast(node) {
            Some(&node.syntax)
        } else if let Some(node) = vue::Style::cast(node) {
            Some(&node.syntax)
        } else {
            None
        }
    }))
}

pub fn component_script(db: &impl AstDatabase, file_id: SourceId) -> Option<(SourceId, &'static str)> {
    let source_map = db.vue_source_map(file_id);
    let component = db.vue_ast(file_id);
    let script = component.script()?;
    let script_id = db.intern_component_script(ComponentScript {
        ast_id: source_map.ast_id(script).with_file_id(file_id),
        lang: "js", // TODO: detect source language (e.g. handle `lang="ts"` attribute)
    });
    let content = script.script()?.source()?.text().to_string();
    let source = Source::from_source_key(script_id, content.into());
    let source_id = db.source_id(source);
    Some((source_id, "js"))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ComponentScriptId(salsa::InternId);
impl_intern_key!(ComponentScriptId);
impl_source_key!(ComponentScriptId);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ComponentScript {
    /// The syntax node of the script tag.
    pub ast_id: AstId<vue::Script>,
    /// The `lang` attribute of the script
    pub lang: &'static str,
}
