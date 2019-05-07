use grammar_utils::{syntax_kinds, LanguageKind, SyntaxKind};

pub use grammar_utils::syntax_kind::*;

pub const HTML: LanguageKind = LanguageKind(1);
pub const TAG_NAME: SyntaxKind = IDENTIFIER;
pub const TAG_CLOSE: SyntaxKind = R_ANGLE;

pub fn as_str(k: SyntaxKind) -> Option<&'static str> {
    self::default::as_str(k)
        .or(self::text::as_str(k))
        .or(self::symbols::as_str(k))
        .or(self::nodes::as_str(k))
}

pub fn as_debug_repr(k: SyntaxKind) -> Option<SyntaxKindMeta> {
    self::default::as_debug_repr(k)
        .or(self::text::as_debug_repr(k))
        .or(self::symbols::as_debug_repr(k))
        .or(self::nodes::as_debug_repr(k))
}

syntax_kinds! {
    language HTML;

    text {
        TEXT 1    // any text in the middle of the document, excluding whitespace
        QUOTED 2  // a quoted string (e.g. in an attribute <input id="hello">)
        SCRIPT_BODY 3  // non-plaintext inside a special tag (e.g. CSS or JS)
    }

    symbols {
        L_ANGLE_BANG 11 ("<!")
        L_ANGLE_SLASH 12 ("</") [TAG_OPEN]
        SLASH_R_ANGLE 13 ("/>") [TAG_SELF_CLOSE]
    }

    nodes {
        DOCUMENT 100
        DOCUMENT_TYPE 101
        ELEMENT 102
        ATTRIBUTE 103
        SCRIPT 104
    }
}
