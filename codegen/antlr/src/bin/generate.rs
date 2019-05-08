use antlr_codegen::ast::*;
use antlr_codegen::grammar;
use antlr_codegen::queries::*;
use antlr_codegen::transform;
use combine::parser::Parser;
use combine::stream::state::State;
use std::collections::{BTreeMap as Map, BTreeSet as Set};
use std::fs;

const MIN_CONST_TOKENSET: usize = 5;

fn main() -> Result<(), std::io::Error> {
    let filepaths = &[
        ("codegen/antlr/grammars/html.g", "grammar/html/src/grammar.rs"),
        ("codegen/antlr/grammars/javascript.g", "grammar/javascript/src/grammar.rs")
    ];
    for (input, output) in filepaths {
        let content = fs::read_to_string(input)?;
        let result = grammar::grammar().easy_parse(State::new(content.as_str()));
        let (root, _) = match result {
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("while parsing file `{}`:\n{}", input, err);
                std::process::exit(1);
            }
        };

        let db = Database::from(root);
        let mut tokensets = Map::<_, Vec<&str>>::new();
        for rule in &db.grammar().rules {
            let ts = db.next_terms_of(&rule.pattern);
            if ts.len() >= MIN_CONST_TOKENSET {
                tokensets.entry(ts).or_default().push(&rule.name);
            }
        }
        for (ts, names) in tokensets {
            let mut left_corners = names
                .iter()
                .map(|a| (a, names.iter().filter(|b| db.is_left_corner(b, a)).collect::<Vec<_>>()))
                .collect::<Vec<_>>();
            left_corners.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
            let mut sources = Vec::new();
            while let Some((name, callers)) = left_corners.pop() {
                left_corners.retain(|(n, _)| callers.iter().all(|c| c != n));
                sources.push(name.to_string());
            }
            let name = sources.join("_OR_").to_uppercase();
            db.set_tokenset_name(ts, format!("AT_{}", name));
        }

        let mut out = String::new();
        out.push_str(&r#"
// This file is automatically generated by running `cargo run -p antlr_codegen`.
//
// =====================
// Do not edit manually.
// =====================
//
#![allow(dead_code)]
#![allow(unused_imports)]

"#[1..]);
        out.push_str("//! This module contains an auto-generated ");
        out.push_str(&db.grammar().name);
        out.push_str(" parser.\n");
        if db.grammar().rules.iter().any(|r| db.is_left_recursive(&r.name)) {
            out.push_str("use crate::grammar_ext;\n");
        }
        out.push_str(&r#"
use crate::syntax_kind::*;
use grammar_utils::{catch, tokenset, Parser, TokenSet};
use grammar_utils::parser::Continue;
"#[1..]);
        emit_grammar(&mut out, &db);

        let mut tokensets = db.tokensets();
        tokensets.sort_by(|a, b| a.0.cmp(&b.0));
        if tokensets.len() > 1 {
            out.push('\n');
        }
        for (name, set, used) in tokensets {
            if used {
                out.push_str("const ");
                out.push_str(&name);
                out.push_str(": TokenSet = tokenset![");
                emit_tokenset_list(&mut out, set);
                out.push_str("];\n");
            }
        }
        fs::write(output, out.as_bytes())?;
    }
    Ok(())
}

fn emit_grammar(out: &mut String, db: &Database) {
    for rule in &db.grammar().rules {
        out.push('\n');
        if !rule.name.starts_with("_") {
            out.push_str("pub ");
        }
        out.push_str("fn ");
        out.push_str(&rule.name);
        if rule.pattern.is_empty() {
            out.push_str("(_: &mut Parser) -> Option<Continue> { Some(Continue) }\n");
        } else {
            out.push_str("(p: &mut Parser) -> Option<Continue> {\n");
            let result = emit_pattern(out, &db, &rule.name, &rule.pattern, 1, Precond::empty(), &[]);
            match result {
                PatternType::Continue => (),
                PatternType::Unit =>{
                    emit_depth(out, 1);
                    out.push_str("Some(Continue)\n");
                }
            }
            out.push_str("}\n");
        }
    }
}

enum PatternType {
    Continue,
    Unit,
}

fn emit_pattern<'a>(
    out: &mut String,
    db: &Database,
    rule: &str,
    pat: &Pattern,
    dep: u8,
    precond: Precond,
    next_patterns: impl IntoIterator<Item = &'a Pattern>,
) -> PatternType {
    match pat {
        Pattern::Empty => (),
        Pattern::Literal(lit) => {
            let ident = Pattern::Ident(String::from(to_token_of_literal(lit)));
            emit_pattern(out, db, rule, &ident, dep, precond, &[]);
        }
        Pattern::Ident(ident) => {
            emit_depth(out, dep);
            if is_term(ident) {
                if let Some(set) = precond.includes {
                    if set.len() == 1 && set.iter().next().map(|x| x.token.as_str()) == Some(&ident) {
                        out.push_str("p.bump();\n");
                        return PatternType::Unit;
                    }
                }
                out.push_str(&format!("p.expect({})?;\n", ident));
            } else {
                out.push_str(&format!("{}(p)?;\n", ident));
            }
        }
        Pattern::Series(series) => {
            let mut iter = series.into_iter().enumerate();

            // Only propogate the precondition to the first pattern in the series
            if let Some((i, pat)) = iter.next() {
                emit_pattern(out, db, rule, pat, dep, precond, series.into_iter().skip(i + 1));
            }

            // Emit the remainder of the series
            for (i, pat) in iter {
                emit_pattern(out, db, rule, pat, dep, Precond::empty(), series.into_iter().skip(i + 1));
            }
        }
        outer @ Pattern::Choice(_) => {
            return emit_choice(out, db, rule, outer, dep, precond);
        }
        Pattern::Repeat(pat, repeat) => {
            match repeat {
                Repeat::ZeroOrOne => {
                    let ts = db.next_predicates_of(pat);
                    if pat.is_term() {
                        emit_depth(out, dep);
                        out.push_str("p.eat(");
                        out.push_str(&ts.into_iter().next().map(|t| t.token).unwrap());
                        out.push_str(");\n");
                        return PatternType::Unit;
                    }
                    emit_depth(out, dep);
                    out.push_str("if ");
                    emit_lookahead(out, db, &ts, false);
                    out.push_str(" {\n");

                    // Handle ambiguity with following terms
                    if db.is_possibly_empty(pat) && {
                        let ts = ts.iter().cloned().map(|t| t.token.into()).collect::<Set<_>>();
                        let mut next_ts = Set::new();
                        for pat in next_patterns {
                            next_ts.extend(db.next_terms_of(pat));
                            if !db.is_possibly_empty(pat) {
                                break;
                            }
                        }
                        !ts.is_disjoint(&next_ts)
                    } {
                        emit_let_checkpoint(out, true, dep + 1);
                        emit_catch(out, db, rule, pat, dep + 1, Precond::one_of(ts), None);
                        emit_depth(out, dep + 1);
                        out.push_str("p.commit(_checkpoint)?.ok()\n");
                    } else {
                        emit_pattern(out, db, rule, pat, dep + 1, Precond::one_of(ts), &[]);
                    }
                    emit_depth(out, dep);
                    out.push_str("}\n");
                }
                Repeat::ZeroOrMore => {
                    let ts = db.next_predicates_of(pat);
                    if pat.is_term() {
                        let term = ts.into_iter().next().unwrap();
                        emit_depth(out, dep);
                        out.push_str("while p.eat(");
                        out.push_str(&term.token);
                        out.push_str(") {}\n");
                    } else {
                        emit_depth(out, dep);
                        out.push_str("while ");
                        emit_lookahead(out, db, &ts, false);
                        out.push_str(" {\n");
                        emit_pattern(out, db, rule, pat, dep + 1, Precond::one_of(ts), &[]);
                        emit_depth(out, dep);
                        out.push_str("}\n");
                    }
                }
                Repeat::OneOrMore => {
                    let ts = db.next_predicates_of(pat);
                    if pat.is_term() {
                        let term = ts.into_iter().next().unwrap();
                        emit_pattern(out, db, rule, &Pattern::Ident(term.token.clone().into()), dep, precond, &[]);
                        emit_depth(out, dep);
                        out.push_str("while p.eat(");
                        out.push_str(&term.token);
                        out.push_str(") {}\n");
                    } else {
                        emit_depth(out, dep);
                        out.push_str("loop {\n");
                        emit_pattern(out, db, rule, pat, dep + 1, Precond::empty(), &[]);
                        emit_depth(out, dep + 1);
                        out.push_str("if !");
                        emit_lookahead(out, db, &ts, true);
                        out.push_str(" { break }\n");
                        emit_depth(out, dep);
                        out.push_str("}\n");
                    }
                }
            }
        }
        Pattern::Predicate(expr, pat) => {
            let includes = precond.includes.unwrap_or_default();
            let excludes = precond.excludes.unwrap_or_default();
            if includes.len() == 0 || !includes.iter().all(|t| &t.predicate == expr) {
                emit_depth(out, dep);
                out.push_str("if !(");
                emit_predicate_expr(out, expr);
                out.push_str(") {\n");
                emit_depth(out, dep + 1);
                out.push_str("p.error(\"expected input to be ");
                emit_predicate_description(out, expr);
                out.push_str("\")?;\n");
                emit_depth(out, dep);
                out.push_str("}\n");
            }
            let includes = includes.into_iter().map(QualifiedTerm::unqualified).collect();
            let precond = Precond::one_of(includes).but_not(&excludes);
            emit_pattern(out, db, rule, pat, dep, precond, &[]);
        }
        Pattern::Precedence(nonterm, prec) => {
            emit_depth(out, dep);
            out.push_str(nonterm);
            out.push_str("(p, ");
            out.push_str(&prec.to_string());
            out.push_str(");\n");
        }
        Pattern::Node(kind, pat) => {
            let marker_name = emit_let_marker(out, dep);
            let must_catch = emit_catch(out, db, rule, pat, dep, precond, Some("_ok"));
            emit_depth(out, dep);
            out.push_str("p.complete(");
            out.push_str(&marker_name);
            out.push_str(", ");
            out.push_str(kind);
            out.push_str(");\n");
            if must_catch {
                if dep > 1 {
                    emit_depth(out, dep);
                    out.push_str("if _ok.is_none() {\n");
                    emit_depth(out, dep + 1);
                    out.push_str("return None;\n");
                    emit_depth(out, dep);
                    out.push_str("}\n");
                } else {
                    emit_depth(out, dep);
                    out.push_str("_ok\n");
                    return PatternType::Continue;
                }
            }
        }
        Pattern::NodeStart(pat) => {
            emit_let_checkpoint(out, false, dep);
            emit_let_marker(out, dep);
            emit_pattern(out, db, rule, pat, dep, precond, &[]);
        }
        Pattern::NodeComplete(kind, pat) => {
            emit_pattern(out, db, rule, pat, dep, precond, &[]);
            emit_depth(out, dep);
            out.push_str("p.complete(_checkpoint.branch(&_marker), ");
            out.push_str(kind);
            out.push_str(");\n");
        }
    }
    PatternType::Unit
}

