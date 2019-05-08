use crate::grammar::*;
use crate::syntax_kind::{self, *};
use grammar_utils::Parser;
use grammar_utils::parser::Continue;
use grammar_utils::{catch, tokenset};

pub fn expression(p: &mut Parser) -> Option<Continue> {
    fn _expression_head(p: &mut Parser) -> Option<Continue> {
        if p.at(FUNCTION_KW) {
            function_expression(p)?;
        } else if p.at(CLASS_KW) {
            class_expression(p)?;
        } else if p.at(NEW_KW) {
            // new_expression
            //     : NEW_KW expression[prec ≥ 17] arguments?
            //     # NEW_EXPRESSION
            //     ;
            {
                let marker = p.start();
                let ok = catch!({
                    p.bump(); // eat NEW_KW
                    _expression_prec(p, 17)?;
                    if p.at(L_PAREN) {
                        arguments(p)?;
                    }
                    Some(Continue)
                });
                p.complete(marker, NEW_EXPRESSION);
                ok?;
            }
        } else if p.at_ts(&tokenset![DELETE_KW, VOID_KW, TYPEOF_KW, PLUS, MINUS, TILDE, BANG]) {
            // unary_expression
            //     : unary_operator expression[prec ≥ 15]
            //     # UNARY_EXPRESSION
            //     ;
            {
                let marker = p.start();
                let ok = catch!({
                    p.bump();
                    _expression_prec(p, 15)?;
                    Some(Continue)
                });
                p.complete(marker, UNARY_EXPRESSION);
                ok?;
            }
        } else if p.at_ts(&tokenset![INCREMENT, DECREMENT]) {
            // update_expression
            //     : update_operator expression[prec ≥ 15]
            //     # UPDATE_EXPRESSION
            //     ;
            {
                let marker = p.start();
                let ok = catch!({
                    p.bump();
                    _expression_prec(p, 15)?;
                    Some(Continue)
                });
                p.complete(marker, UPDATE_EXPRESSION);
                ok?;
            }
        } else if p.at(TEMPLATE_LITERAL) {
            p.bump();
        } else if p.at(IDENTIFIER) {
            if p.nth(1) == FAT_ARROW {
                arrow_function_expression(p)?;
            } else {
                p.bump(); // just `IDENTIFIER`
            }
        } else if p.at(THIS_KW) {
            let marker = p.start();
            p.bump();
            p.complete(marker, THIS_EXPRESSION);
        } else if p.at(SUPER_KW) {
            let marker = p.start();
            p.bump();
            p.complete(marker, SUPER_EXPRESSION);
        } else if p.at(AWAIT_KW) {
            let marker = p.start();
            p.bump();
            expression(p)?;
            p.complete(marker, AWAIT_EXPRESSION);
        } else if p.at(YIELD_KW) {
            let marker = p.start();
            p.bump();
            p.eat(ASTERISK);
            expression(p)?;
            p.complete(marker, YIELD_EXPRESSION);
        } else if p.at_ts(&tokenset![FALSE_KW, NULL_KW, NUMBER_LITERAL, REGEXP_LITERAL, STRING_LITERAL, TRUE_KW]) {
            literal(p)?;
        } else if p.at(L_SQUARE) {
            array_expression(p)?;
        } else if p.at(L_CURLY) {
            object_expression(p)?;
        } else if p.at(L_PAREN) {
            // Try `arrow_function_expression`
            let checkpoint = p.checkpoint(true);
            arrow_function_expression(p);

            // Otherwise, expect `expression_sequence`
            if !p.commit(checkpoint)?.is_ok() {
                p.bump();
                expression_sequence(p)?;
                p.expect(R_PAREN)?;
            }
        } else {
            let kind = p.current();
            let unexpected: &str = syntax_kind::as_str(kind)
                .or_else(|| syntax_kind::as_debug_repr(kind).map(|x| x.name))
                .unwrap_or("<anonymous token>");
            p.error(format!("unexpected \"{}\", expected an expression", unexpected))?;
        }
        Some(Continue)
    }

    fn _expression_prec(p: &mut Parser, prec: u32) -> Option<Continue> {
        // N.B. We start markers here, but its ok not to consume them; if we don't `complete` it
        //      then the extra level of nesting will just be ignored.
        let marker = p.start();
        _expression_head(p)?;

        // member_expression[p ≤ 19]
        //     : expression '[' expression_sequence ']'
        //     # MEMBER_EXPRESSION
        //     ;
        // member_expression[p ≤ 18]
        //     : expression '.' identifier_name
        //     # MEMBER_EXPRESSION
        //
        while prec <= 19 && (p.at(L_SQUARE) || p.at(DOT)) {
            if prec < 19 && p.at(L_SQUARE) {
                p.bump();
                expression_sequence(p)?;
                p.expect(R_SQUARE)?;
                p.complete_and_wrap(&marker, MEMBER_EXPRESSION);
            } else if  prec < 18 && p.at(DOT) {
                p.bump();
                identifier_name(p)?;
                p.complete_and_wrap(&marker, MEMBER_EXPRESSION);
            } else {
                break;
            }
        }
        if prec > 17 {
            return Some(Continue);
        }

        // call_expression[p ≤ 17]
        //     : expression arguments
        //     # CALL_EXPRESSION
        //     ;
        while prec <= 17 && p.at(L_PAREN) {
            arguments(p)?;
            p.complete_and_wrap(&marker, CALL_EXPRESSION);
        }
        if prec > 16 {
            return Some(Continue);
        }

        // update_expression[p ≤ 16]
        //     : expression {!at_line_terminator()}? '++'
        //     # UPDATE_EXPRESSION
        //     ;
        while prec <= 16 && !p.at_line_terminator() && p.at(INCREMENT) {
            p.bump();
            p.complete_and_wrap(&marker, UPDATE_EXPRESSION);
        }
        if prec > 15 {
            return Some(Continue);
        }

        // update_expression[p ≤ 15]
        //     : expression {!at_line_terminator()}? '--'
        //     # UPDATE_EXPRESSION
        //     ;
        while prec <= 15 && !p.at_line_terminator() && p.at(DECREMENT) {
            p.bump();
            p.complete_and_wrap(&marker, UPDATE_EXPRESSION);
        }
        if prec > 14 {
            return Some(Continue);
        }

        // binary_expression[p ≤ 14]
        //     : expression ('*' | '/' | '%') expression
        //     # BINARY_EXPRESSION
        //     ;
        let ts = tokenset![STAR, SLASH, PERCENT];
        while prec <= 14 && p.at_ts(&ts) {
            p.bump();
            _expression_prec(p, 14)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }
        if prec > 13 {
            return Some(Continue);
        }

        // binary_expression[p ≤ 13]
        //     : expression ('+' | '-') expression
        //     # BINARY_EXPRESSION
        //     ;
        let ts = tokenset![PLUS, MINUS];
        while prec <= 13 && p.at_ts(&ts) {
            p.bump();
            _expression_prec(p, 13)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }
        if prec > 12 {
            return Some(Continue);
        }

        // binary_expression[p ≤ 12]
        //     : expression ('<<' | '>>' | '>>>') expression
        //     # BINARY_EXPRESSION
        //     ;
        let ts = tokenset![SHL, SHR, SHU];
        while prec <= 12 && p.at_ts(&ts) {
            p.bump();
            _expression_prec(p, 12)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }
        if prec > 11 {
            return Some(Continue);
        }

        // binary_expression[p ≤ 11]
        //     : expression ('<' | '>' | '<=' | '>=') expression
        //     # BINARY_EXPRESSION
        //     ;
        let ts = tokenset![LT, GT, LT_EQ, GT_EQ];
        while prec <= 11 && p.at_ts(&ts) {
            p.bump();
            _expression_prec(p, 11)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }
        if prec > 10 {
            return Some(Continue);
        }

        // binary_expression[p ≤ 10]
        //     : expression INSTANCEOF_KW expression
        //     # BINARY_EXPRESSION
        //     ;
        while prec <= 10 && p.at(INSTANCEOF_KW) {
            p.bump();
            _expression_prec(p, 10)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }
        if prec > 9 {
            return Some(Continue);
        }

        // binary_expression[p ≤ 9]
        //     : expression IN_KW expression
        //     # BINARY_EXPRESSION
        //     ;
        while prec <= 9 && p.at(IN_KW) {
            p.bump();
            _expression_prec(p, 9)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }
        if prec > 8 {
            return Some(Continue);
        }

        // binary_expression[p ≤ 8]
        //     : expression ('==' | '!=' | '===' | '!==') expression
        //     # BINARY_EXPRESSION
        //     ;
        let ts = tokenset![EQEQ, BANG_EQ, EQEQEQ, BANG_EQEQ];
        while prec <= 8 && p.at_ts(&ts) {
            p.bump();
            _expression_prec(p, 8)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }
        if prec > 7 {
            return Some(Continue);
        }

        // binary_expression[p ≤ 7]
        //     : expression '&' expression
        //     # BINARY_EXPRESSION
        //     ;
        while prec <= 7 && p.at(AMPERSAND) {
            p.bump();
            _expression_prec(p, 7)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }
        if prec > 6 {
            return Some(Continue);
        }

        // binary_expression[p ≤ 6]
        //     : expression '^' expression
        //     # BINARY_EXPRESSION
        //     ;
        while prec <= 6 && p.at(CARET) {
            p.bump();
            _expression_prec(p, 6)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }
        if prec > 5 {
            return Some(Continue);
        }

        // binary_expression[p ≤ 5]
        //     : expression '|' expression
        //     # BINARY_EXPRESSION
        //     ;
        while prec <= 5 && p.at(PIPE) {
            p.bump();
            _expression_prec(p, 5)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }
        if prec > 4 {
            return Some(Continue);
        }

        // logical_expression[p ≤ 4]
        //     : expression '&&' expression
        //     # LOGICAL_EXPRESSION
        //     ;
        while prec <= 4 && p.at(AND) {
            p.bump();
            _expression_prec(p, 4)?;
            p.complete_and_wrap(&marker, LOGICAL_EXPRESSION);
        }
        if prec > 3 {
            return Some(Continue);
        }

        // logical_expression[p ≤ 3]
        //     : expression '||' expression
        //     # LOGICAL_EXPRESSION
        //     ;
        while prec <= 3 && p.at(OR) {
            p.bump();
            _expression_prec(p, 3)?;
            p.complete_and_wrap(&marker, LOGICAL_EXPRESSION);
        }
        if prec > 3 {
            return Some(Continue);
        }

        // conditional_expression[p ≤ 2]
        //     : expression '?' expression ':' expression
        //     # LOGICAL_EXPRESSION
        //     ;
        while prec <= 2 && p.at(QUESTION) {
            p.bump();
            _expression_prec(p, 2)?;
            p.expect(COLON);
            _expression_prec(p, 2)?;
            p.complete_and_wrap(&marker, CONDITIONAL_EXPRESSION);
        }
        if prec > 1 {
            return Some(Continue);
        }

        // assignment_expression[p ≤ 1]
        //     : expression assignment_operator expression
        //     # ASSIGNMENT_EXPRESSION
        //     ;
        let ts = tokenset![AMPERSAND_EQ, ASTERISK_EQ, CARET_EQ, EQ, MINUS_EQ, PERCENT_EQ, PIPE_EQ, PLUS_EQ, SHL_EQ, SHR_EQ, SHU_EQ, SLASH_EQ];
        while prec <= 1 && p.at_ts(&ts) {
            assignment_operator(p)?;
            _expression_prec(p, 1)?;
            p.complete_and_wrap(&marker, ASSIGNMENT_EXPRESSION);
        }
        if prec > 0 {
            return Some(Continue);
        }

        // tagged_template_expression[p ≤ 0]
        //     : expression TEMPLATE_LITERAL
        //     # TAGGED_TEMPLATE_EXPRESSION
        //     ;
        if prec <= 0 && p.at(TEMPLATE_LITERAL) {
            assignment_operator(p)?;
            p.complete_and_wrap(&marker, TAGGED_TEMPLATE_EXPRESSION);
        }

        Some(Continue)
    }

    _expression_prec(p, 0)
}