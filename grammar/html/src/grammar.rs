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
use code_grammar::{catch, tokenset, Parser, TokenSet};
use code_grammar::parser::Continue;

pub fn document(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = catch!({
        p.eat(WS);
        if p.at(L_ANGLE_BANG) {
            doctype(p)?;
        }
        p.eat(WS);
        while p.at_ts(&tokenset![COMMENT, L_ANGLE, TEXT, WHITESPACE]) {
            let _checkpoint = p.checkpoint_ambiguous();
            elements(p);
            if !p.commit(_checkpoint)?.is_ok() {
                break;
            }
        }
        p.expect(EOF)?;
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
            p.error("expected to be at keyword 'DOCTYPE'")?;
        }
        p.expect(IDENT)?;
        p.eat(WS);
        p.expect_ts(&tokenset![IDENT, QUOTED])?;
        p.eat(WS);
        while p.at_ts(&tokenset![IDENT, QUOTED]) {
            p.expect_ts(&tokenset![IDENT, QUOTED])?;
            p.eat(WS);
        }
        p.expect(R_ANGLE)?;
        Some(Continue)
    });
    p.complete(_marker, DOCUMENT_TYPE);
    _ok
}

pub fn elements(p: &mut Parser) -> Option<Continue> {
    while p.at_ts(&tokenset![COMMENT, TEXT, WHITESPACE]) {
        html_misc(p)?;
    }
    element(p)?;
    while p.at_ts(&tokenset![COMMENT, TEXT, WHITESPACE]) {
        html_misc(p)?;
    }
    Some(Continue)
}

pub fn element(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = element_pattern(p);
    p.complete(_marker, ELEMENT);
    _ok
}

pub fn element_pattern(p: &mut Parser) -> Option<Continue> {
    p.expect(L_ANGLE)?;
    if ((p.at_keyword("area") || p.at_keyword("base") || p.at_keyword("br") || p.at_keyword("col") || p.at_keyword("embed") || p.at_keyword("hr") || p.at_keyword("img") || p.at_keyword("input") || p.at_keyword("link") || p.at_keyword("meta") || p.at_keyword("param") || p.at_keyword("source") || p.at_keyword("track") || p.at_keyword("wbr")) && p.at(TAG_NAME)) && {
        // try --> empty_element_tag_name (WS)? (attribute (WS)?)* ('>' | '/>')
        let mut _checkpoint = p.checkpoint(true);
        catch!({
            empty_element_tag_name(p)?;
            p.eat(WS);
            while p.at(TAG_NAME) {
                let _checkpoint = p.checkpoint_ambiguous();
                catch!({
                    attribute(p)?;
                    p.eat(WS);
                    Some(Continue)
                });
                if !p.commit(_checkpoint)?.is_ok() {
                    break;
                }
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
            let _checkpoint = p.checkpoint_ambiguous();
            catch!({
                attribute(p)?;
                p.eat(WS);
                Some(Continue)
            });
            if !p.commit(_checkpoint)?.is_ok() {
                break;
            }
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
                p.expected_ts_in("element_pattern", &tokenset![L_ANGLE, L_ANGLE_SLASH])?;
            }
            p.eat(WS);
            p.expect(TAG_NAME)?;
            p.eat(WS);
            p.expect(R_ANGLE)?;
        } else {
            p.expected_ts_in("element_pattern", &tokenset![R_ANGLE, SLASH_R_ANGLE])?;
        }
    } else {
        // otherwise, emit an error
        p.expected_in("element_pattern", TAG_NAME)?;
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
        p.expected_in("empty_element_tag_name", TAG_NAME)?;
    }
    Some(Continue)
}

pub fn attribute(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = catch!({
        p.expect(TAG_NAME)?;
        if p.at_ts(&tokenset![EQ, WS]) {
            let mut _checkpoint = p.checkpoint(true);
            catch!({
                p.eat(WS);
                p.expect(EQ)?;
                p.eat(WS);
                attribute_value(p)?;
                Some(Continue)
            });
            p.commit(_checkpoint)?.ok();
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
            let _checkpoint = p.checkpoint_ambiguous();
            catch!({
                if p.at(L_ANGLE) {
                    element(p)?;
                } else if p.at(COMMENT) {
                    p.bump();
                }
                if p.at_ts(&tokenset![TEXT, WHITESPACE]) {
                    html_chardata(p)?;
                }
                Some(Continue)
            });
            if !p.commit(_checkpoint)?.is_ok() {
                break;
            }
        }
    } else if p.at(SCRIPT_CONTENT) {
        if p.at(SCRIPT_CONTENT) {
            script_block(p)?;
        }
    } else if p.at(STYLE_CONTENT) {
        if p.at(STYLE_CONTENT) {
            style_block(p)?;
        }
    }
    Some(Continue)
}

pub fn html_chardata(p: &mut Parser) -> Option<Continue> {
    p.expect_ts(&tokenset![TEXT, WHITESPACE])
}

pub fn html_misc(p: &mut Parser) -> Option<Continue> {
    p.expect_ts(&tokenset![COMMENT, TEXT, WHITESPACE])
}

pub fn script_block(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = p.expect(SCRIPT_CONTENT);
    p.complete(_marker, SCRIPT_BLOCK);
    _ok
}

pub fn style_block(p: &mut Parser) -> Option<Continue> {
    let _marker = p.start();
    let _ok = p.expect(STYLE_CONTENT);
    p.complete(_marker, STYLE_BLOCK);
    _ok
}
