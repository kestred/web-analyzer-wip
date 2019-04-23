use web_grammars_utils::{LanguageKind, SyntaxKind};

pub use web_grammars_utils::syntax_kind::*;

pub const JAVASCRIPT: LanguageKind = LanguageKind(2);

// Unusual javascript symbols
pub const SHU: SyntaxKind = JAVASCRIPT.syntax_kind(50); // '>>>'
pub const SHU_EQ: SyntaxKind = JAVASCRIPT.syntax_kind(51); // '>>>='
pub const EQEQEQ: SyntaxKind = JAVASCRIPT.syntax_kind(52); // '==='
pub const BANGEQEQ: SyntaxKind = JAVASCRIPT.syntax_kind(53); // '!=='

// Keywords defined in ES2015
pub const BREAK_KW: SyntaxKind = JAVASCRIPT.syntax_kind(101);
pub const CASE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(102);
pub const CATCH_KW: SyntaxKind = JAVASCRIPT.syntax_kind(103);
pub const CLASS_KW: SyntaxKind = JAVASCRIPT.syntax_kind(104);
pub const CONST_KW: SyntaxKind = JAVASCRIPT.syntax_kind(105);
pub const CONTINUE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(106);
pub const DEBUGGER_KW: SyntaxKind = JAVASCRIPT.syntax_kind(107);
pub const DEFAULT_KW: SyntaxKind = JAVASCRIPT.syntax_kind(108);
pub const DELETE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(109);
pub const DO_KW: SyntaxKind = JAVASCRIPT.syntax_kind(110);
pub const ELSE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(111);
pub const EXPORT_KW: SyntaxKind = JAVASCRIPT.syntax_kind(113);
pub const EXTENDS_KW: SyntaxKind = JAVASCRIPT.syntax_kind(114);
pub const FALSE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(115);
pub const FINALLY_KW: SyntaxKind = JAVASCRIPT.syntax_kind(118);
pub const FOR_KW: SyntaxKind = JAVASCRIPT.syntax_kind(119);
pub const FUNCTION_KW: SyntaxKind = JAVASCRIPT.syntax_kind(120);
pub const IF_KW: SyntaxKind = JAVASCRIPT.syntax_kind(121);
pub const IMPORT_KW: SyntaxKind = JAVASCRIPT.syntax_kind(122);
pub const IN_KW: SyntaxKind = JAVASCRIPT.syntax_kind(123);
pub const INSTANCEOF_KW: SyntaxKind = JAVASCRIPT.syntax_kind(124);
pub const NEW_KW: SyntaxKind = JAVASCRIPT.syntax_kind(125);
pub const NULL_KW: SyntaxKind = JAVASCRIPT.syntax_kind(126);
pub const RETURN_KW: SyntaxKind = JAVASCRIPT.syntax_kind(130);
pub const SUPER_KW: SyntaxKind = JAVASCRIPT.syntax_kind(131);
pub const SWITCH_KW: SyntaxKind = JAVASCRIPT.syntax_kind(132);
pub const THIS_KW: SyntaxKind = JAVASCRIPT.syntax_kind(133);
pub const THROW_KW: SyntaxKind = JAVASCRIPT.syntax_kind(134);
pub const TRUE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(135);
pub const TRY_KW: SyntaxKind = JAVASCRIPT.syntax_kind(136);
pub const TYPEOF_KW: SyntaxKind = JAVASCRIPT.syntax_kind(137);
pub const VAR_KW: SyntaxKind = JAVASCRIPT.syntax_kind(138);
pub const VOID_KW: SyntaxKind = JAVASCRIPT.syntax_kind(139);
pub const WHILE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(140);
pub const WITH_KW: SyntaxKind = JAVASCRIPT.syntax_kind(141);
pub const YIELD_KW: SyntaxKind = JAVASCRIPT.syntax_kind(142);

// In ES2015, these are always reserved
pub const ENUM_KW: SyntaxKind = JAVASCRIPT.syntax_kind(143);

// In ES2015, the following are only reserved when they are found in strict mode code:
pub const IMPLEMENTS_KW: SyntaxKind = JAVASCRIPT.syntax_kind(144);
pub const INTERFACE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(145);
pub const LET_KW: SyntaxKind = JAVASCRIPT.syntax_kind(146);
pub const PACKAGE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(147);
pub const PRIVATE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(148);
pub const PROTECTED_KW: SyntaxKind = JAVASCRIPT.syntax_kind(149);
pub const PUBLIC_KW: SyntaxKind = JAVASCRIPT.syntax_kind(150);
pub const STATIC_KW: SyntaxKind = JAVASCRIPT.syntax_kind(151);

// In ES2015, the following are only reserved when they are found in module code:
pub const AWAIT_KW: SyntaxKind = JAVASCRIPT.syntax_kind(152);

// The following are reserved as future keywords by older ECMAScript specifications (ECMAScript 1 till 3).
pub const ABSTRACT_KW: SyntaxKind = JAVASCRIPT.syntax_kind(153);
pub const BOOLEAN_KW: SyntaxKind = JAVASCRIPT.syntax_kind(154);
pub const BYTE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(155);
pub const CHAR_KW: SyntaxKind = JAVASCRIPT.syntax_kind(156);
pub const DOUBLE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(157);
pub const FINAL_KW: SyntaxKind = JAVASCRIPT.syntax_kind(158);
pub const FLOAT_KW: SyntaxKind = JAVASCRIPT.syntax_kind(159);
pub const GOTO_KW: SyntaxKind = JAVASCRIPT.syntax_kind(160);
pub const INT_KW: SyntaxKind = JAVASCRIPT.syntax_kind(161);
pub const LONG_KW: SyntaxKind = JAVASCRIPT.syntax_kind(162);
pub const NATIVE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(163);
pub const SHORT_KW: SyntaxKind = JAVASCRIPT.syntax_kind(164);
pub const SYNCHRONIZED_KW: SyntaxKind = JAVASCRIPT.syntax_kind(165);
pub const THROWS_KW: SyntaxKind = JAVASCRIPT.syntax_kind(166);
pub const TRANSIENT_KW: SyntaxKind = JAVASCRIPT.syntax_kind(167);
pub const VOLATILE_KW: SyntaxKind = JAVASCRIPT.syntax_kind(168);

// Literals
pub const NUMBER_LIT: SyntaxKind = JAVASCRIPT.syntax_kind(201);
pub const STRING_LIT: SyntaxKind = JAVASCRIPT.syntax_kind(202);
pub const REGEXP_LIT: SyntaxKind = JAVASCRIPT.syntax_kind(203);
pub const TEMPLATE_LIT: SyntaxKind = JAVASCRIPT.syntax_kind(204);

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