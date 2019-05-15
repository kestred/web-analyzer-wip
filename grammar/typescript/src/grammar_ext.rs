use crate::grammar::*;
use crate::syntax_kind::{self, *};
use code_grammar::Parser;
use code_grammar::parser::Continue;
use code_grammar::{catch, tokenset};

pub fn ts_type_annotation(p: &mut Parser) -> Option<Continue> {
    fn _ts_type_annotation_head(p: &mut Parser) -> Option<Continue> {
        if p.at(PIPE) {
            let marker = p.start();
            let ok = catch!({
                p.bump(); // eat PIPE
                ts_type_annotation(p)?;
                Some(Continue)
            });
            p.complete(marker, UNION_TYPE_EXPR);
            ok?;
        } else if p.at(TYPEOF_KW) {
            _ts_type_annotation_typeof(p)?;
        } else if p.at(L_CURLY) {
            _ts_type_annotation_interface(p)?;
        } else if p.at(L_SQUARE) {
            ts_tuple_type(p)?;
        } else if p.at(L_PAREN) {
            // Try `ts_function_type`
            let checkpoint = p.checkpoint(true);
            ts_function_type(p); /* .ok(); */

            // Otherwise, expect `ts_type_annotation`
            if !p.commit(checkpoint)?.is_ok() {
                p.bump();
                ts_type_annotation(p)?;
                p.expect(R_PAREN)?;
            }
        } else if p.at_ts(&tokenset![IDENTIFIER, BOOLEAN_KW]) {
            identifier_or_primitive(p)?;
        } else if p.at_ts(&AT_LITERAL) {
            literal(p)?;
        } else {
            let kind = p.current();
            let unexpected: &str = syntax_kind::as_str(kind)
                .or_else(|| syntax_kind::as_debug_repr(kind).map(|x| x.name))
                .unwrap_or("<anonymous token>");
            p.error(format!("unexpected \"{}\", expected a type", unexpected))?;
        }
        Some(Continue)
    }

    fn _ts_type_annotation_prec(p: &mut Parser, prec: u32) -> Option<Continue> {
        // N.B. We start markers here, but its ok not to consume them; if we don't `complete` it
        //      then the extra level of nesting will just be ignored.
        let marker = p.start();
        _ts_type_annotation_head(p)?;

        // Handle postfix type operators
        {
            // member_ts_type_annotation[p ≤ 5]
            //     : ts_type_annotation '&' identifier_or_keyword
            //     # MEMBER_TYPE_EXPR
            //     ;
            // generic_ts_type_annotation[p ≤ 4]
            //     : ts_type_annotation <' type_generic_arg (',' type_generic_arg)* '>'
            //     # GENERIC_TYPE_EXPR
            //     ;
            // array_ts_type_annotation[p ≤ 3]
            //     : ts_type_annotation '[' ']'
            //     # ARRAY_TYPE_EXPR
            //     ;
            while p.at_ts(&tokenset![DOT, L_ANGLE, L_SQUARE]) {
                if prec < 5 && p.at(DOT) {
                    p.bump();
                    identifier_or_keyword(p)?;
                    p.complete_and_wrap(&marker, MEMBER_TYPE_EXPR);
                } else if prec < 4 && p.at(L_ANGLE) {
                    ts_type_arguments(p)?;
                    p.complete_and_wrap(&marker, GENERIC_TYPE_EXPR);
                } else if prec < 3 && p.at(L_SQUARE) {
                    p.bump();
                    p.expect(R_SQUARE)?;
                    p.complete_and_wrap(&marker, ARRAY_TYPE_EXPR);
                } else {
                    break;
                }
            }
        }

        // intersection_ts_type_annotation[p ≤ 2]
        //     : ts_type_annotation '&' ts_type_annotation
        //     # INTERSECTION_TYPE_EXPR
        //     ;
        if prec > 2 { return Some(Continue); }
        while p.at(AMPERSAND) {
            p.bump();
            _ts_type_annotation_prec(p, 2)?;
            p.complete_and_wrap(&marker, INTERSECTION_TYPE_EXPR);
        }

        // union_ts_type_annotation[p ≤ 1]
        //     : ts_type_annotation '|' ts_type_annotation
        //     # UNION_TYPE_EXPR
        //     ;
        if prec > 1 { return Some(Continue); }
        while p.at(PIPE) {
            p.bump();
            _ts_type_annotation_prec(p, 1)?;
            p.complete_and_wrap(&marker, UNION_TYPE_EXPR);
        }

        // conditional_ts_type_annotation[p ≤ 0]
        //     : ts_type_annotation '?' ts_type_annotation ':' ts_type_annotation
        //     # CONDITIONAL_TYPE_EXPR
        //     ;
        if prec > 0 { return Some(Continue); }
        while p.at(QUESTION) {
            p.bump();
            _ts_type_annotation_prec(p, 0)?;
            p.expect(SEMICOLON)?;
            _ts_type_annotation_prec(p, 0)?;
            p.complete_and_wrap(&marker, CONDITIONAL_TYPE_EXPR);
        }

        Some(Continue)
    }

    _ts_type_annotation_prec(p, 0)
}