fn emit_catch(out: &mut String, db: &Database, rule: &str, pat: &Pattern, dep: u8, precond: Precond, label: Option<&str>) -> bool {
    let includes = precond.includes.clone().unwrap_or_default();
    // Handle a single terminal
    if pat.is_term() {
        let ts = db.next_predicates_of(pat);
        if ts == includes {
            emit_depth(out, dep);
            out.push_str("p.bump();\n");
            return false;
        }

        let term = ts.into_iter().next().unwrap();
        if let Some(label) = label {
            emit_depth(out, dep);
            out.push_str("let ");
            out.push_str(label);
            out.push_str(" = ");
            out.push_str("catch!({ p.expect(");
            out.push_str(&term.token);
            out.push_str(") });\n");
        } else {
            emit_depth(out, dep);
            out.push_str("p.expect(");
            out.push_str(&term.token);
            out.push_str(");\n");
        }
        return true;
    }

    // Handle a single non-terminal
    if pat.is_nonterm() {
        let nonterm = db.next_symbols_of(pat).into_iter().next().unwrap();
        if let Some(label) = label {
            emit_depth(out, dep);
            out.push_str("let ");
            out.push_str(label);
            out.push_str(" = ");
            out.push_str("catch!({ ");
            out.push_str(&nonterm);
            out.push_str("(p) });\n");
        } else {
            emit_depth(out, dep);
            out.push_str(nonterm);
            out.push_str("(p);\n");
        }
        return true;
    }

    // Handle an enum
    if pat.is_enum() {
        let ts = db.next_predicates_of(pat);
        if ts == includes {
            emit_depth(out, dep);
            out.push_str("p.bump();\n");
            return false;
        }

        if let Some(label) = label {
            emit_depth(out, dep);
            out.push_str("let ");
            out.push_str(label);
            out.push_str(" = ");
            out.push_str("catch!({ p.expect_ts(&");
            emit_tokenset(out, db, ts.into_iter().map(|t| t.token.into()).collect());
            out.push_str(") });\n");
        } else {
            emit_depth(out, dep);
            out.push_str("p.expect_ts(&");
            emit_tokenset(out, db, ts.into_iter().map(|t| t.token.into()).collect());
            out.push_str(");\n");
        }
        return true;
    }

    // Handle the general case
    emit_depth(out, dep);
    if let Some(label) = label {
        out.push_str("let ");
        out.push_str(label);
        out.push_str(" = ");
    }
    out.push_str("catch!({\n");
    emit_pattern(out, db, rule, pat, dep + 1, precond, &[]);
    emit_depth(out, dep + 1);
    out.push_str("Some(Continue)\n");
    emit_depth(out, dep);
    out.push_str("});\n");
    true
}

