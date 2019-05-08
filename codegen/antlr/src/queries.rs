use crate::ast::*;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use smol_str::SmolStr;
use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::collections::{HashMap as Map, BTreeSet as Set};
use std::rc::Rc;

pub struct Database {
    grammar: Grammar,
    topology: Graph<SmolStr, ()>,
    reference: Map<SmolStr, NodeIndex<u32>>,
    dictionary: Map<SmolStr, Rc<Rule>>,

    /// A map of tokensets to `const` declaration names and (whether the declaration is used)
    named_tokensets: RefCell<Map<Set<String>, (String, bool)>>,
    next_tokenset: Cell<u32>,

    /// A data structure storing memoized results
    memos: Memos,
}

impl Database {
    pub fn grammar(&self) -> &Grammar {
        &self.grammar
    }

    pub fn rule(&self, name: &str) -> &Rule {
        match self.dictionary.get(name).map(|r| r.as_ref()) {
            Some(rule) => rule,
            None => panic!("undefined rule `{}`", name),
        }
    }

    pub fn rule_index(&self, name: &str) -> NodeIndex<u32> {
        match self.reference.get(name).map(|r| *r) {
            Some(rule) => rule,
            None => panic!("undefined rule `{}`", name),
        }
    }

    pub fn tokenset_name(&self, tokenset: &Set<String>) -> String {
        let mut tokensets = self.named_tokensets.borrow_mut();
        if let Some((name, used)) = tokensets.get_mut(tokenset) {
            *used = true;
            return name.clone();
        }
        let next = self.next_tokenset.get();
        self.next_tokenset.set(next + 1);
        let name = format!("_TS{}", next);
        tokensets.insert(tokenset.clone(), (name.clone(), true));
        name
    }

    pub fn tokensets(&self) -> Vec<(String, Set<String>, bool)> {
        self.named_tokensets.clone().into_inner().into_iter().map(|(set, (name, used))| (name, set, used)).collect()
    }

    pub fn set_tokenset_name(&self, tokenset: Set<String>, name: String) {
        self.named_tokensets.borrow_mut()
            .entry(tokenset)
            .and_modify(|(orig, _)| *orig = name.clone())
            .or_insert((name, false));
    }

    pub fn is_left_recursive(&self, rule: &str) -> bool {
        self.is_left_corner(rule, rule)
    }

    pub fn is_directly_left_recursive(&self, rule: &str) -> bool {
        self.is_direct_left_corner(rule, rule)
    }

    // TODO: Decide whether I care
    // pub fn is_indirectly_left_recursive(&self, rule: &str) -> bool {
    //     !self.is_directly_left_recursive(rule) && self.is_left_recursive(rule)
    // }

    /// If `symbol` `is_direct_left_corner` of `rule` transitively
    pub fn is_left_corner(&self, rule: &str, symbol: &str) -> bool {
        self.is_left_corner_(rule, symbol, &mut Set::new())
    }

