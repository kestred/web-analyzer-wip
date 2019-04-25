use crate::syntax_kind::*;
// use web_grammars_utils::{Parser, SyntaxKind};
use web_grammars_utils::grammar::*;

pub fn variable_declaration() -> impl Predictive {
  token(VAR_KW)
    .then(many1(variable_declarator()).sep_by(token(COMMA)))
    .then(token(SEMI))
    .commit(VARIABLE_DECLARATION)
}

pub fn variable_declarator() -> impl Predictive {
  pattern()
    .then(optional(variable_initializer()))
    .commit(VARIABLE_DECLARATOR)
}

pub fn variable_initializer() -> impl Predictive {
  token(TOMBSTONE) // TODO: Implement
}

pub fn pattern() -> impl Predictive {
  token(IDENT)
}

#[cfg(test)]
mod test {
  use crate::lexer::JavascriptLexer;
  use super::*;
  use web_grammars_utils::{Lexer, Parser};

  #[test]
  fn test_variable_declaration() {
    let text = r#"var foo;"#;
    let tokens = JavascriptLexer::new().tokenize(text);
    let mut parser = Parser::new(text, &tokens, false);
    assert!(variable_declaration().parse(&mut parser).is_ok());
  }

  #[test]
  fn test_parse_sample1() {
    let text = crate::samples::SAMPLE_1;
    let tokens = JavascriptLexer::new().tokenize(text);
    let mut parser = Parser::new(text, &tokens, false);
    assert!(variable_declaration().parse(&mut parser).is_ok());
  }
}