fn emit_choice(
    out: &mut String,
    db: &Database,
    rule: &str,
    pat: &Pattern,
    dep: u8,
    precond: Precond,
) -> PatternType {
    let mut precond = precond;

    if pat.is_enum() {
        let tokens = db.next_terms_of(pat);
        emit_depth(out, dep);
        out.push_str("p.expect_ts(&");
        emit_tokenset(out, db, tokens);
        out.push(')');
        if dep > 1 {
            out.push_str("?;\n");
            return PatternType::Unit;
        }
        out.push('\n');
        return PatternType::Continue;
    }

    //
    // TODO: Maybe implement `inlining` of all nonterminals that are not also used by other rules
    //

    // Extract any shared prefix
    let (prefix, suffix) = match pat {
        Pattern::Choice(choices) => transform::unshift_all(&choices),
        _ => unreachable!(),
    };
    if !prefix.is_empty() {
        emit_pattern(out, db, rule, &prefix, dep, precond.take(), &[]);
    }

    // If the remaining pattern is unambiguous, emit a simple 1-token lookahead parser
    let pat = &suffix;
    if !db.is_next_term_ambiguous(pat) {
        emit_choice_ll1(out, db, rule, pat, dep, precond);
        return PatternType::Unit;
    }

    // Otherwise, _iff_ this is the top-level pattern, emit a backtracking parser
    if dep > 1 {
        panic!("found nested ambiguous alternatives in rule '{}': {}", rule, pat);
    }
    if db.is_left_corner_of(pat, rule) {
        if emit_choice_lr_precedence_climbing(out, db, rule, pat, dep) {
            return PatternType::Unit;
        }

        eprintln!("warn: left recursive rule '{}' must be implemented by hand in 'grammar_ext.rs'", rule);
        emit_depth(out, dep);
        out.push_str("grammar_ext::");
        out.push_str(rule);
        out.push_str("(p)");
        if dep > 1 {
            out.push_str("?;\n");
            return PatternType::Unit;
        }
        out.push('\n');
        return PatternType::Continue;
    }
    emit_choice_ambiguous(out, db, rule, pat, dep, precond);
    return PatternType::Unit;
}

