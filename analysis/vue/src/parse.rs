mod input;
mod language;
mod stable;

use analysis_utils::{LineIndex, SourceDatabase};
use grammar_utils::{AstNode, TreeArc};
use html_grammar::ast as html;
use javascript_grammar::ast as javascript;
use vue_grammar::ast as vue;
use std::sync::Arc;

pub(crate) use self::input::{FileLikeId, ScriptId, ScriptSource};
pub(crate) use self::language::SourceLanguage;
pub(crate) use self::stable::{AstId, AstIdMap};

#[salsa::query_group(ParseDatabaseStorage)]
pub(crate) trait ParseDatabase: SourceDatabase {
    fn input_text(&self, file_id: FileLikeId) -> Arc<String>;
    fn input_language(&self, file_id: FileLikeId) -> Option<SourceLanguage>;
    fn input_line_index(&self, file_id: FileLikeId) -> Arc<LineIndex>;
    fn parse_html(&self, file_id: FileLikeId) -> TreeArc<html::Document>;
    fn parse_javascript(&self, file_id: FileLikeId) -> TreeArc<javascript::Program>;
    fn parse_vue(&self, file_id: FileLikeId) -> TreeArc<vue::Component>;
    fn source_map_html(&self, file_id: FileLikeId) -> Arc<AstIdMap>;
    fn source_map_vue(&self, file_id: FileLikeId) -> Arc<AstIdMap>;

    #[salsa::interned]
    fn script_id(&self, script: ScriptSource) -> ScriptId;
}

pub(crate) fn input_line_index(db: &dyn ParseDatabase, file_id: FileLikeId) -> Arc<LineIndex> {
    let text = db.input_text(file_id);
    Arc::new(LineIndex::new(&*text))
}

pub(crate) fn input_text(db: &dyn ParseDatabase, file_id: FileLikeId) -> Arc<String> {
    match file_id {
        FileLikeId::File(file_id) => db.file_text(file_id),
        FileLikeId::Script(script_id) => {
            let script = db.lookup_script_id(script_id);
            let file_id = script.ast_id.file_id();
            let file_language = db.input_language(file_id);
            let (root, source_map) = match file_language {
                Some(SourceLanguage::Html) => (db.parse_html(file_id).syntax.to_owned(), db.source_map_html(file_id)),
                Some(SourceLanguage::Vue) => (db.parse_vue(file_id).syntax.to_owned(), db.source_map_vue(file_id)),
                _ => unreachable!(),
            };
            let block = source_map.find_in_root::<html::Script>(&root, script.ast_id);
            let content = block.syntax.first_token().unwrap();
            Arc::new(content.text().to_string())
        }
    }
}

pub(crate) fn input_language(db: &dyn ParseDatabase, file_id: FileLikeId) -> Option<SourceLanguage> {
    match file_id {
        FileLikeId::File(file_id) => match db.file_extension(file_id)?.as_str() {
            "htm" | "html" => Some(SourceLanguage::Html),
            "js" => Some(SourceLanguage::Javascript),
            "ts" => Some(SourceLanguage::Typescript),
            "vue" => Some(SourceLanguage::Vue),
            _ => None,
        },
        FileLikeId::Script(script_id) => Some(db.lookup_script_id(script_id).language),
    }
}

pub(crate) fn parse_html(db: &dyn ParseDatabase, file_id: FileLikeId) -> TreeArc<html::Document> {
    let text = db.input_text(file_id);
    let (ast, _) = html::Document::parse(text.as_str());
    ast
}

pub(crate) fn parse_javascript(db: &dyn ParseDatabase, file_id: FileLikeId) -> TreeArc<javascript::Program> {
    let text = db.input_text(file_id);
    let (ast, _) = javascript::Program::parse(text.as_str());
    ast
}

pub(crate) fn parse_vue(db: &dyn ParseDatabase, file_id: FileLikeId) -> TreeArc<vue::Component> {
    let text = db.input_text(file_id);
    let (ast, _) = vue::Component::parse(&*text);
    ast
}

pub(crate) fn source_map_html(db: &dyn ParseDatabase, file_id: FileLikeId) -> Arc<AstIdMap> {
    let document = db.parse_html(file_id);
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

pub(crate) fn source_map_vue(db: &dyn ParseDatabase, file_id: FileLikeId) -> Arc<AstIdMap> {
    let component = db.parse_vue(file_id);
    Arc::new(AstIdMap::from_root(&component.syntax, |node| {
        if let Some(node) = html::Script::cast(node) {
            Some(&node.syntax)
        } else if let Some(node) = html::Style::cast(node) {
            Some(&node.syntax)
        } else {
            None
        }
    }))
}
