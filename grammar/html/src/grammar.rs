// This file is automatically generated by running `cargo run -p antlr_codegen`.
//
// =====================
// Do not edit manually.
// =====================
//
#![allow(dead_code)]
#![allow(unused_imports)]

//! This module contains an auto-generated HTML parser.
use crate::syntax_kind::*;
use grammar_utils::{catch, tokenset, Parser, TokenSet};
use grammar_utils::parser::Continue;

pub fn document(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = catch!({
        p.eat(WS);
        if p.at(L_ANGLE_BANG) {
            doctype(p)?;
        }
        p.eat(WS);
        while p.at_ts(&tokenset![COMMENT, L_ANGLE, WHITESPACE]) {
            elements(p)?;
        }
        Some(Continue)
    });
    p.complete(_marker, DOCUMENT);
    _ok
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

pub fn elements(p: &mut Parser) -> Option<Continue> {
    while p.at_ts(&tokenset![COMMENT, WHITESPACE]) {
        html_misc(p)?;
    }
    element(p)?;
    while p.at_ts(&tokenset![COMMENT, WHITESPACE]) {
        html_misc(p)?;
    }
    Some(Continue)
}

pub fn element(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = catch!({ element_pattern(p) });
    p.complete(_marker, ELEMENT);
    _ok
}

pub fn element_pattern(p: &mut Parser) -> Option<Continue> {
    p.expect(L_ANGLE)?;
    if ((p.at_keyword("area") || p.at_keyword("base") || p.at_keyword("br") || p.at_keyword("col") || p.at_keyword("embed") || p.at_keyword("hr") || p.at_keyword("img") || p.at_keyword("input") || p.at_keyword("link") || p.at_keyword("meta") || p.at_keyword("param") || p.at_keyword("source") || p.at_keyword("track") || p.at_keyword("wbr")) && p.at(TAG_NAME)) && {
        // try --> empty_element_tag_name (WS)? (attribute (WS)?)* (> | />)
        let mut _checkpoint = p.checkpoint(true);
        catch!({
            empty_element_tag_name(p)?;
            p.eat(WS);
            while p.at(TAG_NAME) {
                attribute(p)?;
                p.eat(WS);
            }
            p.expect_ts(&tokenset![R_ANGLE, SLASH_R_ANGLE])?;
            Some(Continue)
        });
        p.commit(_checkpoint)?.is_ok()
    } {
        // ok
    } else if p.at(TAG_NAME) {
        p.bump();
        p.eat(WS);
        while p.at(TAG_NAME) {
            attribute(p)?;
            p.eat(WS);
        }
        if p.at(SLASH_R_ANGLE) {
            p.bump();
        } else if p.at(R_ANGLE) {
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
        } else {
            p.expected_ts(&tokenset![R_ANGLE, SLASH_R_ANGLE])?;
        }
    } else {
        // otherwise, emit an error
        p.expected(TAG_NAME)?;
    }
    Some(Continue)
}

pub fn empty_element_tag_name(p: &mut Parser) -> Option<Continue> {
    if p.at_keyword("area") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("base") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("br") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("col") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("embed") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("hr") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("img") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("input") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("link") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("meta") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("param") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("source") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("track") && p.at(TAG_NAME) {
        p.bump();
    } else if p.at_keyword("wbr") && p.at(TAG_NAME) {
        p.bump();
    } else {
        // otherwise, emit an error
        p.expected(TAG_NAME)?;
    }
    Some(Continue)
}

pub fn attribute(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = catch!({
        p.expect(TAG_NAME)?;
        if p.at_ts(&tokenset![EQ, WS]) {
            p.eat(WS);
            p.expect(EQ)?;
            p.eat(WS);
            attribute_value(p)?;
        }
        Some(Continue)
    });
    p.complete(_marker, ATTRIBUTE);
    _ok
}

pub fn attribute_value(p: &mut Parser) -> Option<Continue> {
    p.expect_ts(&tokenset![QUOTED, TAG_NAME])
}

pub fn html_content(p: &mut Parser) -> Option<Continue> {
    if p.at_ts(&tokenset![COMMENT, L_ANGLE, TEXT, WHITESPACE]) {
        if p.at_ts(&tokenset![TEXT, WHITESPACE]) {
            html_chardata(p)?;
        }
        while p.at_ts(&tokenset![COMMENT, L_ANGLE]) {
            if p.at(L_ANGLE) {
                element(p)?;
            } else if p.at(COMMENT) {
                p.bump();
            }
            if p.at_ts(&tokenset![TEXT, WHITESPACE]) {
                html_chardata(p)?;
            }
        }
    } else if p.at(SCRIPT_CONTENT) {
        if p.at(SCRIPT_CONTENT) {
            script(p)?;
        }
    }
    Some(Continue)
}

pub fn html_chardata(p: &mut Parser) -> Option<Continue> {
    p.expect_ts(&tokenset![TEXT, WHITESPACE])
}

pub fn html_misc(p: &mut Parser) -> Option<Continue> {
    p.expect_ts(&tokenset![COMMENT, WHITESPACE])
}

pub fn script(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = catch!({ p.expect(SCRIPT_CONTENT) });
    p.complete(_marker, SCRIPT);
    _ok
}
