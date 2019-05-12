use code_grammar::{syntax_kinds, SyntaxKind, SyntaxLanguage};

pub use code_grammar::syntax_kind::*;

pub const HTML: SyntaxLanguage = SyntaxLanguage(1);
pub const TAG_NAME: SyntaxKind = IDENTIFIER;

pub fn as_str(k: SyntaxKind) -> Option<&'static str> {
    self::default::as_str(k)
        .or_else(|| self::text::as_str(k))
        .or_else(|| self::symbols::as_str(k))
        .or_else(|| self::nodes::as_str(k))
}

pub fn as_debug_repr(k: SyntaxKind) -> Option<SyntaxKindMeta> {
    self::default::as_debug_repr(k)
        .or_else(|| self::text::as_debug_repr(k))
        .or_else(|| self::symbols::as_debug_repr(k))
        .or_else(|| self::nodes::as_debug_repr(k))
}

syntax_kinds! {
    language HTML;

    text {
        /// Any text in the middle of the document
        TEXT 1
        /// A quoted string (e.g. in an attribute `<input id="hello">`)
        QUOTED 2
        /// Delimited template interoplation (e.g. in mustache templates `{{ myVariable }}`)
        DELIMITED 3 [MUSTACHE]
        /// The contents inside a style tag
        STYLE_CONTENT 4
        /// The contents inside a script tag
        SCRIPT_CONTENT 5
    }

    symbols {
        L_ANGLE_BANG 11 ("<!")
        L_ANGLE_SLASH 12 ("</")
        SLASH_R_ANGLE 13 ("/>")
    }

    nodes {
        DOCUMENT 100
        DOCUMENT_TYPE 101
        ELEMENT 102
        ATTRIBUTE 103
        STYLE_BLOCK 105
        SCRIPT_BLOCK 106
    }
}
