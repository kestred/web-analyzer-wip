use html_grammar::ast as html;
use javascript_grammar::ast as javascript;
use web_analysis_utils::{FileId, SourceDatabase};
use web_grammar_utils::TreeArc;

#[salsa::query_group(ParseDatabaseStorage)]
pub(crate) trait ParseDatabase: SourceDatabase {
    fn parse_html(&self, file_id: FileId) -> TreeArc<html::Document>;
    fn parse_javascript(&self, file_id: FileId) -> TreeArc<javascript::Program>;
}

pub(crate) fn parse_html(db: &dyn ParseDatabase, file_id: FileId) -> TreeArc<html::Document> {
    let text = db.file_text(file_id);
    html::Document::parse(&*text).0
}

pub(crate) fn parse_javascript(db: &dyn ParseDatabase, file_id: FileId) -> TreeArc<javascript::Program> {
    let text = db.file_text(file_id);
    javascript::Program::parse(&*text).0
}