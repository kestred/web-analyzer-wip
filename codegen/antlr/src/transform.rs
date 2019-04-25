use crate::ast::*;

// TODO: Build more general "HIR" (grammar abstract syntax-level)
//       to "MIR" (mid-/low-level parser definition) transform and data structures.

/*
/// Transforms directly left recursive rules which can be disambiguated
/// with k=2 token lookaprec to a non left-recursive rule while preserving
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
/// > head    : "++" prec(4)
/// >         | "--" prec(4)
/// >         | "(" expr ")"
/// >         | TERM
/// >         ;
/// > prec(n) : head
/// >           ( {p ≤ 5}? "/" prec(5) )*
/// >           ( {p ≤ 4}? "*" prec(4) )*
/// >           ( {p ≤ 3}? "-" prec(3) )*
/// >           ( {p ≤ 2}? "+" prec(2) )*
/// >           ( {p ≤ 1}? "--" )*
/// >           ( {p ≤ 0}? "++" )*
/// >         ;
///
pub fn lr_operator_transform(db: &Database, rule: &str, choices: &[Pattern]) -> (Rule, Rule, Rule) {
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
    let mut head_choices = Vec::with_capacity(head_patterns.len());
    for (pat, prec) in head_patterns {
        if prec == max_prec {
            head_choices.push(pat.clone());
        } else {
            head_choices.push(convert_next_nonterm(pat, rule, &prec_ident, prec).0);
        }
    }
    let mut prec_series = Vec::with_capacity(prec_patterns.len() + 1);
    let mut prec_iter = prec_patterns.iter();
    let (head, tail) = unshift(prec_iter.next().unwrap().0).unwrap();
    prec_series.push(head.clone());
    prec_series.push(convert_next_nonterm(&tail, rule, &prec_ident, 0).0);
    for (pat, prec) in prec_iter {
        let (head2, tail2) = unshift(pat).unwrap();
        assert_eq!(head, head2);
        prec_series.push(Pattern::Repeat {
            pattern: Box::new(Pattern::Series(vec![
                Pattern::Predicate(PredicateExpression::TestPrecedence(*prec)),
                convert_next_nonterm(&tail2, rule, &prec_ident, *prec).0
            ])),
            repeat: Repeat::ZeroOrMore,
        });
    }

    let orig = Rule {
        name: rule.into(),
        pattern: Pattern::Precedence(prec_ident.clone(), 0),
        attributes: Vec::new(),
    };

    let head = Rule {
        name: head_ident,
        pattern: Pattern::Choice(head_choices),
        attributes: Vec::new(),
    };

    let prec = Rule {
        name: prec_ident,
        pattern: Pattern::Series(prec_series),
        attributes: vec![Attribute::Word("precedence".into())],
    };

    (orig, head, prec)
}
*/

pub fn convert_next_nonterm(pat: &Pattern, nonterm: &str, replace: &str, prec: u32) -> (Pattern, bool) {
    match pat {
        Pattern::Ident(ident) if ident == nonterm => (Pattern::Precedence(replace.into(), prec), true),
        Pattern::Series(series) => {
            let mut iter = series.iter();
            let mut result = Vec::with_capacity(series.len());
            while let Some(pat) = iter.next() {
                let (pat, replaced) = convert_next_nonterm(pat, nonterm, replace, prec);
                result.push(pat);
                if replaced {
                    result.extend(iter.cloned());
                    break;
                }
            }
            (Pattern::Series(result), true)
        }
        Pattern::Choice(choice) => {
            (Pattern::Choice(choice.iter().map(|pat| convert_next_nonterm(pat, nonterm, replace, prec).0).collect()), true)
        }
        Pattern::Repeat(pat, repeat) => {
           let (pat, ret) = convert_next_nonterm(pat, nonterm, replace, prec);
           (Pattern::Repeat(Box::new(pat), *repeat), ret)
        }
        Pattern::Predicate(expr, pat) => {
           let (pat, ret) = convert_next_nonterm(pat, nonterm, replace, prec);
           (Pattern::Predicate(expr.clone(), Box::new(pat)), ret)
        }
        Pattern::Node(name, pat) => {
           let (pat, ret) = convert_next_nonterm(pat, nonterm, replace, prec);
           (Pattern::Node(name.clone(), Box::new(pat)), ret)
        }
        _ => (pat.clone(), false),
    }
}

pub fn unshift(pat: &Pattern) -> Option<(Pattern, Pattern)> {
    match pat {
        Pattern::Node(name, pat) => {
            let (head, tail) = unshift(pat)?;
            Some((Pattern::NodeStart(Box::new(head)), Pattern::NodeComplete(name.clone(), Box::new(tail.flatten_once()))))
        }
        Pattern::NodeStart(_) => None,
        Pattern::NodeComplete(name, pat) => {
            let (head, tail) = unshift(pat)?;
            Some((head, Pattern::NodeComplete(name.clone(), Box::new(tail.flatten_once()))))
        }
        Pattern::Series(series) if series.len() > 1 => {
            let mut iter = series.iter();
            let head = iter.next()?;
            let tail = iter.cloned().collect();
            Some((head.clone(), Pattern::Series(tail).flatten_once()))
        }
        Pattern::Choice(_) => None,
        Pattern::Repeat { .. } => None,
        Pattern::Predicate { .. } => None,
        _ => Some((pat.clone(), Pattern::Empty)),
    }
}

pub fn unshift_all(choices: &[Pattern]) -> (Pattern, Pattern) {
    let mut series = Vec::new();
    let mut choices = choices.into_iter().cloned().collect::<Vec<Pattern>>();
    if choices.is_empty() {
        return (Pattern::Empty, Pattern::Empty);
    } else if choices.len() == 1 {
        return (choices[0].clone(), Pattern::Empty);
    }

'done:
    loop {
        let mut prefix = None;
        let mut suffixes = Vec::with_capacity(choices.len());
        for pat in &choices {
            let (head, tail) = match unshift(&pat) {
                Some(pair) => pair,
                None => break 'done,
            };
            if prefix.is_none() {
                prefix = Some(head);
            } else if prefix != Some(head.clone()) {
                break 'done;
            }
            suffixes.push(tail);
        }
        series.push(prefix.unwrap());
        choices = suffixes;
    }
    (
        Pattern::Series(series).flatten_once(),
        Pattern::Choice(choices).flatten_once(),
    )
}
