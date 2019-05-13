use code_grammar::{syntax_kinds, SyntaxKind, SyntaxLanguage};
use javascript_grammar::syntax_kind as js;

pub use javascript_grammar::syntax_kind::*;

pub const TYPESCRIPT: SyntaxLanguage = SyntaxLanguage(3);

pub fn as_str(k: SyntaxKind) -> Option<&'static str> {
    js::as_str(k)
        .or_else(|| self::keywords::as_str(k))
        .or_else(|| self::nodes::as_str(k))
}

pub fn as_debug_repr(k: SyntaxKind) -> Option<SyntaxKindMeta> {
    js::as_debug_repr(k)
        .or_else(|| self::keywords::as_debug_repr(k))
        .or_else(|| self::nodes::as_debug_repr(k))
}

syntax_kinds! {
    language TYPESCRIPT;

    keywords {
        TYPE_KW 101 ("type")
    }

    nodes {
        INTERFACE_DECLARATION 201
        INTERFACE_PROPERTY 202
        ALIAS_DECLARATION 203
        ENUM_DECLARATION 204
        ENUM_VARIANT 205

        MEMBER_TYPE_EXPR 210
        GENERIC_TYPE_EXPR 211
        ARRAY_TYPE_EXPR 212
        UNION_TYPE_EXPR 213
        INTERSECTION_TYPE_EXPR 214
        CONDITIONAL_TYPE_EXPR 215
        INTERFACE_TYPE_EXPR 216
        FUNCTION_TYPE_EXPR 217
        TUPLE_TYPE_EXPR 218
        TYPEOF_TYPE_EXPR 219

        TYPE_ARGUMENT 230
        AS_EXPRESSION 231
        NOT_NULL_EXPRESSION 232
    }
}
