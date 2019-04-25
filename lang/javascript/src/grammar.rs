use crate::syntax_kind::*;
use web_grammars_utils::{Parser, SyntaxKind};
use web_grammars_utils::grammar::*;

// fn variable_declaration(p: &mut Parser) -> SyntaxKind {
//   (
//     token(VAR_KW),
//     many1(variable_declarator).sep_by(COMMA)
//   )
//   .commit(p, VARIABLE_DECLARATION)
// }

// fn variable_declarator(p: &mut Parser) -> SyntaxKind {
//   pattern
//     .then(optional(variable_initializer))
//     .commit(p, VARIABLE_DECLARATOR)
// }

// fn variable_initializer(p: &mut Parser) -> SyntaxKind {
//   unimplemented!()
//   // .is(EXPRESSION)
//   // .eval(p)
// }

// fn pattern(p: &mut Parser) -> SyntaxKind {
//   unimplemented!()
// }

// #[cfg(test)]
// mod test {
//   use crate::lexer::JavascriptLexer;
//   use super::*;
//   use web_grammars_utils::{Lexer, Parser};

//   fn test_variable_declaration() {
//     let text = crate::samples::SAMPLE_1;
//     let tokens = JavascriptLexer::new().tokenize(text);
//     let mut parser = Parser::new(text, &tokens);
//     parser.eval(&variable_declaration);
//     assert!(parser.has_errors(), false);
//     assert!(parser.is_eof(), false);
//   }

//   #[test]
//   fn test_parse_sample1() {
//     let text = crate::samples::SAMPLE_1;
//     let tokens = JavascriptLexer::new().tokenize(text);
//     let mut parser = Parser::new(text, &tokens);
//     parser.eval(&variable_declaration);
//     assert!(parser.has_errors(), false);
//     assert!(parser.is_eof(), false);
//   }
// }