fn emit_choice_ambiguous(out: &mut String, db: &Database, rule: &str, pat: &Pattern, dep: u8, precond: Precond) {
    match pat {
        Pattern::Choice(choices) => {
            let is_not = precond.excludes.unwrap_or_default();
            let mut ts_count = Map::new();
            for pat in choices {
                for t in db.next_predicates_of(pat).difference(&is_not) {
                    ts_count.entry(t.clone().unqualified()).and_modify(|n| *n += 1).or_insert(1);
                }
            }
            emit_depth(out, dep);
            for (i, pat) in choices.iter().enumerate() {
                fn emit_if(out: &mut String, i: usize) {
                    if i > 0  {
                        out.push_str(" else if ");
                    } else {
                        out.push_str("if ");
                    }
                }
                let ts: Set<_> = db.next_predicates_of(pat).difference(&is_not).cloned().collect();
                for t in &ts {
                    ts_count.entry(t.clone().unqualified()).and_modify(|count| *count -= 1).or_insert(0);
                }
                let is_trivial_predicate = match pat {
                    Pattern::Predicate(_, tail) => match tail.as_ref() {
                        Pattern::Ident(_) | Pattern::Literal(_) => true,
                        _ => false,
                    }
                    _ => false,
                };
                let is_ambigous_choice =
                    !is_trivial_predicate &&
                        ts.iter().any(|t| ts_count[&t.clone().unqualified()] > 0);
                if ts.len() > 0 {
                    emit_if(out, i);
                    emit_lookahead(out, db, &ts, is_ambigous_choice);
                } else if pat.is_empty() {
                    if i > 0 {
                        continue;
                    } else {
                        panic!("unsupported! choice starts with empty pattern in rule `{}`", rule);
                    }
                } else {
                    panic!("unreachable pattern for choice in rule `{}`: {}", rule, pat);
                }

                if is_ambigous_choice {
                    out.push_str(" && {\n");
                    emit_depth(out, dep + 1);
                    out.push_str("// try --> ");
                    out.push_str(&pat.to_string());
                    out.push('\n');
                    emit_let_checkpoint(out, true, dep + 1);
                    emit_catch(out, db, rule, pat, dep + 1, Precond::one_of(ts).but_not(&is_not), None);
                    emit_depth(out, dep + 1);
                    out.push_str("p.commit(_checkpoint)?.is_ok()\n");
                    emit_depth(out, dep);
                    out.push_str("} {\n");
                    emit_depth(out, dep + 1);
                    out.push_str("// ok\n");
                    emit_depth(out, dep);
                    out.push('}');
                } else {
                    out.push_str(" {\n");
                    emit_pattern(out, db, rule, pat, dep + 1, Precond::one_of(ts).but_not(&is_not), &[]);
                    emit_depth(out, dep);
                    out.push('}');
                }
            }
        }
        _ => unreachable!(),
    }

    out.push_str(" else {\n");
    emit_depth(out, dep + 1);
    out.push_str("// otherwise, emit an error\n");
    emit_expected(out, db, pat, dep + 1);
    emit_depth(out, dep);
    out.push_str("}\n");
}

