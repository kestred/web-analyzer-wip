use web_grammars_utils::{syntax_kinds, LanguageKind, SyntaxKind};

pub use crate::generated::syntax_kind::*;
pub use web_grammars_utils::syntax_kind::*;

pub const JAVASCRIPT: LanguageKind = LanguageKind(2);

pub fn as_str(k: SyntaxKind) -> Option<&'static str> {
    self::default::as_str(k)
        .or(self::symbols::as_str(k))
        .or(self::keywords::as_str(k))
        .or(self::literals::as_str(k))
}

pub fn as_debug_repr(k: SyntaxKind) -> Option<SyntaxKindMeta> {
    self::default::as_debug_repr(k)
        .or(self::symbols::as_debug_repr(k))
        .or(self::keywords::as_debug_repr(k))
        .or(self::literals::as_debug_repr(k))
}

syntax_kinds! {
    language JAVASCRIPT;

    symbols {
        // Unusual javascript symbols
        SHU 50 (">>>")
        SHU_EQ 51 (">>>=")
        EQEQEQ 52 ("===")
        BANGEQEQ 53 ("!==")
    }

    keywords {
        // Keywords defined in ES2015
        BREAK_KW 101
        CASE_KW 102
        CATCH_KW 103
        CLASS_KW 104
        CONST_KW 105
        CONTINUE_KW 106
        DEBUGGER_KW 107
        DEFAULT_KW 108
        DELETE_KW 109
        DO_KW 110
        ELSE_KW 111
        EXPORT_KW 113
        EXTENDS_KW 114
        FALSE_KW 115
        FINALLY_KW 118
        FOR_KW 119
        FUNCTION_KW 120
        IF_KW 121
        IMPORT_KW 122
        IN_KW 123
        INSTANCEOF_KW 124
        NEW_KW 125
        NULL_KW 126
        RETURN_KW 130
        SUPER_KW 131
        SWITCH_KW 132
        THIS_KW 133
        THROW_KW 134
        TRUE_KW 135
        TRY_KW 136
        TYPEOF_KW 137
        VAR_KW 138
        VOID_KW 139
        WHILE_KW 140
        WITH_KW 141
        YIELD_KW 142

        // In ES2015, these are always reserved
        ENUM_KW 143

        // In ES2015, the following are only reserved when they are found in strict mode code:
        IMPLEMENTS_KW 144
        INTERFACE_KW 145
        LET_KW 146
        PACKAGE_KW 147
        PRIVATE_KW 148
        PROTECTED_KW 149
        PUBLIC_KW 150
        STATIC_KW 151

        // In ES2015, the following are only reserved when they are found in module code:
        AWAIT_KW 152

        // The following are reserved as future keywords by older ECMAScript specifications (ECMAScript 1 till 3).
        ABSTRACT_KW 153
        BOOLEAN_KW 154
        BYTE_KW 155
        CHAR_KW 156
        DOUBLE_KW 157
        FINAL_KW 158
        FLOAT_KW 159
        GOTO_KW 160
        INT_KW 161
        LONG_KW 162
        NATIVE_KW 163
        SHORT_KW 164
        SYNCHRONIZED_KW 165
        THROWS_KW 166
        TRANSIENT_KW 167
        VOLATILE_KW 168
    }

    literals {
        NUMBER_TOKEN 201
        STRING_TOKEN 202
        REGEXP_TOKEN 203
        TEMPLATE_TOKEN 204
    }
}

pub fn is_javascript_punct(k: SyntaxKind) -> bool {
    match k {
        _ if k == DOT => true,
        _ if k == SEMI => true,
        _ if k == COMMA => true,
        _ if k == COLON => true,
        _ if k == QUESTION => true,
        _ if k == L_PAREN => true,
        _ if k == R_PAREN => true, // N.B. amibiguous when distinguishing regexp
        _ if k == L_CURLY => true,
        _ if k == R_CURLY => true, // N.B. amibiguous when distinguishing regexp
        _ if k == L_BRACK => true,
        _ if k == R_BRACK => true, // N.B. ignore when distinguishing regexp
        _ if k == L_ANGLE => true,
        _ if k == R_ANGLE => true,
        _ if k == LTEQ => true,
        _ if k == GTEQ => true,
        _ if k == EQ => true,
        _ if k == EQEQ => true,
        _ if k == EQEQEQ => true,
        _ if k == BANG => true,
        _ if k == BANGEQ => true,
        _ if k == BANGEQEQ => true,
        _ if k == PLUS => true,
        _ if k == PLUS_EQ => true,
        _ if k == MINUS => true,
        _ if k == MINUS_EQ => true,
        _ if k == STAR => true,
        _ if k == STAR_EQ => true,
        _ if k == SLASH => true,
        _ if k == SLASH_EQ => true,
        _ if k == PERCENT => true,
        _ if k == PERCENT_EQ => true,
        _ if k == INCREMENT => true,
        _ if k == DECREMENT => true,
        _ if k == SHL => true,
        _ if k == SHL_EQ => true,
        _ if k == SHR => true,
        _ if k == SHR_EQ => true,
        _ if k == SHU => true,
        _ if k == SHU_EQ => true,
        _ if k == CARET => true,
        _ if k == CARET_EQ => true,
        _ if k == AMPERSAND => true,
        _ if k == AMPERSAND_EQ => true,
        _ if k == PIPE => true,
        _ if k == PIPE_EQ => true,
        _ if k == AND => true,
        _ if k == OR => true,
        _ if k == TILDA => true,
        _ => false,
    }
}

