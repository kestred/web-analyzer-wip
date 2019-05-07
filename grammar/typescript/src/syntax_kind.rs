use grammar_utils::{LanguageKind, SyntaxKind};

pub use javascript_grammar::syntax_kind::*;

pub const TYPESCRIPT: LanguageKind = LanguageKind(3);

pub const TYPE_KW: SyntaxKind = TYPESCRIPT.syntax_kind(101);

pub fn to_typescript_keyword(s: &str) -> Option<SyntaxKind> {
    match s {
        "type" => Some(TYPE_KW),
        _ => None,
    }
}