/// Transforms directly left recursive rules which can be disambiguated
/// with k=2 token lookahead to a non left-recursive rule while preserving
/// the parse precedence.
///
/// For example:
///
/// > expr : expr "++"
/// >      | expr "--"
/// >      | expr "+" expr
/// >      | expr "-" expr
/// >      | "++" expr
/// >      | "--" expr
/// >      | expr "*" expr
/// >      | expr "/" expr
/// >      | "(" expr ")"
/// >      | TERM
/// >      ;
///
/// Becomes:
///
/// > expr    : prec(0) ;
/// > tail    : "++" prec(4)
/// >         | "--" prec(4)
/// >         | "(" expr ")"
/// >         | TERM
/// >         ;
/// > prec(n) : tail
/// >           ( {p ≤ 5}? "/" prec(6) )*
/// >           ( {p ≤ 4}? "*" prec(5) )*
/// >           ( {p ≤ 3}? "-" prec(4) )*
/// >           ( {p ≤ 2}? "+" prec(3) )*
/// >           ( {p ≤ 1}? "--" )*
/// >           ( {p ≤ 0}? "++" )*
/// >         ;
///
fn emit_choice_lr_precedence_climbing(out: &mut String, db: &Database, rule: &str, pat: &Pattern, dep: u8) -> bool {
    let choices = match pat {
        Pattern::Choice(choices) => choices,
        _ => unreachable!(),
    };
    let mut max_prec = 0;
    let mut prec_patterns = Vec::new();
    let mut head_patterns = Vec::new();
    for pat in choices {
        if db.is_direct_left_corner_of(pat, rule) {
            prec_patterns.push((pat, max_prec));
            max_prec += 1;
        } else {
            head_patterns.push((pat, max_prec));
        }
    }

    let head_ident = format!("_{}_head", rule);
    let prec_ident = format!("_{}_prec", rule);

    let mut prec_prefix = None;
    for prefix in prec_patterns.iter().map(|(pat, _)| transform::unshift(pat).unwrap().0) {
        if prec_prefix.is_none() {
            prec_prefix = Some(prefix);
        } else if prec_prefix != Some(prefix) {
            return false;
        }
    }

    // Emit head parser
    let mut head_choices = Vec::with_capacity(head_patterns.len());
    for (pat, prec) in head_patterns {
        if prec == max_prec {
            head_choices.push(pat.clone());
        } else {
            head_choices.push(transform::convert_next_nonterm(pat, rule, &prec_ident, prec).0);
        }
    }
    let head_pat = Pattern::Choice(head_choices);
    if db.is_next_term_ambiguous(&head_pat) {
        return false;
    }

    let _ = (out, dep, head_ident);
    false

    // TODO: Update implementation to be correct if I ever want to perform this transform
    /*
    emit_depth(out, dep);
    out.push_str("fn ");
    out.push_str(&head_ident);
    out.push_str("(p: &mut Parser) -> Option<Continue> {\n");
    emit_choice(out, db, rule, &head_pat, dep + 1, Precond::empty());
    emit_depth(out, dep + 1);
    out.push_str("Some(Continue)\n");
    emit_depth(out, dep);
    out.push_str("}\n\n");

    // Emit prec parser
    emit_depth(out, dep);
    out.push_str("fn ");
    out.push_str(&prec_ident);
    out.push_str("(p: &mut Parser, prec: u32) {\n");
    emit_pattern(out, db, rule, &Pattern::Ident(head_ident.clone()), dep + 1, Precond::empty(), &[]);
    for (pat, prec) in prec_patterns.into_iter().rev() {
        let (_, tail) = transform::unshift(pat).unwrap();
        let suffix = transform::convert_next_nonterm(&tail, rule, &prec_ident, 0).0;
        emit_depth(out, dep + 1);
        let ts = db.next_predicates_of(&suffix);
        out.push_str("while ");
        emit_lookahead(out, ts);
        out.push_str(" && prec <= ");
        out.push_str(&prec.to_string());
        out.push_str(" {\n");
        emit_pattern(out, db, rule, &suffix, dep + 2, Precond::one_of(ts), &[]);
        emit_depth(out, dep + 1);
        out.push_str("}\n");
    }
    println!("info: performed lr_precedence_climbing for '{}'", rule);

    true
    */
}

