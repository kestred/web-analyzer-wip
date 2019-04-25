use crate::grammar::*;
use crate::parser::ParseError;
use crate::syntax_kind::ERROR;
use rowan::SyntaxKind;

// TODO: Docs
//
// - Allows more efficient parsing of LL(1)-like than arbitrary lookahead (TODO: Benchmark).
// - Can be used to generate a list of expected tokens
pub trait PredictiveGrammar<Err: ParseError = String>: Grammar<Err> {
    // TODO: docs
    fn predicate(&self) -> TokenSet;

    // TODO: docs
    fn or<R: PredictiveGrammar<Err>>(self, right: R) -> Either<Err, Self, R> where Self: Sized {
        Either {
            errtype: PhantomData,
            left: self,
            right,
        }
    }
}

// TODO: Docs
pub trait PredictiveGrammarNode<Err: ParseError = String>: GrammarNode<Err> {
    // TODO: docs
    fn predicate(&self) -> TokenSet;

    // TODO: docs
    fn or<R: PredictiveGrammarNode<Err>>(self, right: R) -> EitherNode<Err, Self, R> where Self: Sized {
        EitherNode {
            errtype: PhantomData,
            left: self,
            right,
        }
    }
}

/// Represents the return type of `grammar.or(kind)`.
pub struct Either<Err: ParseError, L: PredictiveGrammar<Err>, R: PredictiveGrammar<Err>> {
    errtype: PhantomData<Err>,
    left: L,
    right: R,
}
impl<Err, L, R> Grammar<Err> for Either<Err, L, R>
where
    Err: ParseError,
    L: PredictiveGrammar<Err>,
    R: PredictiveGrammar<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        let tok = p.current();
        let err_left = if self.left.predicate().contains(&tok) {
            match self.left.try_parse(p) {
                Ok(()) => return Outcome::Ok,
                Err(err) => Some(err),
            }
        } else {
            None
        };
        let err_right = if self.right.predicate().contains(&tok) {
            match self.right.try_parse(p) {
                Ok(()) => return Outcome::Ok,
                Err(err) => Some(err),
            }
        } else {
            None
        };
        let err = match (err_left, err_right) {
            (Some(left), _) => left,
            (None, Some(right)) => right,
            (None, None) => {
                Err::from(format!("expected one of {}",
                // TODO: Maybe collect into small vec to avoid allocating?
                // FIXME: Allow a language-specific to_debug_repr method to be configured
                    self.predicate().iter().map(|kind| {
                        crate::syntax_kind::default::as_debug_repr(*kind)
                            .map(|k| format!("{:?}", k.name))
                            .unwrap_or_else(|| format!("[{};", kind.0.to_string()))
                    }).collect::<Vec<_>>().join(",")
                ))
            }
        };
        p.error(err);
        return Outcome::Err;
    }
}

impl<Err, L, R> PredictiveGrammar<Err> for Either<Err, L, R>
where
    Err: ParseError,
    L: PredictiveGrammar<Err>,
    R: PredictiveGrammar<Err>,
{
    fn predicate(&self) -> TokenSet {
        let mut ts = self.left.predicate();
        for token in self.right.predicate().iter() {
            ts.insert(*token);
        }
        ts
    }
}

/// Represents the return type of `grammar_node.or(kind)`.
pub struct EitherNode<Err: ParseError, L: PredictiveGrammarNode<Err>, R: PredictiveGrammarNode<Err>> {
    errtype: PhantomData<Err>,
    left: L,
    right: R,
}
impl<Err, L, R> GrammarNode<Err> for EitherNode<Err, L, R>
where
    Err: ParseError,
    L: PredictiveGrammarNode<Err>,
    R: PredictiveGrammarNode<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> SyntaxKind {
        let tok = p.current();
        let err_left = if self.left.predicate().contains(&tok) {
            match self.left.try_parse(p) {
                Ok(kind) => return kind,
                Err(err) => Some(err),
            }
        } else {
            None
        };
        let err_right = if self.right.predicate().contains(&tok) {
            match self.right.try_parse(p) {
                Ok(kind) => return kind,
                Err(err) => Some(err),
            }
        } else {
            None
        };
        let err = match (err_left, err_right) {
            (Some(left), _) => left,
            (None, Some(right)) => right,
            (None, None) => {
                Err::from(format!("expected one of {}",
                // TODO: Maybe collect into small vec to avoid allocating?
                // FIXME: Allow a language-specific to_debug_repr method to be configured
                    self.predicate().iter().map(|kind| {
                        crate::syntax_kind::default::as_debug_repr(*kind)
                            .map(|k| format!("{:?}", k.name))
                            .unwrap_or_else(|| format!("[{};", kind.0.to_string()))
                    }).collect::<Vec<_>>().join(",")
                ))
            }
        };
        p.error(err);
        return ERROR;
    }
}
impl<Err, L, R> PredictiveGrammarNode<Err> for EitherNode<Err, L, R>
where
    Err: ParseError,
    L: PredictiveGrammarNode<Err>,
    R: PredictiveGrammarNode<Err>,
{
    fn predicate(&self) -> TokenSet {
        let mut ts = self.left.predicate();
        for token in self.right.predicate().iter() {
            ts.insert(*token);
        }
        ts
    }
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