    fn is_left_corner_<'a>(&'a self, rule: &str, symbol: &str, visited: &mut Set<&'a str>) -> bool {
        if is_term(symbol) {
            return self.is_direct_left_corner(rule, symbol);
        }
        if self.is_direct_left_corner(rule, symbol) {
            return true;
        }
        for dep in self.topology.neighbors(self.rule_index(rule)) {
            let call = self.topology[dep].as_str();
            if !visited.contains(call) {
                visited.insert(call);
                if self.is_direct_left_corner(rule, call) {
                    if self.is_left_corner_(call, symbol, visited) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn is_left_corner_of(&self, pat: &Pattern, symbol: &str) -> bool {
        if is_term(symbol) {
            return self.is_direct_left_corner_of(pat, symbol);
        }
        let symbols = self.next_symbols_of(pat);
        if symbols.contains(symbol) {
            return true;
        }
        for rule in symbols.into_iter().filter(|sym| !is_term(sym)) {
            if self.is_left_corner(rule, symbol) {
                return true;
            }
        }
        return false;
    }

    /// If `symbol` appears on the left of any pattern in `rule`.
    pub fn is_direct_left_corner(&self, rule: &str, symbol: &str) -> bool {
        let rule = self.rule(rule);
        self.is_direct_left_corner_of(&rule.pattern, symbol)
    }

    pub fn is_direct_left_corner_of(&self, pat: &Pattern, symbol: &str) -> bool {
        self.next_symbols_of(&pat).contains(symbol)
    }

    pub fn is_possibly_empty(&self, pat: &Pattern) -> bool {
        match pat {
            Pattern::Empty => true,
            Pattern::Literal(_) => false,
            Pattern::Ident(ident) => {
                if is_term(ident) {
                    false
                } else if let Some(result) = self.memos.get_is_possibly_empty(&ident) {
                    result
                } else {
                    self.memos.set_is_possibly_empty(&ident, false);
                    let call = self.rule(&ident);
                    let result = self.is_possibly_empty(&call.pattern);
                    self.memos.set_is_possibly_empty(&ident, result);
                    result
                }
            }
            Pattern::Series(series) => series.iter().all(|pat| self.is_possibly_empty(pat)),
            Pattern::Choice(choice) => choice.iter().any(|pat| self.is_possibly_empty(pat)),
            Pattern::Repeat(pat, repeat) => match repeat {
                Repeat::ZeroOrOne => true,
                Repeat::ZeroOrMore => true,
                Repeat::OneOrMore => self.is_possibly_empty(pat)
            },
            Pattern::Precedence(nonterm, _) => {
                let call = self.rule(nonterm);
                self.is_possibly_empty(&call.pattern)
            },
            Pattern::Predicate(_, pat) => self.is_possibly_empty(pat),
            Pattern::Node(name, pat) => {
                let matches = self.is_possibly_empty(pat);
                assert!(!matches, "node `# {}` matches empty", name);
                matches
            }
            Pattern::NodeStart(pat) => self.is_possibly_empty(pat),
            Pattern::NodeComplete(_, pat) => self.is_possibly_empty(pat),
        }
    }

    pub fn is_next_term_ambiguous(&self, pat: &Pattern) -> bool {
        match pat {
            Pattern::Empty => false,
            Pattern::Literal(_) => false,
            Pattern::Ident(ident) => {
                if is_term(ident) {
                    false
                } else if let Some(result) = self.memos.get_is_next_term_ambiguous(&ident) {
                    result
                } else {
                    self.memos.set_is_next_term_ambiguous(&ident, false);
                    let call = self.rule(&ident);
                    let result = self.is_next_term_ambiguous(&call.pattern);
                    self.memos.set_is_next_term_ambiguous(&ident, result);
                    result
                }
            }
            Pattern::Series(series) => {
                let mut prev_ts = Set::new();
                for pat in series {
                    let ts = self.next_terms_of(pat);
                    if !ts.is_disjoint(&prev_ts) {
                        return true;
                    }
                    if !self.is_possibly_empty(pat) {
                        break;
                    }
                    prev_ts.extend(ts);
                }
                false
            }
            Pattern::Choice(choice) => {
                let mut prev_ts = Set::new();
                for pat in choice {
                    let ts = self.next_terms_of(pat);
                    if !ts.is_disjoint(&prev_ts) {
                        return true;
                    }
                    prev_ts.extend(ts);
                }
                false
            }
            Pattern::Repeat(pat, _) => self.is_next_term_ambiguous(pat),
            Pattern::Precedence(nonterm, _) => {
                let call = self.rule(nonterm);
                self.is_next_term_ambiguous(&call.pattern)
            },
            Pattern::Predicate(_, pat) => self.is_next_term_ambiguous(pat),
            Pattern::Node(_, pat) => self.is_next_term_ambiguous(pat),
            Pattern::NodeStart(pat) => self.is_next_term_ambiguous(pat),
            Pattern::NodeComplete(_, pat) => self.is_next_term_ambiguous(pat),
        }
    }

    pub fn next_symbols_of<'a>(&'a self, pat: &'a Pattern) -> Set<&'a str> {
        match pat {
            Pattern::Empty => Set::new(),
            Pattern::Literal(lit) => {
                let mut set = Set::new();
                set.insert(to_token_of_literal(lit));
                set
            }
            Pattern::Ident(ident) => {
                let mut set = Set::new();
                set.insert(ident.as_str());
                set
            }

            // Get tokenset from just the all patterns upto the first non-optional pattern in a series
            Pattern::Series(series) => {
                let mut set = Set::new();
                for pat in series.iter().next() {
                    set.extend(self.next_symbols_of(pat));
                }
                set
            }

            // Combine tokenset from all patterns in a choice
            Pattern::Choice(choice) => {
                let mut set = Set::new();
                for pat in choice {
                    set.extend(self.next_symbols_of(pat));
                }
                set
            }

            Pattern::Repeat(pat, _) => self.next_symbols_of(pat),

            // Verify pred and expr both have the same `next_tokenset`
            Pattern::Precedence(_, _) => Set::new(),

            // Verify pred and expr both have the same `next_tokenset`
            Pattern::Predicate(_, pat) => self.next_symbols_of(pat),

            // Simple recursion
            Pattern::Node(_, pat) => self.next_symbols_of(pat),
            Pattern::NodeStart(pat) => self.next_symbols_of(pat),
            Pattern::NodeComplete(_, pat) => self.next_symbols_of(pat),
        }
    }

    pub fn next_terms_of<'a>(&'a self, pat: &'a Pattern) -> Set<String> {
        self.next_predicates_of(pat).into_iter().map(|t| t.token.into()).collect()
    }

    pub fn next_predicates_of(&self, pat: &Pattern) -> Set<QualifiedTerm> {
        match pat {
            Pattern::Empty => Set::new(),
            Pattern::Literal(lit) => {
                let mut set = Set::new();
                set.insert(QualifiedTerm::new(to_token_of_literal(lit)));
                set
            }
            Pattern::Ident(ident) => {
                if is_term(ident) {
                    let mut set = Set::new();
                    set.insert(QualifiedTerm::new(&ident));
                    set
                } else if let Some(result) = self.memos.get_next_predicates_of(&ident) {
                    result
                } else {
                    self.memos.set_next_predicates_of(&ident, Set::new());
                    let call = self.rule(&ident);
                    let result = self.next_predicates_of(&call.pattern);
                    self.memos.set_next_predicates_of(&ident, result.clone());
                    result
                }
            }

            // Get tokenset from just the all patterns upto the first non-optional pattern in a series
            Pattern::Series(series) => {
                let mut set = Set::new();
                for pat in series {
                    set.extend(self.next_predicates_of(pat));
                    if !self.is_possibly_empty(pat) {
                        break;
                    }
                }
                set
            }

            // Combine tokenset from all patterns in a choice
            Pattern::Choice(choice) => {
                let mut set = Set::new();
                for pat in choice {
                    set.extend(self.next_predicates_of(pat));
                }
                set
            }

            Pattern::Repeat(pat, _) => self.next_predicates_of(pat),

            // Verify pred and expr both have the same `next_terms`
            Pattern::Predicate(pred, pat) => {
                self.next_predicates_of(pat)
                    .into_iter()
                    .map(|term| term.qualified_by(pred.clone()))
                    .collect()
            }

            Pattern::Precedence(nonterm, _) => {
                if let Some(result) = self.memos.get_next_predicates_of(&nonterm) {
                    result
                } else {
                    self.memos.set_next_predicates_of(&nonterm, Set::new());
                    let call = self.rule(&nonterm);
                    let result = self.next_predicates_of(&call.pattern);
                    self.memos.set_next_predicates_of(&nonterm, result.clone());
                    result
                }
            }

            // Simple recursion
            Pattern::Node(_, pat) => self.next_predicates_of(pat),
            Pattern::NodeStart(pat) => self.next_predicates_of(pat),
            Pattern::NodeComplete(_, pat) => self.next_predicates_of(pat),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedTerm {
    pub token: SmolStr, // TODO: make PR to SmolStr to impl ord?
    pub predicate: PredicateExpression,
}

impl QualifiedTerm {
    pub fn new(token: &str) -> QualifiedTerm {
        QualifiedTerm {
            token: token.into(),
            predicate: PredicateExpression::Empty,
        }
    }

    pub fn unqualified(self) -> QualifiedTerm {
        QualifiedTerm {
            token: self.token,
            predicate: PredicateExpression::Empty,
        }
    }

    pub fn qualified_by(self, pred: PredicateExpression) -> QualifiedTerm {
        QualifiedTerm {
            token: self.token,
            predicate: match self.predicate {
                PredicateExpression::Empty => pred,
                _ => PredicateExpression::Binary {
                    left: Box::new(self.predicate),
                    oper: "&&",
                    right: Box::new(pred),
                },
            },
        }
    }

    pub fn is_unqualified(&self) -> bool {
        match self.predicate {
            PredicateExpression::Empty => true,
            _ => false,
        }
    }
}

impl Ord for QualifiedTerm {
    fn cmp(&self, other: &QualifiedTerm) -> Ordering {
        self.predicate.cmp(&other.predicate).then(self.token.cmp(&other.token))
    }
}

impl PartialOrd for QualifiedTerm {
    fn partial_cmp(&self, other: &QualifiedTerm) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Grammar> for Database {
    fn from(grammar: Grammar) -> Database {
        let mut topology = Graph::new();
        let mut reference = Map::new();
        let mut dictionary = Map::new();
        for rule in &grammar.rules {
            let refn = topology.add_node(rule.name.as_str().into());
            reference.insert(rule.name.as_str().into(), refn);
            dictionary.insert(rule.name.as_str().into(), Rc::clone(rule));
        }
        for rule in &grammar.rules {
            let left = reference.get(rule.name.as_str()).unwrap();
            for dep in non_terminals(&rule.pattern) {
                let right = match reference.get(dep.as_str()) {
                    Some(idx) => idx,
                    None => panic!("rule `{}` is not defined", dep),
                };
                topology.add_edge(*left, *right, ());
            }
        }
        Database {
            grammar,
            topology,
            reference,
            dictionary,
            named_tokensets: RefCell::new(Map::new()),
            next_tokenset: Cell::new(0),
            memos: Memos::new(),
        }
    }
}

struct Memos {
    /// Memoized results for `is_possibly_empty`
    is_possibly_empty: RefCell<Map<SmolStr, bool>>,
    /// Memoized results for `is_next_term_ambiguous`
    is_next_term_ambiguous: RefCell<Map<SmolStr, bool>>,
    /// Memoized results for `next_predicates_of`
    next_predicates_of: RefCell<Map<SmolStr, Set<QualifiedTerm>>>,
}

impl Memos {
    fn new() -> Memos {
        Memos {
            is_possibly_empty: RefCell::new(Map::new()),
            is_next_term_ambiguous: RefCell::new(Map::new()),
            next_predicates_of: RefCell::new(Map::new()),
        }
    }

    fn get_is_possibly_empty(&self, key: &str) -> Option<bool> {
        self.is_possibly_empty.borrow().get(key).cloned()
    }

    fn set_is_possibly_empty(&self, key: &str, result: bool) {
        self.is_possibly_empty.borrow_mut().insert(key.into(), result);
    }

    fn get_is_next_term_ambiguous(&self, key: &str) -> Option<bool> {
        self.is_next_term_ambiguous.borrow().get(key).cloned()
    }

    fn set_is_next_term_ambiguous(&self, key: &str, result: bool) {
        self.is_next_term_ambiguous.borrow_mut().insert(key.into(), result);
    }

    fn get_next_predicates_of(&self, key: &str) -> Option<Set<QualifiedTerm>> {
        self.next_predicates_of.borrow().get(key).cloned()
    }

    fn set_next_predicates_of(&self, key: &str, result: Set<QualifiedTerm>) {
        self.next_predicates_of.borrow_mut().insert(key.into(), result);
    }
}

fn non_terminals(p: &Pattern) -> Set<String> {
    let mut set = Set::new();
    collect_non_terminals(&mut set, &p);
    set
}

fn collect_non_terminals(set: &mut Set<String>, p: &Pattern) {
    match p {
        Pattern::Ident(ident) if !is_term(ident) => {
            set.insert(ident.as_str().into());
        }
        Pattern::Series(series) => {
            for pat in series {
                collect_non_terminals(set, pat);
            }
        }
        Pattern::Choice(choice) => {
            for pat in choice {
                collect_non_terminals(set, pat);
            }
        }
        Pattern::Repeat(pat, _) => collect_non_terminals(set, pat),
        Pattern::Predicate(_, pat) => collect_non_terminals(set, pat),
        Pattern::Node(_, pat) => collect_non_terminals(set, pat),
        _ => (),
    }
}

pub fn to_token_of_literal(lit: &str) -> &'static str {
    match lit {
        "{" => "L_CURLY",
        "}" => "R_CURLY",
        "(" => "L_PAREN",
        ")" => "R_PAREN",
        "[" => "L_SQUARE",
        "]" => "R_SQUARE",
        "<" => "L_ANGLE",
        ">" => "R_ANGLE",
        ";" => "SEMICOLON",
        "," => "COMMA",
        ":" => "COLON",
        "::" => "COLONCOLON",
        "." => "DOT",
        ".." => "DOTDOT",
        "..." => "DOTDOTDOT",
        "++" => "INCREMENT",
        "--" => "DECREMENT",
        "<<" => "SHL",
        "<<=" => "SHL_EQ",
        ">>" => "SHR",
        ">>=" => "SHR_EQ",
        ">>>" => "SHU",
        ">>>=" => "SHU_EQ",
        "&&" => "AND",
        "||" => "OR",
        "=" => "EQ",
        "==" => "EQEQ",
        "===" => "EQEQEQ",
        "!" => "BANG",
        "!=" => "BANG_EQ",
        "!==" => "BANG_EQEQ",
        ">=" => "GT_EQ",
        "<=" => "LT_EQ",
        "*" => "ASTERISK",
        "*=" => "ASTERISK_EQ",
        "/" => "SLASH",
        "/=" => "SLASH_EQ",
        "%" => "PERCENT",
        "%=" => "PERCENT_EQ",
        "+" => "PLUS",
        "+=" => "PLUS_EQ",
        "-" => "MINUS",
        "-=" => "MINUS_EQ",
        "&" => "AMPERSAND",
        "&=" => "AMPERSAND_EQ",
        "|" => "PIPE",
        "|=" => "PIPE_EQ",
        "^" => "CARET",
        "^=" => "CARET_EQ",
        "~" => "TILDE",
        "?" => "QUESTION",
        "->" => "THIN_ARROW",
        "=>" => "FAT_ARROW",
        "<!" => "L_ANGLE_BANG",
        "</" => "L_ANGLE_SLASH",
        "/>" => "SLASH_R_ANGLE",
        _ => panic!("unhandled literal {}", lit),
    }
}