fn emit_choice_ll1(out: &mut String, db: &Database, rule: &str, pat: &Pattern, dep: u8, precond: Precond) {
    if db.next_predicates_of(pat).is_empty() {
        panic!("unsupported! choice consists only of predicates and/or empty patterns");
    }

    let mut is_not = precond.excludes.unwrap_or_default();
    match pat {
        Pattern::Choice(choice) => {
            for (choice_num, pat) in choice.iter().enumerate() {
                fn emit_if(out: &mut String, dep: u8, i: usize) {
                    if i > 0  {
                        emit_depth(out, dep);
                        out.push_str("} else if ");
                    } else {
                        emit_depth(out, dep);
                        out.push_str("if ");
                    }
                }
                let ts: Set<_> = db.next_predicates_of(pat).difference(&is_not).cloned().collect();
                if ts.len() > 0 {
                    emit_if(out, dep, choice_num);
                    emit_lookahead(out, db, &ts, false);
                    out.push_str(" {\n");
                    emit_pattern(out, db, rule, pat, dep + 1, Precond::one_of(ts.clone()).but_not(&is_not), &[]);
                } else if pat.is_empty() {
                    if choice_num > 0 {
                        continue;
                    } else {
                        panic!("unsupported! choice starts with empty pattern in rule `{}`", rule);
                    }
                } else if let Pattern::Predicate(expr, pat) = pat {
                    emit_if(out, dep, choice_num);
                    emit_predicate_expr(out, expr);
                    out.push_str(" {\n");
                    emit_pattern(out, db, rule, pat, dep + 1, Precond::one_of(ts.clone()).but_not(&is_not), &[]);
                } else {
                    panic!("unreachable pattern for choice in rule `{}`: {}", rule, pat);
                }
                is_not.extend(ts);
            }
        }
        _ => unreachable!(),
    }
    let must_be = precond.includes.unwrap_or_default();
    let is_optional = match pat {
        Pattern::Choice(choices) => choices.iter().all(|pat| {
            if let Pattern::Predicate(_, pat) = pat {
                if pat.is_empty() {
                    return false;
                }
            }
            db.is_possibly_empty(pat)
        }),
        _ => unreachable!(),
    };
    if !is_optional && must_be.symmetric_difference(&is_not).count() > 0 {
        emit_depth(out, dep);
        out.push_str("} else {\n");
        emit_expected(out, db, pat, dep + 1);
        emit_depth(out, dep);
        out.push_str("}\n");
    } else {
        emit_depth(out, dep);
        out.push_str("}\n");
    }
}

