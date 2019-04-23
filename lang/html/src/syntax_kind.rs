use web_grammars_utils::{LanguageKind, SyntaxKind};

pub use web_grammars_utils::syntax_kind::*;

pub const HTML: LanguageKind = LanguageKind(1);

// Any text in the middle of the document, excluding whitespace
pub const RAW_TEXT: SyntaxKind = HTML.syntax_kind(1);
pub const QUOTED_STRING: SyntaxKind = HTML.syntax_kind(2);

// HTML Symbols
pub const L_ANGLE_BANG: SyntaxKind = HTML.syntax_kind(11); // '<!'
pub const L_ANGLE_SLASH: SyntaxKind = HTML.syntax_kind(12); // '</'
pub const R_ANGLE_SLASH: SyntaxKind = HTML.syntax_kind(13); // '/>'