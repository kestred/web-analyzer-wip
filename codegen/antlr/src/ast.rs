use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Grammar {
    pub name: String,
    pub rules: Vec<Rc<Rule>>,
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub pattern: Pattern,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub enum Attribute {
    Word(String), // e.g. `syn::Word`
    Group(String, Vec<Attribute>), // e.g. `syn::Meta::List`
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Empty,

    /// A non-terminal or terminal (e.g. `L_PAREN`)
    Ident(String),

    /// A literal string (e.g. `"keyword"`)
    Literal(String),

    /// Multiple patterns matched in a row (e.g. `L_PAREN IDENT R_PAREN`)
    Series(Vec<Pattern>),

    /// A choice between multiple patterns (e.g. `"public" | "private"`)
    Choice(Vec<Pattern>),

    /// A repetition of a pattern group (e.g. `(IDENT COLON type_expr COMMA)+`)
    Repeat(Box<Pattern>, Repeat),

    /// A pre-condition to check before parsing a tree, typically
    /// to resolve syntactic ambiguity; e.g. the `(...) => ...` in:
    ///
    /// >    lhs_expression:
    /// >        ("new" expression) => newExpression
    /// >        | expression
    /// >        ;
    ///
    Predicate(PredicateExpression, Box<Pattern>),

    /// A node to be committed to the source tree
    Node(String, Box<Pattern>),

    // N.B. not present in grammar
    Precedence(String, u32),
    NodeStart(Box<Pattern>),
    NodeComplete(String, Box<Pattern>),
}

impl Pattern {
    pub fn is_empty(&self) -> bool {
        match self {
            Pattern::Empty => true,
            _ => false,
        }
    }

    pub fn is_term(&self) -> bool {
        match self {
            Pattern::Ident(ident) if is_term(ident) => true,
            Pattern::Literal(_) => true,
            _ => false,
        }
    }

    pub fn is_enum(&self) -> bool {
        match self {
            Pattern::Choice(choices) => choices.iter().all(|p| p.is_term() || p.is_enum()),
            _ => false,
        }
    }

    pub fn is_nonterm(&self) -> bool {
        match self {
            Pattern::Ident(ident) if !is_term(ident) => true,
            _ => false,
        }
    }

    /// Unwraps the top-level of the pattern tree, if possible.
    pub fn flatten_once(mut self) -> Pattern {
        match self {
            Pattern::Choice(ref mut p) if p.len() == 0 => Pattern::Empty,
            Pattern::Choice(ref mut p) if p.len() == 1 => p.drain(0..1).next().unwrap(),
            Pattern::Series(ref mut p) if p.len() == 0 => Pattern::Empty,
            Pattern::Series(ref mut p) if p.len() == 1 => p.drain(0..1).next().unwrap(),
            Pattern::Repeat(ref p, _) if p.is_empty() => Pattern::Empty,
            _ => self,
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pattern::Empty => Ok(()),
            Pattern::Ident(ident) => write!(f, "{}", ident),
            Pattern::Literal(lit) => write!(f, "{}", lit),
            Pattern::Series(series) => write!(f, "{}", series.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" ")),
            Pattern::Choice(choices) => write!(f, "({})", choices.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" | ")),
            Pattern::Repeat(pattern, repeat) => match repeat {
                Repeat::ZeroOrOne => write!(f, "({})?", pattern),
                Repeat::ZeroOrMore => write!(f, "({})*", pattern),
                Repeat::OneOrMore => write!(f, "({})+", pattern),
            },
            Pattern::Predicate(_expr, tail) => write!(f, "{{ <predicate> }}? {}", tail),
            Pattern::Node(kind, pattern) => write!(f, "{} #{}", pattern, kind),
            Pattern::NodeStart(pattern) => write!(f, "{}", pattern),
            Pattern::NodeComplete(kind, pattern) => write!(f, "{} #{}", pattern, kind),
            Pattern::Precedence(nonterm, prec) => write!(f, "{}[{}]", nonterm, prec),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Repeat {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PredicateExpression {
    Empty,
    Call { method: String, args: Vec<String> },
    Unary { oper: char, expr: Box<PredicateExpression> },
    Binary { left: Box<PredicateExpression>, oper: &'static str, right: Box<PredicateExpression> },
}

pub fn is_term(ident: &str) -> bool {
    ident.chars().all(|c| !char::is_alphabetic(c) || char::is_uppercase(c))
}
