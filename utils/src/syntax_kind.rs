mod language_kind;
mod meta;

use rowan::SyntaxKind;

pub use self::language_kind::LanguageKind;
pub use self::meta::SyntaxKindMeta;

const DEFAULT: LanguageKind = LanguageKind(0);

pub const TOMBSTONE: SyntaxKind = DEFAULT.syntax_kind(0);
pub const ERROR: SyntaxKind = DEFAULT.syntax_kind(1);
pub const EOF: SyntaxKind = DEFAULT.syntax_kind(2);
pub const WHITESPACE: SyntaxKind = DEFAULT.syntax_kind(3);
pub const COMMENT: SyntaxKind = DEFAULT.syntax_kind(4);
pub const IDENT: SyntaxKind = DEFAULT.syntax_kind(5);
pub const SHEBANG: SyntaxKind = DEFAULT.syntax_kind(6); // '#!/'
pub const L_PAREN: SyntaxKind = DEFAULT.syntax_kind(7); // '('
pub const R_PAREN: SyntaxKind = DEFAULT.syntax_kind(8); // ')'
pub const L_CURLY: SyntaxKind = DEFAULT.syntax_kind(9); // '{'
pub const R_CURLY: SyntaxKind = DEFAULT.syntax_kind(10); // '}'
pub const L_ANGLE: SyntaxKind = DEFAULT.syntax_kind(11); // '<'
pub const R_ANGLE: SyntaxKind = DEFAULT.syntax_kind(12); // '>'
pub const L_BRACK: SyntaxKind = DEFAULT.syntax_kind(13); // '['
pub const R_BRACK: SyntaxKind = DEFAULT.syntax_kind(14); // ']'
pub const SHL: SyntaxKind = DEFAULT.syntax_kind(15); // '<<'
pub const SHL_EQ: SyntaxKind = DEFAULT.syntax_kind(16); // '<<='
pub const SHR: SyntaxKind = DEFAULT.syntax_kind(17); // '>>'
pub const SHR_EQ: SyntaxKind = DEFAULT.syntax_kind(18); // '>>='
pub const AND: SyntaxKind = DEFAULT.syntax_kind(19); // '&&'
pub const OR: SyntaxKind = DEFAULT.syntax_kind(20); // '||'
pub const INCREMENT: SyntaxKind = DEFAULT.syntax_kind(21); // '++'
pub const DECREMENT: SyntaxKind = DEFAULT.syntax_kind(22); // '--'
pub const THIN_ARROW: SyntaxKind = DEFAULT.syntax_kind(23); // '->'
pub const FAT_ARROW: SyntaxKind = DEFAULT.syntax_kind(24); // '=>'
pub const EQ: SyntaxKind = DEFAULT.syntax_kind(25); // '='
pub const EQEQ: SyntaxKind = DEFAULT.syntax_kind(26); // '=='
pub const BANGEQ: SyntaxKind = DEFAULT.syntax_kind(27); // '!='
pub const GTEQ: SyntaxKind = DEFAULT.syntax_kind(28); // '>='
pub const LTEQ: SyntaxKind = DEFAULT.syntax_kind(29); // '<='
pub const SEMI: SyntaxKind = DEFAULT.syntax_kind(30); // ';'
pub const COLON: SyntaxKind = DEFAULT.syntax_kind(31); // ':'
pub const COLONCOLON: SyntaxKind = DEFAULT.syntax_kind(32); // '::'
pub const COMMA: SyntaxKind = DEFAULT.syntax_kind(33); // ','
pub const DOT: SyntaxKind = DEFAULT.syntax_kind(34); // '.'
pub const DOTDOT: SyntaxKind = DEFAULT.syntax_kind(35); // '..'
pub const DOTDOTDOT: SyntaxKind = DEFAULT.syntax_kind(36); // '...'
pub const STAR: SyntaxKind = DEFAULT.syntax_kind(37); // '*'
pub const STAR_EQ: SyntaxKind = DEFAULT.syntax_kind(38); // '*='
pub const SLASH: SyntaxKind = DEFAULT.syntax_kind(39); // '/'
pub const SLASH_EQ: SyntaxKind = DEFAULT.syntax_kind(40); // '/='
pub const PERCENT: SyntaxKind = DEFAULT.syntax_kind(41); // '%'
pub const PERCENT_EQ: SyntaxKind = DEFAULT.syntax_kind(42); // '%='
pub const PLUS: SyntaxKind = DEFAULT.syntax_kind(43); // '+'
pub const PLUS_EQ: SyntaxKind = DEFAULT.syntax_kind(44); // '+='
pub const MINUS: SyntaxKind = DEFAULT.syntax_kind(45); // '-'
pub const MINUS_EQ: SyntaxKind = DEFAULT.syntax_kind(46); // '-='
pub const AMPERSAND: SyntaxKind = DEFAULT.syntax_kind(47); // '&'
pub const AMPERSAND_EQ: SyntaxKind = DEFAULT.syntax_kind(48); // '&='
pub const PIPE: SyntaxKind = DEFAULT.syntax_kind(49); // '|'
pub const PIPE_EQ: SyntaxKind = DEFAULT.syntax_kind(50); // '|='
pub const BANG: SyntaxKind = DEFAULT.syntax_kind(51); // '!'
pub const QUESTION: SyntaxKind = DEFAULT.syntax_kind(52); // '?'
pub const CARET: SyntaxKind = DEFAULT.syntax_kind(53); // '^'
pub const CARET_EQ: SyntaxKind = DEFAULT.syntax_kind(54); // '^='
pub const TILDA: SyntaxKind = DEFAULT.syntax_kind(55); // '~'
pub const DOLLAR: SyntaxKind = DEFAULT.syntax_kind(56); // '$'
pub const ALPHASAND: SyntaxKind = DEFAULT.syntax_kind(57); // '@'