fn emit_expected(out: &mut String, db: &Database, pat: &Pattern, dep: u8) {
    let tokens = db.next_terms_of(pat);
    emit_depth(out, dep);
    if tokens.len() > 1 {
        out.push_str("p.expected_ts(&");
        emit_tokenset(out, db, tokens);
    } else {
        out.push_str("p.expected(");
        out.push_str(&tokens.into_iter().next().unwrap());
    }
    out.push_str(")?;\n");
}

fn emit_tokenset(out: &mut String, db: &Database, tokens: Set<String>) {
    if tokens.len() >= MIN_CONST_TOKENSET {
        out.push_str(&db.tokenset_name(&tokens));
    } else if tokens.len() > 1 {
        out.push_str("tokenset![");
        emit_tokenset_list(out, tokens);
        out.push_str("]");
    } else {
        panic!("[internal error] tried to emit tokenset less than 2 tokens")
    }
}

fn emit_predicate_expr(out: &mut String, expr: &PredicateExpression) {
    match expr {
        PredicateExpression::Empty => out.push_str("true"),
        PredicateExpression::Call { method, args } => {
            out.push_str("p.");
            out.push_str(&method);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&arg);
            }
            out.push(')');
        }
        PredicateExpression::Unary { oper, expr } => {
            out.push(*oper);
            emit_predicate_expr(out, expr);
        }
        PredicateExpression::Binary { left, oper, right } => {
            emit_predicate_expr(out, left);
            out.push(' ');
            out.push_str(oper);
            out.push(' ');
            emit_predicate_expr(out, right);
        }
    }
}

fn emit_predicate_description(out: &mut String, expr: &PredicateExpression) {
    match expr {
        PredicateExpression::Empty => out.push_str("true"),
        PredicateExpression::Call { method, args } => {
            out.push_str(&method.replace("_", " "));
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(" (with ");
                    out.push_str(&arg);
                    out.push(')');
                } else {
                    out.push(' ');
                    out.push_str(&arg.replace("\\", "\\\\").replace("\"", "'"));
                }
            }
        }
        PredicateExpression::Unary { oper, expr } => {
            if *oper == '!' {
                out.push_str("not ");
            } else {
                out.push(*oper);
                out.push(' ');
            }
            emit_predicate_description(out, expr);
        }
        PredicateExpression::Binary { left, oper, right } => {
            emit_predicate_description(out, left);
            out.push(' ');
            if *oper == "&&" {
                out.push_str("and");
            } else if *oper == "||" {
                out.push_str("or");
            } else {
                out.push_str(*oper);
            }
            out.push(' ');
            emit_predicate_description(out, right);
        }
    }
}

fn emit_let_marker(out: &mut String, dep: u8) -> String {
    let marker_ident = format!("_marker");
    emit_depth(out, dep);
    out.push_str("let ");
    out.push_str(&marker_ident);
    out.push_str(" = p.start();\n");
    marker_ident
}

fn emit_let_checkpoint(out: &mut String, rollback: bool, dep: u8) -> String {
    let checkpoint_ident = format!("_checkpoint");
    emit_depth(out, dep);
    out.push_str("let mut ");
    out.push_str(&checkpoint_ident);
    if rollback {
        out.push_str(" = p.checkpoint(true);\n");
    } else {
        out.push_str(" = p.checkpoint(false);\n");
    }
    checkpoint_ident
}

fn emit_tokenset_list<'a, Iter>(out: &mut String, iter: Iter)
where
    Iter: IntoIterator,
    Iter::Item: AsRef<str>,
{
    for (i, tok) in iter.into_iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(tok.as_ref());
    }
}

