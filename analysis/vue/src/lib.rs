mod database;
mod diagnostics;
mod model;
mod parse;
mod syntax;

use analysis_utils::{FileId, LineIndex, PackageGraph, SourceChange, SourceDatabase, SourceRootId};
use grammar_utils::TreeArc;
use html_grammar::ast as html;
use javascript_grammar::ast as javascript;
use std::sync::Arc;

use self::database::RootDatabase;
use self::parse::{InputId, ParseDatabase, SourceLanguage};

#[derive(Debug)]
pub struct Analysis {
    db: RootDatabase,
}

impl Analysis {
    // Creates an analysis instance for a single file, without any extenal dependencies.
    pub fn from_single_file(text: String) -> (Analysis, FileId) {
        let mut db = RootDatabase::default();
        let source_root = SourceRootId(0);
        let mut change = SourceChange::new();
        change.add_root(source_root, true);
        let file_id = FileId(0);
        let mut package_graph = PackageGraph::default();
        package_graph.add_package_root(file_id);
        change.add_file(source_root, file_id, "main.rs".into(), Arc::new(text));
        change.set_package_graph(package_graph);
        change.apply_to(&mut db);
        (Analysis { db }, file_id)
    }

    /// Gets the text of the source file.
    pub fn file_text(&self, file_id: FileId) -> Arc<String> {
        self.db.file_text(file_id)
    }

    /// Gets the (sometimes inferred) programming language of the source file.
    pub fn file_language(&self, file_id: FileId) -> Option<SourceLanguage> {
        self.db.input_language(file_id.into())
    }

    /// Gets the file's `LineIndex`: data structure to convert between absolute
    /// offsets and line/column representation.
    pub fn file_line_index(&self, file_id: FileId) -> Arc<LineIndex> {
        self.db.input_line_index(file_id.into())
    }

    /// Gets the html syntax tree of the file.
    pub fn parse_html(&self, file_id: FileId) -> TreeArc<html::Document> {
        self.db.parse_html(file_id.into()).clone()
    }

    /// Gets the javascript syntax tree of the file.
    pub fn parse_javascript(&self, file_id: FileId) -> TreeArc<javascript::Program> {
        self.db.parse_javascript(file_id.into()).clone()
    }

    /// Computes the set of diagnostics for the given file.
    pub fn diagnostics(&self, file_id: FileId) -> Vec<String> {
        diagnostics::check(&self.db, file_id.into())
    }
}