pub fn to_javascript_symbol(c: char) -> Option<SyntaxKind> {
    match c {
        '(' => Some(L_PAREN),
        ')' => Some(R_PAREN),
        '{' => Some(L_CURLY),
        '}' => Some(R_CURLY),
        '[' => Some(L_BRACK),
        ']' => Some(R_BRACK),
        ';' => Some(SEMI),
        ',' => Some(COMMA),
        '?' => Some(QUESTION),
        '~' => Some(TILDA),
        _ => None
    }
}

pub fn to_javascript_keyword(s: &str) -> Option<SyntaxKind> {
    match s {
        // ES2015 Keywords
        "break" => Some(BREAK_KW),
        "case" => Some(CASE_KW),
        "catch" => Some(CATCH_KW),
        "class" => Some(CLASS_KW),
        "const" => Some(CONST_KW),
        "continue" => Some(CONTINUE_KW),
        "debugger" => Some(DEBUGGER_KW),
        "default" => Some(DEFAULT_KW),
        "delete" => Some(DELETE_KW),
        "do" => Some(DO_KW),
        "else" => Some(ELSE_KW),
        "export" => Some(EXPORT_KW),
        "extends" => Some(EXTENDS_KW),
        "false" => Some(FALSE_KW),
        "finally" => Some(FINALLY_KW),
        "for" => Some(FOR_KW),
        "function" => Some(FUNCTION_KW),
        "if" => Some(IF_KW),
        "import" => Some(IMPORT_KW),
        "in" => Some(IN_KW),
        "instanceof" => Some(INSTANCEOF_KW),
        "new" => Some(NEW_KW),
        "null" => Some(NULL_KW),
        "return" => Some(RETURN_KW),
        "super" => Some(SUPER_KW),
        "switch" => Some(SWITCH_KW),
        "this" => Some(THIS_KW),
        "throw" => Some(THROW_KW),
        "true" => Some(TRUE_KW),
        "try" => Some(TRY_KW),
        "typeof" => Some(TYPEOF_KW),
        "var" => Some(VAR_KW),
        "while" => Some(WHILE_KW),
        "with" => Some(WITH_KW),
        "yield" => Some(YIELD_KW),

        // Reserved Keywords
        "enum" => Some(ENUM_KW),
        "implements" => Some(IMPLEMENTS_KW),
        "interface" => Some(INTERFACE_KW),
        "let" => Some(LET_KW),
        "package" => Some(PACKAGE_KW),
        "private" => Some(PRIVATE_KW),
        "protected" => Some(PROTECTED_KW),
        "public" => Some(PUBLIC_KW),
        "static" => Some(STATIC_KW),
        "await" => Some(AWAIT_KW),
        "abstract" => Some(ABSTRACT_KW),
        "boolean" => Some(BOOLEAN_KW),
        "byte" => Some(BYTE_KW),
        "char" => Some(CHAR_KW),
        "double" => Some(DOUBLE_KW),
        "final" => Some(FINAL_KW),
        "float" => Some(FLOAT_KW),
        "goto" => Some(GOTO_KW),
        "int" => Some(INT_KW),
        "long" => Some(LONG_KW),
        "native" => Some(NATIVE_KW),
        "short" => Some(SHORT_KW),
        "synchronized" => Some(SYNCHRONIZED_KW),
        "throws" => Some(THROWS_KW),
        "transient" => Some(TRANSIENT_KW),
        "volatile" => Some(VOLATILE_KW),

        _ => None,
    }
}