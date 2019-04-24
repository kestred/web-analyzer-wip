mod language_kind;
mod meta;

use crate::syntax_kinds;

pub use self::language_kind::LanguageKind;
pub use self::meta::SyntaxKindMeta;

const DEFAULT: LanguageKind = LanguageKind(0);

syntax_kinds!{
    language DEFAULT;

    default {
        TOMBSTONE 0
        ERROR 1
        EOF 2
        WHITESPACE 3
        COMMENT 4
        IDENT 5
        SHEBANG 6 ("#!/")
        L_PAREN 7 ("(")
        R_PAREN 8 (")")
        L_CURLY 9 ("{")
        R_CURLY 10 ("}")
        L_ANGLE 11 ("<")
        R_ANGLE 12 (">")
        L_BRACK 13 ("[")
        R_BRACK 14 ("]")
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
        BANGEQ 27 ("!=")
        GTEQ 28 (">=")
        LTEQ 29 ("<=")
        SEMI 30 (";")
        COLON 31 (":")
        COLONCOLON 32 ("::")
        COMMA 33 (",")
        DOT 34 (".")
        DOTDOT 35 ("..")
        DOTDOTDOT 36 ("...")
        STAR 37 ("*")
        STAR_EQ 38 ("*=")
        SLASH 39 ("/")
        SLASH_EQ 40 ("/=")
        PERCENT 41 ("%")
        PERCENT_EQ 42 ("%=")
        PLUS 43 ("+")
        PLUS_EQ 44 ("+=")
        MINUS 45 ("-")
        MINUS_EQ 46 ("-=")
        AMPERSAND 47 ("&")
        AMPERSAND_EQ 48 ("&=")
        PIPE 49 ("|")
        PIPE_EQ 50 ("|=")
        BANG 51 ("!")
        QUESTION 52 ("?")
        CARET 53 ("^")
        CARET_EQ 54 ("^=")
        TILDA 55 ("~")
        DOLLAR 56 ("$")
        ALPHASAND 57 ("@")
    }
}
