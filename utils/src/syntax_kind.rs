mod language_kind;
mod meta;

use crate::syntax_kinds;

pub use self::language_kind::LanguageKind;
pub use self::meta::SyntaxKindMeta;

const DEFAULT: LanguageKind = LanguageKind(0);

syntax_kinds! {
    language DEFAULT;

    default {
        TOMBSTONE 0
        ERROR 1
        EOF 2
        WHITESPACE 3
        COMMENT 4
        IDENTIFIER 5 [IDENT]
        SHEBANG 6 ("#!/")
        L_PAREN 7 ("(")
        R_PAREN 8 (")")
        L_CURLY 9 ("{") [R_BRACE]
        R_CURLY 10 ("}") [L_BRACE]
        L_ANGLE 11 ("<") [LT]
        R_ANGLE 12 (">") [GT]
        L_SQUARE 13 ("[") [L_BRACK]
        R_SQUARE 14 ("]") [R_BRACK]
        SHL 15 ("<<")
        SHL_EQ 16 ("<<=")
        SHR 17 (">>")
        SHR_EQ 18 (">>=")
        AND 19 ("&&")
        OR 20 ("||")
        INCREMENT 21 ("++")
        DECREMENT 22 ("--")
        THIN_ARROW 23 ("->")
        FAT_ARROW 24 ("=>")
        EQ 25 ("=")
        EQEQ 26 ("==")
        BANG 27 ("!")
        BANG_EQ 28 ("!=")
        GT_EQ 29 (">=")
        LT_EQ 30 ("<=")
        SEMICOLON 31 (";") [SEMI]
        COLON 32 (":")
        COLONCOLON 33 ("::") [CONS]
        COMMA 34 (",")
        DOT 35 (".")
        DOTDOT 36 ("..")
        DOTDOTDOT 37 ("...")
        ASTERISK 38 ("*") [STAR]
        ASTERISK_EQ 39 ("*=") [STAR_EQ]
        SLASH 40 ("/")
        SLASH_EQ 41 ("/=")
        PERCENT 42 ("%")
        PERCENT_EQ 43 ("%=")
        PLUS 44 ("+")
        PLUS_EQ 45 ("+=")
        MINUS 46 ("-")
        MINUS_EQ 47 ("-=")
        AMPERSAND 48 ("&")
        AMPERSAND_EQ 49 ("&=")
        PIPE 50 ("|")
        PIPE_EQ 51 ("|=")
        CARET 52 ("^")
        CARET_EQ 53 ("^=")
        QUESTION 54 ("?")
        TILDE 55 ("~")
        BSLASH 56 ("\\")
        DOLLAR 57 ("$")
        AT 58 ("@")
        SINGLE_QUOTE 59 ("'") [APOSTROPHE]
        DOUBLE_QUOTE 60 ("\"")
    }
}
