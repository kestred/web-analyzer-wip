use crate::syntax_kind::*;
// use web_grammar_utils::{Parser, SyntaxKind};
use web_grammar_utils::grammar::*;

pub fn program() -> impl GrammarNode {
    ( token(EOF)
    | directive()
    | statement()
    ).is(PROGRAM)
}

pub fn directive() -> impl PredictiveGrammar {
    expression_statement()
}

pub fn statement() -> impl PredictiveGrammar {
    ( never()

    // FIXME: Implement proper arbitrary lookahead to avoid infinite recursion
    // | expression_statement()

    // TODO: Implement
    // BlockStatement = BLOCK_STATEMENT,
    // EmptyStatement = EMPTY_STATEMENT,
    // DebuggerStatement = DEBUGGER_STATEMENT,
    // WithStatement = WITH_STATEMENT,
    // ReturnStatement = RETURN_STATEMENT,
    // LabeledStatement = LABELED_STATEMENT,
    // BreakStatement = BREAK_STATEMENT,
    // ContinueStatement = CONTINUE_STATEMENT,
    // IfStatement = IF_STATEMENT,
    // SwitchStatement = SWITCH_STATEMENT,
    // ThrowStatement = THROW_STATEMENT,
    // TryStatement = TRY_STATEMENT,
    // WhileStatement = WHILE_STATEMENT,
    // DoWhileStatement = DO_WHILE_STATEMENT,
    // ForStatement = FOR_STATEMENT,
    // ForInStatement = FOR_IN_STATEMENT,
    | declaration()
    ).into_predictive_boxed()
}

pub fn expression_statement() -> impl PredictiveGrammar {
    ( expression()
    , token(SEMI)
    ).commit(EXPRESSION_STATEMENT)
}

pub fn declaration() -> impl PredictiveGrammar {
    ( never()
    // TODO: Implement
    // FunctionDeclaration = FUNCTION_DECLARATION
    | variable_declaration()
    // ClassDeclaration = CLASS_DECLARATION,
    )
}

pub fn variable_declaration() -> impl PredictiveGrammar {
    ( token(VAR_KW)
    , many1(variable_declarator()).sep_by(token(COMMA))
    , token(SEMI)
    ).commit(VARIABLE_DECLARATION)
}

pub fn variable_declarator() -> impl PredictiveGrammar {
    pattern()
        .then(optional(variable_initializer()))
        .commit(VARIABLE_DECLARATOR)
}

pub fn variable_initializer() -> impl PredictiveGrammar {
    token(EQ).then(expression())
}

pub fn pattern() -> impl PredictiveGrammar {
    token(IDENT) // TODO: Implement destructuring
}

pub fn expression() -> impl PredictiveGrammar {
    ( token(IDENT)
    | literal()
    // TODO: Implement
    // ThisExpression = THIS_EXPRESSION,
    // ArrayExpression = ARRAY_EXPRESSION,
    // ObjectExpression = OBJECT_EXPRESSION,
    // FunctionExpression = FUNCTION_EXPRESSION,
    // UnaryExpression = UNARY_EXPRESSION,
    // UpdateExpression = UPDATE_EXPRESSION,
    // BinaryExpression = BINARY_EXPRESSION,
    // AssignmentExpression = ASSIGNMENT_EXPRESSION,
    // LogicalExpression = LOGICAL_EXPRESSION,
    // MemberExpression = MEMBER_EXPRESSION,
    // ConditionalExpression = CONDITIONAL_EXPRESSION,
    | call_expression()
    // NewExpression = NEW_EXPRESSION,
    // SequenceExpression = SEQUENCE_EXPRESSION,
    // ArrowFunctionExpression = ARROW_FUNCTION_EXPRESSION,
    // YieldExpression = YIELD_EXPRESSION,
    // TemplateLiteral = TEMPLATE_LITERAL,
    // TaggedTemplateExpression = TAGGED_TEMPLATE_EXPRESSION,
    // ClassExpression = CLASS_EXPRESSION,
    // MetaProperty = META_PROPERTY,
    // AwaitExpression = AWAIT_EXPRESSION,
    ).into_predictive_boxed()
}

pub fn call_expression() -> impl PredictiveGrammar {
  ( expression()
  , token(L_PAREN)
  // TODO: Parse arguments
  , token(R_PAREN)
  )
//   CALL_EXPRESSION
//     type: "CallExpression";
//     callee: Expression;
//     arguments: [ Expression ];
}

pub fn literal() -> impl PredictiveGrammar {
    ( token(STRING_TOKEN)
    | token(NUMBER_TOKEN)
    | token(REGEXP_TOKEN)
    | token(FALSE_KW)
    | token(TRUE_KW)
    | token(NULL_KW)
    ).commit(LITERAL)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::JavascriptLexer;
    use web_grammar_utils::{Lexer, Parser};

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
        for &text in examples {
            let tokens = JavascriptLexer::new().tokenize(text);
            let mut parser = Parser::new((text, &tokens).into(), false);
            assert!(variable_declaration().parse(&mut parser).is_ok());
        }
    }

  #[test]
    fn test_parse_sample1() {
        let text = crate::samples::SAMPLE_1;
        let tokens = JavascriptLexer::new().tokenize(text);
        let parser = Parser::new((text, &tokens).into(), false);
        let (_, remaining) = parser.parse(&program());
        assert!(remaining.tokens.is_empty());
    }

  #[test]
    fn test_parse_sample2() {
        let text = crate::samples::SAMPLE_2;
        let tokens = JavascriptLexer::new().tokenize(text);
        let parser = Parser::new((text, &tokens).into(), false);
        let (_, remaining) = parser.parse(&program());
        assert!(remaining.tokens.is_empty());
    }
}