// BLECH... really didn't want to have to copy this; but need to support type assertions
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
            let marker = p.start();
            p.bump();
            p.complete(marker, TEMPLATE_EXPRESSION);
        } else if p.at(IDENTIFIER) {
            if p.nth(1) == FAT_ARROW {
                arrow_function_expression(p)?;
            } else if !p.at_keyword("async") {
                identifier(p)?;
            } else {
                let checkpoint = p.checkpoint(true);
                arrow_function_expression(p); /* .ok(); */

                // Otherwise, expect a single identifier
                if !p.commit(checkpoint)?.is_ok() {
                    identifier(p)?;
                }
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
        } else if p.at_ts(&AT_LITERAL) {
            literal(p)?;
        } else if p.at(L_SQUARE) {
            array_expression(p)?;
        } else if p.at(L_CURLY) {
            object_expression(p)?;
        } else if p.at(L_ANGLE) {
            // TODO: Handle prefix `<...>` type assertions
            arrow_function_expression(p);
        } else if p.at(L_PAREN) {
            // N.B. Do some custom lookahead logic here to avoid
            // TODO: Implement auto-genned 2-4 token lookahead for ambiguous cases
            let peek = p.nth(1);
            if peek == DOTDOTDOT || peek == R_PAREN {
                arrow_function_expression(p)?;
            } else if !tokenset![IDENTIFIER, L_SQUARE, L_CURLY].contains(&peek) {
                p.bump();
                expression_list(p)?;
                p.expect(R_PAREN)?;
            } else {
                // Try `arrow_function_expression`
                //
                // N.B. Allow lots of rollback to disambiguate
                //      between a pattern and an expression.
                let checkpoint = p.checkpoint_upto(32);
                arrow_function_expression(p);

                // Otherwise, expect `expression_list`
                if !p.commit(checkpoint)?.is_ok() {
                    p.bump();
                    expression_list(p)?;
                    p.expect(R_PAREN)?;
                }
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

        // Handle postfix expressions
        {
            // member_expression[p ≤ 19]
            //     : expression '[' expression_list ']'
            //     # MEMBER_EXPRESSION
            //     ;
            // member_expression[p ≤ 18]
            //     : expression '.' identifier_or_keyword
            //     # MEMBER_EXPRESSION
            //     ;
            // call_expression[p ≤ 17]
            //     : expression arguments
            //     # CALL_EXPRESSION
            //     ;
            // update_expression[p ≤ 16]
            //     : expression {!at_beginning_of_line()}? '++'
            //     # UPDATE_EXPRESSION
            //     ;
            // update_expression[p ≤ 15]
            //     : expression {!at_beginning_of_line()}? '--'
            //     # UPDATE_EXPRESSION
            //     ;
            // not_null_expression[p ≤ 15]
            //     : expression {!at_beginning_of_line()}? '!'
            //     # TS_NON_NULL_EXPRESSION
            //     ;
            while prec <= 19 && p.at_ts(&tokenset![L_SQUARE, DOT, L_PAREN, INCREMENT, DECREMENT, BANG]) {
                if prec < 19 && p.at(L_SQUARE) {
                    p.bump();
                    expression_list(p)?;
                    p.expect(R_SQUARE)?;
                    p.complete_and_wrap(&marker, MEMBER_EXPRESSION);
                } else if  prec < 18 && p.at(DOT) {
                    p.bump();
                    identifier_or_keyword(p)?;
                    p.complete_and_wrap(&marker, MEMBER_EXPRESSION);
                } else if prec <= 17 && p.at(L_PAREN) {
                    arguments(p)?;
                    p.complete_and_wrap(&marker, CALL_EXPRESSION);
                } else if prec <= 16 && p.at(INCREMENT) {
                    p.bump();
                    p.complete_and_wrap(&marker, UPDATE_EXPRESSION);
                } else if prec <= 15 && p.at(DECREMENT) {
                    p.bump();
                    p.complete_and_wrap(&marker, UPDATE_EXPRESSION);
                } else if prec <= 15 && p.at(BANG) {
                    p.bump();
                    p.complete_and_wrap(&marker, TS_NON_NULL_EXPRESSION);
                } else {
                    break;
                }
            }
        }

        // binary_expression[p ≤ 14]
        //     : expression ('*' | '/' | '%') expression
        //     # BINARY_EXPRESSION
        //     ;
        if prec > 14 { return Some(Continue); }
        while prec <= 14 && p.at_ts(&tokenset![STAR, SLASH, PERCENT]) {
            p.bump();
            _expression_prec(p, 14)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }

        // binary_expression[p ≤ 13]
        //     : expression ('+' | '-') expression
        //     # BINARY_EXPRESSION
        //     ;
        if prec > 13 { return Some(Continue); }
        while prec <= 13 && p.at_ts(&tokenset![PLUS, MINUS]) {
            p.bump();
            _expression_prec(p, 13)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }

        // binary_expression[p ≤ 12]
        //     : expression ('<<' | '>>' | '>>>') expression
        //     # BINARY_EXPRESSION
        //     ;
        if prec > 12 { return Some(Continue); }
        while prec <= 12 && p.at_ts(&tokenset![L_ANGLE, R_ANGLE]) {
            if p.current3() == Some((R_ANGLE, R_ANGLE, R_ANGLE)) {
                p.bump_compound(SHU, 3);
            } else if p.current2() == Some((R_ANGLE, R_ANGLE)) {
                p.bump_compound(SHR, 2);
            } else if p.current2() == Some((L_ANGLE, L_ANGLE)) {
                p.bump_compound(SHL, 2);
            } else {
                break;
            }
            _expression_prec(p, 12)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }

        // binary_expression[p ≤ 11]
        //     : expression ('<' | '>' | '<=' | '>=') expression
        //     # BINARY_EXPRESSION
        //     ;
        if prec > 11 { return Some(Continue); }
        while prec <= 11 && p.at_ts(&tokenset![LT, GT]) {
            if p.current2() == Some((L_ANGLE, EQ)) {
                p.bump_compound(LT_EQ, 2);
            } else if p.current2() == Some((R_ANGLE, EQ)) {
                p.bump_compound(GT_EQ, 2);
            } else {
                p.bump();
            }
            _expression_prec(p, 11)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }

        // binary_expression[p ≤ 10]
        //     : expression INSTANCEOF_KW expression
        //     # BINARY_EXPRESSION
        //     ;
        if prec > 10 { return Some(Continue); }
        while prec <= 10 && p.at(INSTANCEOF_KW) {
            p.bump();
            _expression_prec(p, 10)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }

        // binary_expression[p ≤ 9]
        //     : expression IN_KW expression
        //     # BINARY_EXPRESSION
        //     ;
        if prec > 9 { return Some(Continue); }
        while prec <= 9 && p.at(IN_KW) {
            p.bump();
            _expression_prec(p, 9)?;
            p.complete_and_wrap(&marker, BINARY_EXPRESSION);
        }

        // FIXME: Need to introduce a new precedence level here
        {
            // as_expression[p ≤ 8]
            //     : expression 'as' ts_type_annotation
            //     # TS_AS_EXPRESSION
            //     ;
            if prec > 8 { return Some(Continue); }
            while prec <= 8 && p.at_keyword("as") && p.at(IDENTIFIER) {
                p.expect_keyword(AS_KW, "as")?;
                ts_type_annotation(p)?;
                p.complete_and_wrap(&marker, TS_AS_EXPRESSION);
            }
            }

        // binary_expression[p ≤ 8]
        //     : expression ('==' | '!=' | '===' | '!==') expression
        //     # BINARY_EXPRESSION
        //     ;
        if prec > 8 { return Some(Continue); }
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
            p.expect(COLON)?;
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
            p.bump();
            p.complete_and_wrap(&marker, TAGGED_TEMPLATE_EXPRESSION);
        }

        Some(Continue)
    }

    _expression_prec(p, 0)
}
