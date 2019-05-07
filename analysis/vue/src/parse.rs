mod input;
mod language;
mod stable;

use analysis_utils::{FileId, LineIndex, SourceDatabase};
use grammar_utils::{AstNode, SyntaxKind, TreeArc};
use html_grammar::ast as html;
use javascript_grammar::ast as javascript;
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
    fn source_map_html(&self, input_id: InputId) -> Arc<AstIdMap>;

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
            _ => None,
        },
        InputId::Script(script_id) => Some(db.lookup_script_id(script_id).language),
    }
}

pub(crate) fn parse_html(db: &dyn ParseDatabase, input_id: InputId) -> TreeArc<html::Document> {
    let text = db.input_text(input_id);
    html::Document::parse(&*text).0
}

pub(crate) fn parse_javascript(db: &dyn ParseDatabase, input_id: InputId) -> TreeArc<javascript::Program> {
    let text = db.input_text(input_id);
    javascript::Program::parse(&*text).0
}

pub(crate) fn source_map_html(db: &dyn ParseDatabase, input_id: InputId) -> Arc<AstIdMap> {
    let document = db.parse_html(input_id);
    Arc::new(AstIdMap::from_root(&document.syntax, |node| {
        if let Some(node) = html::Script::cast(node) {
            Some(&node.syntax)
        } else {
            None
        }
    }))
}
