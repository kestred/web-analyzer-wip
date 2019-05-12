use code_grammar::{syntax_kinds, SyntaxKind, SyntaxLanguage};

pub use html_grammar::syntax_kind::{self as html, *};

pub const VUE: SyntaxLanguage = SyntaxLanguage(4);

pub fn as_str(k: SyntaxKind) -> Option<&'static str> {
    html::as_str(k).or_else(|| self::nodes::as_str(k))
}

pub fn as_debug_repr(k: SyntaxKind) -> Option<SyntaxKindMeta> {
    html::as_debug_repr(k).or_else(|| self::nodes::as_debug_repr(k))
}

syntax_kinds! {
    language VUE;

    nodes {
        COMPONENT 100
        COMPONENT_TEMPLATE 101
        COMPONENT_SCRIPT 102
        COMPONENT_STYLE 103
        ATTRIBUTE_BINDING 104
        ATTRIBUTE_LISTENER 105
        ATTRIBUTE_MODIFIER 106
        ATTRIBUTE_KEY 107
    }
}
