use crate::grammar::*;
use crate::syntax_kind::{self, *};
use code_grammar::Parser;
use code_grammar::parser::Continue;
use code_grammar::tokenset;

pub use javascript_grammar::grammar::expression;

pub fn type_expr(p: &mut Parser) -> Option<Continue> {
    fn _type_expr_head(p: &mut Parser) -> Option<Continue> {
        if p.at(L_CURLY) {
            _type_expr_interface(p)?;
        } else if p.at(L_PAREN) {
            _type_expr_function(p)?;
        } else if p.at(L_SQUARE) {
            _type_expr_tuple(p)?;
        } else if p.at(TYPEOF_KW) {
            _type_expr_typeof(p)?;
        } else if p.at(IDENTIFIER) {
            identifier(p)?;
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

    fn _type_expr_prec(p: &mut Parser, prec: u32) -> Option<Continue> {
        // N.B. We start markers here, but its ok not to consume them; if we don't `complete` it
        //      then the extra level of nesting will just be ignored.
        let marker = p.start();
        _type_expr_head(p)?;

        // Handle postfix type operators
        {
            // generic_type_expr[p ≤ 4]
            //     : type_expr <' type_generic_arg (',' type_generic_arg)* '>'
            //     # GENERIC_TYPE
            //     ;
            // array_type_expr[p ≤ 3]
            //     : type_expr '[' ']'
            //     # ARRAY_TYPE
            //     ;
            while p.at_ts(&tokenset![L_ANGLE, L_SQUARE]) {
                if prec < 4 && p.at(L_ANGLE) {
                    type_arguments(p)?;
                    p.complete_and_wrap(&marker, GENERIC_TYPE_EXPR);
                } else if prec < 3 && p.at(L_SQUARE) {
                    p.bump();
                    p.expect(R_SQUARE);
                    p.complete_and_wrap(&marker, ARRAY_TYPE_EXPR);
                } else {
                    break;
                }
            }
        }

        // intersection_type_expr[p ≤ 2]
        //     : type_expr '&' type_expr
        //     # INTERSECTION_TYPE
        //     ;
        if prec > 2 { return Some(Continue); }
        while p.at(AMPERSAND) {
            p.bump();
            _type_expr_prec(p, 2)?;
            p.complete_and_wrap(&marker, INTERSECTION_TYPE_EXPR);
        }

        // union_type_expr[p ≤ 1]
        //     : type_expr '|' type_expr
        //     # UNION_TYPE
        //     ;
        if prec > 1 { return Some(Continue); }
        while p.at(PIPE) {
            p.bump();
            _type_expr_prec(p, 1)?;
            p.complete_and_wrap(&marker, UNION_TYPE_EXPR);
        }

        // conditional_type_expr[p ≤ 0]
        //     : type_expr '?' type_expr ':' type_expr
        //     # CONDITIONAL_TYPE
        //     ;
        if prec > 0 { return Some(Continue); }
        while p.at(QUESTION) {
            p.bump();
            _type_expr_prec(p, 0)?;
            p.expect(SEMICOLON);
            _type_expr_prec(p, 0)?;
            p.complete_and_wrap(&marker, CONDITIONAL_TYPE_EXPR);
        }

        Some(Continue)
    }

    _type_expr_prec(p, 0)
}
