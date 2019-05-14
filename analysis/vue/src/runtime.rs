use crate::Config;
use code_analysis::{FileId, LineIndex, PackageGraph, SourceChange, SourceRootId};
use code_grammar::TreeArc;
use html_grammar::ast as html;
use javascript_grammar::ast as js;
use typescript_grammar::ast as ts;
use vue_grammar::ast as vue;
use std::sync::Arc;

use crate::AstDatabase as _;
use crate::ConfigDatabase as _;
use code_analysis::SourceDatabase as _;
use html_analysis::AstDatabase as _;
use javascript_analysis::AstDatabase as _;
use typescript_analysis::AstDatabase as _;

use crate::AstDatabaseStorage as VueAstStorage;
use crate::ConfigDatabaseStorage as VueConfigStorage;
use code_analysis::SourceDatabaseStorage as SourceStorage;
use html_analysis::AstDatabaseStorage as HtmlAstStorage;
use javascript_analysis::AstDatabaseStorage as JsAstStorage;
use typescript_analysis::AstDatabaseStorage as TsAstStorage;

#[salsa::database(
    SourceStorage,
    HtmlAstStorage,
    VueAstStorage,
    VueConfigStorage,
    JsAstStorage,
    TsAstStorage,
)]
#[derive(Debug)]
pub(crate) struct HostDatabase {
    runtime: salsa::Runtime<HostDatabase>,
}

impl salsa::Database for HostDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<HostDatabase> {
        &self.runtime
    }
}

impl Default for HostDatabase {
    fn default() -> HostDatabase {
        let mut db = HostDatabase {
            runtime: salsa::Runtime::default(),
        };
        db.set_local_roots(Default::default());
        db.set_foreign_roots(Default::default());
        db.set_package_graph(Default::default());
        db
    }
}

#[derive(Debug, Default)]
pub struct Analysis {
    db: HostDatabase,
}

impl Analysis {
    // Creates an analysis instance for a single file, without any extenal dependencies.
    pub fn from_single_file(filename: String, text: String) -> (Analysis, FileId) {
        let file_id = FileId(0);
        let source_root = SourceRootId(0);
        let mut db = HostDatabase::default();
        let mut packages = PackageGraph::default();
        packages.add_package_root(file_id);
        let mut change = SourceChange::new();
        change.add_root(source_root, true);
        change.add_file(source_root, file_id, filename.into(), Arc::new(text));
        change.set_package_graph(packages);
        change.apply_to(&mut db);
        db.set_vue_config(source_root, Arc::new(Config::default()));
        (Analysis { db }, file_id)
    }

    pub fn set_config(&mut self, config: Config) {
        self.db.set_vue_config(SourceRootId(0), Arc::new(config));
    }

    pub fn apply_change(&mut self, change: SourceChange) {
        change.apply_to(&mut self.db);
    }

    /// Gets the text of the source file.
    pub fn file_text(&self, file_id: FileId) -> Arc<String> {
        self.db.file_text(file_id)
    }

    /// Computes the set of diagnostics for the given file.
    pub fn file_syntax_tree(&self, file_id: FileId) -> String {
        let file_ext = self.db.file_extension(file_id);
        let source_ext = file_ext.as_ref().map(|ext| ext.as_str()).unwrap_or("");
        let source_id = self.db.file_source(file_id);
        crate::debug::syntax_tree(&self.db, source_id, source_ext)
    }

    /// Gets the file's `LineIndex`: data structure to convert between absolute
    /// offsets and line/column representation.
    pub fn file_line_index(&self, file_id: FileId) -> Arc<LineIndex> {
        self.db.source_line_index(self.db.file_source(file_id))
    }

    /// Gets the html syntax tree of the file.
    pub fn parse_html(&self, file_id: FileId) -> TreeArc<html::Document> {
        self.db.html_ast(self.db.file_source(file_id)).clone()
    }

    /// Gets the javascript syntax tree of the file.
    pub fn parse_javacsript(&self, file_id: FileId) -> TreeArc<js::Program> {
        self.db.javascript_ast(self.db.file_source(file_id)).clone()
    }

    /// Gets the typescript syntax tree of the file.
    pub fn parse_typecsript(&self, file_id: FileId) -> TreeArc<ts::Program> {
        self.db.typescript_ast(self.db.file_source(file_id)).clone()
    }

    /// Gets the javascript syntax tree of the file.
    pub fn parse_vue(&self, file_id: FileId) -> TreeArc<vue::Component> {
        self.db.vue_ast(self.db.file_source(file_id)).clone()
    }

    /// Computes the set of diagnostics for the given file.
    pub fn diagnostics(&self, file_id: FileId) -> Vec<String> {
        crate::diagnostics::check(&self.db, file_id)
    }
}
