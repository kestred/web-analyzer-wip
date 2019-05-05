// This file is automatically generated from the ESTree spec when `cargo run -p estree_grammar` is run.
// Do not edit manually.
#![allow(dead_code)]
#![allow(unused_imports)]

//! This module contains an auto-generated HTML parser.
use crate::syntax_kind::*;
use web_grammar_utils::{catch, tokenset, Parser, TokenSet};
use web_grammar_utils::parser::Continue;

pub fn html_document(p: &mut Parser) -> Option<Continue> {
    p.eat(WS);
    if p.at(L_ANGLE_BANG) {
        dtd(p)?;
    }
    p.eat(WS);
    while p.at_ts(&tokenset![COMMENT, L_ANGLE, WHITESPACE]) {
        html_elements(p)?;
    }
    Some(Continue)
}

pub fn html_elements(p: &mut Parser) -> Option<Continue> {
    while p.at_ts(&tokenset![COMMENT, WHITESPACE]) {
        html_misc(p)?;
    }
    html_element(p)?;
    while p.at_ts(&tokenset![COMMENT, WHITESPACE]) {
        html_misc(p)?;
    }
    Some(Continue)
}

pub fn html_element(p: &mut Parser) -> Option<Continue> {
    p.expect(L_ANGLE)?;
    p.expect(TAG_NAME)?;
    p.eat(WS);
    while p.at(TAG_NAME) {
        html_attribute(p)?;
        p.eat(WS);
    }
    if p.at(R_ANGLE) && {
        // try --> > SCRIPT
        let mut _checkpoint = p.checkpoint(true);
        catch!({
            p.bump();
            p.expect(SCRIPT)?;
            Some(Continue)
        });
        p.commit(_checkpoint)?.is_ok()
    } {
        // ok
    } else if p.at(R_ANGLE) && {
        // try --> > html_content (< / | </) (WS)? TAG_NAME (WS)? >
        let mut _checkpoint = p.checkpoint(true);
        catch!({
            p.bump();
            html_content(p)?;
            if p.at(L_ANGLE) {
                p.bump();
                p.expect(SLASH)?;
            } else if p.at(L_ANGLE_SLASH) {
                p.bump();
            } else {
                p.expected_ts(&tokenset![L_ANGLE, L_ANGLE_SLASH])?;
            }
            p.eat(WS);
            p.expect(TAG_NAME)?;
            p.eat(WS);
            p.expect(R_ANGLE)?;
            Some(Continue)
        });
        p.commit(_checkpoint)?.is_ok()
    } {
        // ok
    } else if p.at(SLASH_R_ANGLE) {
        p.bump();
    } else if p.at(R_ANGLE) {
        p.bump();
    } else {
        // otherwise, emit an error
        p.expected_ts(&tokenset![R_ANGLE, SLASH_R_ANGLE])?;
    }
    Some(Continue)
}

pub fn html_content(p: &mut Parser) -> Option<Continue> {
    if p.at_ts(&tokenset![TEXT, WHITESPACE]) {
        html_chardata(p)?;
    }
    while p.at_ts(&tokenset![COMMENT, L_ANGLE]) {
        if p.at(L_ANGLE) {
            html_element(p)?;
        } else if p.at(COMMENT) {
            p.bump();
        }
        if p.at_ts(&tokenset![TEXT, WHITESPACE]) {
            html_chardata(p)?;
        }
    }
    Some(Continue)
}

pub fn html_attribute(p: &mut Parser) -> Option<Continue> {
    p.expect(TAG_NAME)?;
    if p.at_ts(&tokenset![EQ, WS]) {
        p.eat(WS);
        p.expect(EQ)?;
        p.eat(WS);
        html_attribute_value(p)?;
    } else {
        p.expected_ts(&tokenset![EQ, WS])?;
    }
    Some(Continue)
}

pub fn html_attribute_value(p: &mut Parser) -> Option<Continue> {
    p.expect_ts(&tokenset![QUOTED, TAG_NAME])
}

pub fn html_chardata(p: &mut Parser) -> Option<Continue> {
    p.expect_ts(&tokenset![TEXT, WHITESPACE])
}

pub fn html_misc(p: &mut Parser) -> Option<Continue> {
    p.expect_ts(&tokenset![COMMENT, WHITESPACE])
}

pub fn dtd(p: &mut Parser) -> Option<Continue> {
    p.expect(L_ANGLE_BANG)?;
    if !(p.at_keyword("DOCTYPE")) {
        p.error("expected input to be at keyword 'DOCTYPE'")?;
    }
    p.expect(IDENT)?;
    p.eat(WS);
    loop {
        p.expect_ts(&tokenset![IDENT, QUOTED])?;
        p.eat(WS);
        if !p.at_ts(&tokenset![IDENT, QUOTED]) { break }
    }
    p.expect(R_ANGLE)?;
    Some(Continue)
}
