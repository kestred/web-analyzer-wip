mod input;
mod stable;

use analysis_utils::{FileId, FileDatabase};
use grammar_utils::{LanguageKind, SyntaxKind, TreeArc};
use html_grammar::ast as html;
use javascript_grammar::ast as javascript;
use std::{marker::PhantomData, sync::Arc};

pub(crate) use self::input::{InputId, ScriptId}
pub(crate) use self::stable::{AstId, AstMapId};

#[salsa::query_group(ParseDatabaseStorage)]
pub(crate) trait ParseDatabase: FileDatabase {
    fn input_text(&self, input_id: InputId) -> Arc<String>;
    fn input_language(&self, input_id: InputId) -> Option<LanguageKind>;
    fn parse_html(&self, input_id: InputId) -> TreeArc<html::Document>;
    fn parse_javascript(&self, input_id: InputId) -> TreeArc<javascript::Program>;
    fn source_map_html(&self, input_id: InputId) -> Arc<AstIdMap>;

    #[salsa::interned]
    fn script_id(&self, script: ScriptDefinition) -> ScriptId;
}

pub(crate) fn input_text(db: &dyn ParseDatabase, input_id: InputId) -> Arc<String> {
    match input_id {
        InputId::File(file_id) => db.file_text(file_id),
        InputId::Script(_) => unimplemented!(),
    }
}

pub(crate) fn input_language(db: &dyn ParseDatabase, input_id: InputId) -> Option<LanguageKind> {
    match input_id {
        InputId::File(file_id) => match db.file_exension(file_id) {
            "htm" | "html" => Some(html_grammar::syntax_kind::HTML),
            "js" => Some(javascript_grammar::syntax_kind::JAVASCRIPT),
            _ => None,
        }
        InputId::Script(script_id) => db.lookup_script_id(script_id).language,
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
    Arc::new(AstIdMap::from_root(&document.syntax), |node| {
        if let Some(node) = ScriptContent::cast(node) {
            Some(node.syntax)
        } else {
            None
        }
    })
}