fn emit_lookahead<'a, Iter>(out: &mut String, db: &Database, iter: Iter, subexpr: bool)
where
    Iter: IntoIterator<Item = &'a QualifiedTerm>,
{
    // First we want to group terms by predicate (so that we only check each predicate once)
    let mut pred_groups: Map<_, Set<_>> = Map::new();
    for term in iter {
        pred_groups.entry(&term.predicate).or_default().insert(term.token.as_str());
    }

    // Then for each predicate, filter out the terms which are also checked with no predicate
    let unqualifed = pred_groups.get(&PredicateExpression::Empty).cloned().unwrap_or_default();
    for (pred, tokens) in &mut pred_groups {
        let has_predicate = match pred { PredicateExpression::Empty => false, _ => true };
        if has_predicate {
            for tok in &unqualifed {
                tokens.remove(tok);
            }
        }
    }

    // Group the remaining non-empty `(predicate, tokenset)` pairs by tokenset,
    // preparing predicates to be combined with `||` (logical or) such that the
    // tokenset only needs to be tested once if any of the predicates match.
    //
    // Don't insert the null predicate such that the universal tokenset will have
    // an empty predicate list; because we filter out tokensets where all tokens
    // match with the null predicate this operation should never cause a universal
    // tokenset to be mixed with a conditional tokenset.
    let mut ts_groups: Map<Set<_>, Vec<_>> = Map::new();
    for (pred, ts) in pred_groups {
        if !ts.is_empty() {
            let list = ts_groups.entry(ts).or_default();
            match pred {
                PredicateExpression::Empty => (),
                _ => list.push(pred),
            };
        }
    }

    let mut groups = ts_groups.into_iter().collect::<Vec<_>>();
    groups.sort_by(|a, b| a.1.cmp(&b.1)); // sort by predicates (e.g. expect "Empty" first)
    if subexpr && groups.len() > 1 {
        out.push('(');
    }
    for (i, (tokens, preds)) in groups.iter().enumerate() {
        if i > 0 {
            out.push_str(" || ");
        }
        if !preds.is_empty() {
            if subexpr || groups.len() > 1 {
                out.push('(');
            }

            // N.B. join any predicates with LOGICAL_OR, while minimizing grouping operators
            if preds.len() > 1 {
                out.push('(');
                for (j, pred) in preds.iter().enumerate() {
                    if j > 0 {
                        out.push_str(" || ");
                    }
                    emit_predicate_expr(out, pred);
                }
                out.push(')');
            } else {
                emit_predicate_expr(out, &preds[0]);
            }

            out.push_str(" && ");
        }
        if tokens.len() > 1 {
            out.push_str("p.at_ts(&");
            emit_tokenset(out, db, tokens.iter().map(|x| x.to_string()).collect());
            out.push_str(")");
        } else {
            out.push_str("p.at(");
            emit_tokenset_list(out, tokens);
            out.push_str(")");
        }
        if !preds.is_empty() && (subexpr || groups.len() > 1) {
            out.push(')');
        }
    }
    if subexpr && groups.len() > 1 {
        out.push(')');
    }
}

fn emit_depth(out: &mut String, depth: u8) {
    for _ in 0..depth {
        out.push_str("    ");
    }
}

#[derive(Clone, Debug)]
struct Precond {
    includes: Option<Set<QualifiedTerm>>,
    excludes: Option<Set<QualifiedTerm>>,
}

impl Precond {
    fn empty() -> Precond {
        Precond {
            includes: None,
            excludes: None,
        }
    }
    fn one_of(set: Set<QualifiedTerm>) -> Precond {
        Precond {
            includes: Some(set),
            excludes: None,
        }
    }
    fn but_not(self, set: &Set<QualifiedTerm>) -> Precond {
        Precond {
            includes: self.includes
                .map(|x| x.difference(&set).cloned().collect()),
            excludes: self.excludes
                .map(|x| x.union(&set).cloned().collect())
                .or_else(|| Some(set.clone())),
        }
    }

    fn take(&mut self) -> Precond {
        Precond {
            includes: self.includes.take(),
            excludes: self.excludes.take()
        }
    }
}