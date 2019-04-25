use crate::grammar::*;
use crate::parser::ParseError;

// TODO: Docs
//
// - Allows more efficient parsing of LL(1)-like than arbitrary lookahead (TODO: Benchmark).
// - Can be used to generate a list of expected tokens
pub trait Predictive<Err: ParseError>: GrammarLike<Err> {
    fn predicate(&self) -> TokenSet;
}

impl<Err> Predictive<Err> for Expect<Err>
where
    Err: ParseError,
{
    fn predicate(&self) -> TokenSet {
        let mut ts = TokenSet::new();
        ts.insert(self.kind);
        ts
    }
}

impl<Err, O> Predictive<Err> for Many1<Err, O>
where
    Err: ParseError,
    O: Predictive<Err>,
{
    fn predicate(&self) -> TokenSet {
      self.once.predicate()
    }
}


impl<Err, O, S> Predictive<Err> for SepBy1<Err, O, S>
where
    Err: ParseError,
    O: Predictive<Err>,
    S: Predictive<Err>,
{
    fn predicate(&self) -> TokenSet {
      self.once.predicate()
    }
}

impl<Err, A> Predictive<Err> for (A,)
where
    Err: ParseError,
    A: Predictive<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B> Predictive<Err> for (A, B)
where
    Err: ParseError,
    A: Predictive<Err>,
    B: GrammarLike<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C> Predictive<Err> for (A, B, C)
where
    Err: ParseError,
    A: Predictive<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C, D> Predictive<Err> for (A, B, C, D)
where
    Err: ParseError,
    A: Predictive<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
    D: GrammarLike<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C, D, E> Predictive<Err> for (A, B, C, D, E)
where
    Err: ParseError,
    A: Predictive<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
    D: GrammarLike<Err>,
    E: GrammarLike<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C, D, E, F> Predictive<Err> for (A, B, C, D, E, F)
where
    Err: ParseError,
    A: Predictive<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
    D: GrammarLike<Err>,
    E: GrammarLike<Err>,
    F: GrammarLike<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C, D, E, F, G> Predictive<Err> for (A, B, C, D, E, F, G)
where
    Err: ParseError,
    A: Predictive<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
    D: GrammarLike<Err>,
    E: GrammarLike<Err>,
    F: GrammarLike<Err>,
    G: GrammarLike<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C, D, E, F, G, H> Predictive<Err> for (A, B, C, D, E, F, G, H)
where
    Err: ParseError,
    A: Predictive<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
    D: GrammarLike<Err>,
    E: GrammarLike<Err>,
    F: GrammarLike<Err>,
    G: GrammarLike<Err>,
    H: GrammarLike<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}