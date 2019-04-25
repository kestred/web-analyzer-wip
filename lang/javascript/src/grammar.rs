use crate::syntax_kind::*;
// use web_grammars_utils::{Parser, SyntaxKind};
use web_grammars_utils::grammar::*;

pub fn variable_declaration() -> impl PredictiveGrammar {
    token(VAR_KW)
        .then(many1(variable_declarator()).sep_by(token(COMMA)))
        .then(token(SEMI))
        .commit(VARIABLE_DECLARATION)
}

pub fn variable_declarator() -> impl PredictiveGrammar {
    pattern()
        .then(optional(variable_initializer()))
        .commit(VARIABLE_DECLARATOR)
}

pub fn variable_initializer() -> impl PredictiveGrammar {
    token(EQ).then(expression())
}

pub fn expression() -> impl PredictiveGrammar {
    literal() // TODO: Implement full expression tree
}

pub fn literal() -> impl PredictiveGrammar {
    ( token(STRING_TOKEN)
    | token(NUMBER_TOKEN)
    | token(REGEXP_TOKEN)
    | token(FALSE_KW)
    | token(TRUE_KW)
    | token(NULL_KW)
    ).commit(LITERAL) | token(TEMPLATE_TOKEN).commit(TEMPLATE_LITERAL)
}

pub fn pattern() -> impl PredictiveGrammar {
    token(IDENT) // TODO: Implement destructuring
}

#[cfg(test)]
mod test {
    use crate::lexer::JavascriptLexer;
    use super::*;
    use web_grammars_utils::{Lexer, Parser};

  #[test]
    fn test_variable_declaration() {
        let examples = &[
            "var foo;",
            "var foo = 'hello';",
            "var foo, bar;",
            "var foo, bar = 'world';",
            "var foo = 'hello', bar;",
            "var foo = 'hello', bar = 'world';",
        ];
        for text in examples {
            let tokens = JavascriptLexer::new().tokenize(text);
            let mut parser = Parser::new(text, &tokens, false);
            assert!(variable_declaration().parse(&mut parser).is_ok());
        }
    }

  #[test]
    fn test_parse_sample1() {
        let text = crate::samples::SAMPLE_1;
        let tokens = JavascriptLexer::new().tokenize(text);
        let mut parser = Parser::new(text, &tokens, false);
        assert!(variable_declaration().parse(&mut parser).is_ok());
    }
}
