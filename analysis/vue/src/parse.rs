mod input;
mod language;
mod stable;

use analysis_utils::{FileId, LineIndex, SourceDatabase};
use grammar_utils::{AstNode, SyntaxKind, TreeArc};
use html_grammar::ast as html;
use javascript_grammar::ast as javascript;
use vue_grammar::ast as vue;
use std::{marker::PhantomData, sync::Arc};

pub(crate) use self::input::{InputId, ScriptId, ScriptSource};
pub(crate) use self::language::SourceLanguage;
pub(crate) use self::stable::{AstId, AstIdMap};

#[salsa::query_group(ParseDatabaseStorage)]
pub(crate) trait ParseDatabase: SourceDatabase {
    fn input_text(&self, input_id: InputId) -> Arc<String>;
    fn input_language(&self, input_id: InputId) -> Option<SourceLanguage>;
    fn input_line_index(&self, input_id: InputId) -> Arc<LineIndex>;
    fn parse_html(&self, input_id: InputId) -> TreeArc<html::Document>;
    fn parse_javascript(&self, input_id: InputId) -> TreeArc<javascript::Program>;
    fn parse_vue(&self, input_id: InputId) -> TreeArc<vue::Component>;
    // fn source_map_html(&self, input_id: InputId) -> Arc<AstIdMap>;
    // fn source_map_vue(&self, input_id: InputId) -> Arc<AstIdMap>;

    #[salsa::interned]
    fn script_id(&self, script: ScriptSource) -> ScriptId;
}

pub(crate) fn input_line_index(db: &dyn ParseDatabase, input_id: InputId) -> Arc<LineIndex> {
    let text = db.input_text(input_id);
    Arc::new(LineIndex::new(&*text))
}

pub(crate) fn input_text(db: &dyn ParseDatabase, input_id: InputId) -> Arc<String> {
    match input_id {
        InputId::File(file_id) => db.file_text(file_id),
        InputId::Script(_) => unimplemented!(),
    }
}

pub(crate) fn input_language(db: &dyn ParseDatabase, input_id: InputId) -> Option<SourceLanguage> {
    match input_id {
        InputId::File(file_id) => match db.file_extension(file_id)?.as_str() {
            "htm" | "html" => Some(SourceLanguage::Html),
            "js" => Some(SourceLanguage::Javascript),
            "ts" => Some(SourceLanguage::Typescript),
            "vue" => Some(SourceLanguage::Vue),
            _ => None,
        },
        InputId::Script(script_id) => Some(db.lookup_script_id(script_id).language),
    }
}

pub(crate) fn parse_html(db: &dyn ParseDatabase, input_id: InputId) -> TreeArc<html::Document> {
    let text = db.input_text(input_id);
    let (ast, _) = html::Document::parse(&*text);
    ast
}

pub(crate) fn parse_javascript(db: &dyn ParseDatabase, input_id: InputId) -> TreeArc<javascript::Program> {
    let text = db.input_text(input_id);
    let (ast, _) = javascript::Program::parse(&*text);
    ast
}

pub(crate) fn parse_vue(db: &dyn ParseDatabase, input_id: InputId) -> TreeArc<vue::Component> {
    let text = db.input_text(input_id);
    let (ast, _) = vue::Component::parse(&*text);
    ast
}

// pub(crate) fn source_map_html(db: &dyn ParseDatabase, input_id: InputId) -> Arc<AstIdMap> {
//     let document = db.parse_html(input_id);
//     Arc::new(AstIdMap::from_root(&document.syntax, |node| {
//         if let Some(node) = html::Script::cast(node) {
//             Some(&node.syntax)
//         } else if let Some(node) = html::Style::cast(node) {
//             Some(&node.style)
//         } else {
//             None
//         }
//     }))
// }

// pub(crate) fn source_map_vue(db: &dyn ParseDatabase, input_id: InputId) -> Arc<AstIdMap> {
//     let document = db.parse_html(input_id);
//     Arc::new(AstIdMap::from_root(&document.syntax, |node| {
//         if let Some(node) = vue::ComponentScript::cast(node) {
//             Some(&node.syntax)
//         } else if let Some(node) = vue::ComponentStyle::cast(node) {
//             Some(&node.style)
//         } else {
//             None
//         }
//     }))
// }
