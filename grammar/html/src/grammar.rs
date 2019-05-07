// This file is automatically generated from the ESTree spec when `cargo run -p estree_grammar` is run.
// Do not edit manually.
#![allow(dead_code)]
#![allow(unused_imports)]

//! This module contains an auto-generated HTML parser.
use crate::syntax_kind::*;
use grammar_utils::{catch, tokenset, Parser, TokenSet};
use grammar_utils::parser::Continue;

pub fn html_document(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = catch!({
        p.eat(WS);
        if p.at(L_ANGLE_BANG) {
            doctype(p)?;
        }
        p.eat(WS);
        while p.at_ts(&tokenset![COMMENT, L_ANGLE, WHITESPACE]) {
            html_elements(p)?;
        }
        Some(Continue)
    });
    p.complete(_marker, DOCUMENT);
    _ok
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
    let mut _checkpoint = p.checkpoint(false);
    let _marker = p.start();
    p.expect(L_ANGLE)?;
    p.expect(TAG_NAME)?;
    p.eat(WS);
    while p.at(TAG_NAME) {
        html_attribute(p)?;
        p.eat(WS);
    }
    if p.at(R_ANGLE) && {
        // try --> > html_content (< / | </) (WS)? TAG_NAME (WS)? > #ELEMENT
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
            p.complete(_checkpoint.branch(&_marker), ELEMENT);
            Some(Continue)
        });
        p.commit(_checkpoint)?.is_ok()
    } {
        // ok
    } else if p.at(SLASH_R_ANGLE) {
        p.bump();
        p.complete(_checkpoint.branch(&_marker), ELEMENT);
    } else if p.at(R_ANGLE) {
        p.bump();
        p.complete(_checkpoint.branch(&_marker), ELEMENT);
    } else {
        // otherwise, emit an error
        p.expected_ts(&tokenset![R_ANGLE, SLASH_R_ANGLE])?;
    }
    Some(Continue)
}

pub fn html_content(p: &mut Parser) -> Option<Continue> {
    if p.at_ts(&tokenset![COMMENT, L_ANGLE, TEXT, WHITESPACE]) {
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
    } else if p.at(SCRIPT_BODY) {
        if p.at(SCRIPT_BODY) {
            script(p)?;
        }
    } else {
        p.expected_ts(&AT_HTML_CONTENT)?;
    }
    Some(Continue)
}

pub fn script(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = catch!({ p.expect(SCRIPT_BODY) });
    p.complete(_marker, SCRIPT);
    _ok
}

pub fn html_attribute(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = catch!({
        p.expect(TAG_NAME)?;
        if p.at_ts(&tokenset![EQ, WS]) {
            p.eat(WS);
            p.expect(EQ)?;
            p.eat(WS);
            html_attribute_value(p)?;
        }
        Some(Continue)
    });
    p.complete(_marker, ATTRIBUTE);
    _ok
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

pub fn doctype(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = catch!({
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
    });
    p.complete(_marker, DOCUMENT_TYPE);
    _ok
}

const AT_HTML_CONTENT: TokenSet = tokenset![COMMENT, L_ANGLE, SCRIPT_BODY, TEXT, WHITESPACE];
