use crate::grammar::*;
use crate::parser::ParseError;

// TODO: Docs
//
// - Allows more efficient parsing of LL(1)-like than arbitrary lookahead (TODO: Benchmark).
// - Can be used to generate a list of expected tokens
pub trait PredictiveGrammar<Err: ParseError = String>: Grammar<Err> {
    fn predicate(&self) -> TokenSet;
}

pub trait PredictiveGrammarNode<Err: ParseError = String>: GrammarNode<Err> {
    fn predicate(&self) -> TokenSet;
}

impl<Err> PredictiveGrammar<Err> for Expect<Err>
where
    Err: ParseError,
{
    fn predicate(&self) -> TokenSet {
        let mut ts = TokenSet::new();
        ts.insert(self.kind);
        ts
    }
}

impl<Err, G> PredictiveGrammar<Err> for Committed<Err, G>
where
    Err: ParseError,
    G: PredictiveGrammarNode<Err>,
{
    fn predicate(&self) -> TokenSet {
      self.grammar.predicate()
    }
}

impl<Err, G> PredictiveGrammarNode<Err> for Uncommitted<Err, G>
where
    Err: ParseError,
    G: PredictiveGrammar<Err>,
{
    fn predicate(&self) -> TokenSet {
      self.grammar.predicate()
    }
}


impl<Err, O> PredictiveGrammar<Err> for Many1<Err, O>
where
    Err: ParseError,
    O: PredictiveGrammar<Err>,
{
    fn predicate(&self) -> TokenSet {
      self.once.predicate()
    }
}


impl<Err, O, S> PredictiveGrammar<Err> for SepBy1<Err, O, S>
where
    Err: ParseError,
    O: PredictiveGrammar<Err>,
    S: PredictiveGrammar<Err>,
{
    fn predicate(&self) -> TokenSet {
      self.once.predicate()
    }
}

impl<Err, A> PredictiveGrammar<Err> for (A,)
where
    Err: ParseError,
    A: PredictiveGrammar<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B> PredictiveGrammar<Err> for (A, B)
where
    Err: ParseError,
    A: PredictiveGrammar<Err>,
    B: Grammar<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C> PredictiveGrammar<Err> for (A, B, C)
where
    Err: ParseError,
    A: PredictiveGrammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C, D> PredictiveGrammar<Err> for (A, B, C, D)
where
    Err: ParseError,
    A: PredictiveGrammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
    D: Grammar<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C, D, E> PredictiveGrammar<Err> for (A, B, C, D, E)
where
    Err: ParseError,
    A: PredictiveGrammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
    D: Grammar<Err>,
    E: Grammar<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C, D, E, F> PredictiveGrammar<Err> for (A, B, C, D, E, F)
where
    Err: ParseError,
    A: PredictiveGrammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
    D: Grammar<Err>,
    E: Grammar<Err>,
    F: Grammar<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C, D, E, F, G> PredictiveGrammar<Err> for (A, B, C, D, E, F, G)
where
    Err: ParseError,
    A: PredictiveGrammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
    D: Grammar<Err>,
    E: Grammar<Err>,
    F: Grammar<Err>,
    G: Grammar<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}

impl<Err, A, B, C, D, E, F, G, H> PredictiveGrammar<Err> for (A, B, C, D, E, F, G, H)
where
    Err: ParseError,
    A: PredictiveGrammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
    D: Grammar<Err>,
    E: Grammar<Err>,
    F: Grammar<Err>,
    G: Grammar<Err>,
    H: Grammar<Err>,
{
    fn predicate(&self) -> TokenSet {
        self.0.predicate()
    }
}