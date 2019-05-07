use grammar_utils::{syntax_kinds, SyntaxKind, SyntaxLanguage};

pub use crate::generated::syntax_kind::*;
pub use grammar_utils::syntax_kind::*;

pub const JAVASCRIPT: SyntaxLanguage = SyntaxLanguage(2);

pub fn as_str(k: SyntaxKind) -> Option<&'static str> {
    self::default::as_str(k)
        .or(self::symbols::as_str(k))
        .or(self::keywords::as_str(k))
        .or(self::literals::as_str(k))

        // N.B. None of the nodes have a canonical str repr
        // .or(self::nodes::as_str(k))
}

pub fn as_debug_repr(k: SyntaxKind) -> Option<SyntaxKindMeta> {
    self::default::as_debug_repr(k)
        .or(self::symbols::as_debug_repr(k))
        .or(self::keywords::as_debug_repr(k))
        .or(self::literals::as_debug_repr(k))
        .or(self::nodes::as_debug_repr(k))
}

syntax_kinds! {
    language JAVASCRIPT;

    symbols {
        // Unusual javascript symbols
        SHU 10 (">>>")
        SHU_EQ 11 (">>>=")
        EQEQEQ 12 ("===")
        BANG_EQEQ 13 ("!==")
    }

    keywords {
        // Keywords defined in ES2015
        BREAK_KW 101 ("break")
        CASE_KW 102 ("case")
        CATCH_KW 103 ("catch")
        CLASS_KW 104 ("class")
        CONST_KW 105 ("const")
        CONTINUE_KW 106 ("continue")
        DEBUGGER_KW 107 ("debugger")
        DEFAULT_KW 108 ("default")
        DELETE_KW 109 ("delete")
        DO_KW 110 ("do")
        ELSE_KW 111 ("else")
        EXPORT_KW 113 ("export")
        EXTENDS_KW 114 ("extends")
        FALSE_KW 115 ("false")
        FINALLY_KW 118 ("finally")
        FOR_KW 119 ("for")
        FUNCTION_KW 120 ("function")
        IF_KW 121 ("if")
        IMPORT_KW 122 ("import")
        IN_KW 123 ("in")
        INSTANCEOF_KW 124 ("instanceof")
        NEW_KW 125 ("new")
        NULL_KW 126 ("null")
        RETURN_KW 130 ("return")
        SUPER_KW 131 ("super")
        SWITCH_KW 132 ("switch")
        THIS_KW 133 ("this")
        THROW_KW 134 ("throw")
        TRUE_KW 135 ("true")
        TRY_KW 136 ("try")
        TYPEOF_KW 137 ("typeof")
        VAR_KW 138 ("var")
        VOID_KW 139 ("void")
        WHILE_KW 140 ("while")
        WITH_KW 141 ("with")
        YIELD_KW 142 ("yield")

        // In ES2015, these are always reserved
        ENUM_KW 143 ("enum")

        // In ES2015, the following are only reserved when they are found in strict mode code:
        IMPLEMENTS_KW 144 ("implements")
        INTERFACE_KW 145 ("interface")
        LET_KW 146 ("let")
        PACKAGE_KW 147 ("package")
        PRIVATE_KW 148 ("private")
        PROTECTED_KW 149 ("protected")
        PUBLIC_KW 150 ("public")
        STATIC_KW 151 ("static")

        // In ES2015, the following are only reserved when they are found in module code:
        AWAIT_KW 152 ("await")

        // The following are reserved as future keywords by older ECMAScript specifications (ECMAScript 1 till 3).
        ABSTRACT_KW 153 ("abstract")
        BOOLEAN_KW 154 ("boolean")
        BYTE_KW 155 ("byte")
        CHAR_KW 156 ("char")
        DOUBLE_KW 157 ("double")
        FINAL_KW 158 ("final")
        FLOAT_KW 159 ("float")
        GOTO_KW 160 ("goto")
        INT_KW 161 ("int")
        LONG_KW 162 ("long")
        NATIVE_KW 163 ("native")
        SHORT_KW 164 ("short")
        SYNCHRONIZED_KW 165 ("synchronized")
        THROWS_KW 166 ("throws")
        TRANSIENT_KW 167 ("transient")
        VOLATILE_KW 168 ("volatile")

        // These are contextual keywords (which are treated as identifiers except in particular contexts)
        AS_KW 170 ("as")
        FROM_KW 171 ("from")
        GET_KW 172 ("get")
        SET_KW 173 ("set")
        OF_KW 174 ("of")
        ASYNC_KW 175 ("async")
    }

    literals {
        NUMBER_LITERAL 201
        STRING_LITERAL 202
        REGEXP_LITERAL 203
        TEMPLATE_LITERAL 204
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
        _ if k == LT_EQ => true,
        _ if k == GT_EQ => true,
        _ if k == EQ => true,
        _ if k == EQEQ => true,
        _ if k == EQEQEQ => true,
        _ if k == BANG => true,
        _ if k == BANG_EQ => true,
        _ if k == BANG_EQEQ => true,
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
        _ if k == TILDE => true,
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
        '~' => Some(TILDE),
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

        // Contextual keywords
        "as" => None,
        "from" => None,
        "get" => None,
        "set" => None,
        "of" => None,
        "async" => None,

        _ => None,
    }
}
