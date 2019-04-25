mod bitor;
mod builtin;
mod outcome;
mod predictive;
mod token_set;

use crate::parser::{Parser, ParseError};
use crate::parse_ok;
use rowan::SyntaxKind;
use std::marker::PhantomData;

pub use self::outcome::Outcome;
pub use self::predictive::*;
pub use self::token_set::TokenSet;

// TODO: Docs
pub trait Grammar<Err: ParseError = String> {
    // TODO: Docs
    #[must_use]
    fn parse(&self, p: &mut Parser<Err>) -> Outcome;

    // TODO: Docs
    //
    // N.B. optional parsing via arbitrary backtracking;
    //      probably use `optional` instead when possible
    //      to avoid unnecessary backtracking (TODO: Benchmark)
    //
    // Doesn't emit errors nor consume tokens if parsing fails.
    fn try_parse(&self, p: &mut Parser<Err>) -> Result<(), Err> {
        let start = p.checkpoint();
        self.parse(p).ignore();
        p.commit(start)
    }

    // TODO: Docs
    fn into_boxed<'a>(self) -> Boxed<'a, Err> where Self: 'a + Sized {
        Boxed { grammar: Box::new(self) }
    }

    // TODO: Docs
    fn is(self, kind: SyntaxKind) -> Uncommitted<Err, Self> where Self: Sized {
        Uncommitted {
            errtype: PhantomData,
            grammar: self,
            kind
        }
    }

    // TODO: Docs
    fn commit(self, kind: SyntaxKind) -> Committed<Err, Uncommitted<Err, Self>> where Self: Sized {
        self.is(kind).commit()
    }

    // TODO: Docs
    fn then<G: Grammar<Err>>(self, next: G) -> (Self, G) where Self: Sized {
       (self, next)
    }
}

// TODO: Docs
pub trait GrammarNode<Err: ParseError = String> {
    fn parse(&self, p: &mut Parser<Err>) -> SyntaxKind;

    // TODO: Docs
    //
    // N.B. optional parsing via arbitrary backtracking;
    //      probably use `optional` instead when possible
    //      to avoid unnecessary backtracking (TODO: Benchmark)
    //
    // Doesn't emit errors nor consume tokens if parsing fails.
    fn try_parse(&self, p: &mut Parser<Err>) -> Result<SyntaxKind, Err> {
        let start = p.checkpoint();
        let kind = self.parse(p);
        p.commit(start)?;
        Ok(kind)
    }

    // TODO: Docs
    fn into_boxed<'a>(self) -> BoxedNode<'a, Err> where Self: 'a + Sized {
        BoxedNode { grammar: Box::new(self) }
    }

    // TODO: Docs
    fn commit(self) -> Committed<Err, Self> where Self: Sized {
        Committed {
            errtype: PhantomData,
            grammar: self,
        }
    }
}

/// Represents the return type of `grammar.into_boxed()`.
pub struct Boxed<'a, Err: ParseError> {
    grammar: Box<dyn Grammar<Err> + 'a>,
}
impl<'a, Err> Grammar<Err> for Boxed<'a, Err>
where
    Err: ParseError,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        self.grammar.parse(p)
    }
}

/// Represents the return type of `grammar_node.into_boxed()`.
pub struct BoxedNode<'a, Err: ParseError> {
    grammar: Box<dyn GrammarNode<Err> + 'a>,
}
impl<'a, Err> GrammarNode<Err> for BoxedNode<'a, Err>
where
    Err: ParseError,
{
    fn parse(&self, p: &mut Parser<Err>) -> SyntaxKind {
        self.grammar.parse(p)
    }
}

/// Represents the return type of `grammar_node.commit()`.
pub struct Committed<Err: ParseError, G: GrammarNode<Err>> {
    errtype: PhantomData<Err>,
    grammar: G,
}
impl<Err, G> Grammar<Err> for Committed<Err, G>
where
    Err: ParseError,
    G: GrammarNode<Err>
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        p.eval(&self.grammar).into()
    }
}

/// Represents the return type of `grammar.is(kind)`.
pub struct Uncommitted<Err: ParseError, G: Grammar<Err>> {
    errtype: PhantomData<Err>,
    grammar: G,
    kind: SyntaxKind,
}
impl<Err, G> GrammarNode<Err> for Uncommitted<Err, G>
where
    Err: ParseError,
    G: Grammar<Err>
{
    fn parse(&self, p: &mut Parser<Err>) -> SyntaxKind {
        self.grammar.parse(p).map(self.kind)
    }
}

// TODO: Docs
pub fn token<Err: ParseError>(kind: SyntaxKind) -> Expect<Err> {
    Expect {
        errtype: PhantomData,
        kind
    }
}

/// Represents the return type of `token(kind)`.
pub struct Expect<Err: ParseError> {
    errtype: PhantomData<Err>,
    kind: SyntaxKind,
}
impl<Err> Grammar<Err> for Expect<Err>
where
    Err: ParseError,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        if !p.eat(self.kind) {
            // FIXME: Allow a language-specific to_debug_repr method to be configured
            p.error(format!("expected {}",
                crate::syntax_kind::default::as_debug_repr(self.kind)
                    .map(|k| format!("{:?}", k.name))
                    .unwrap_or_else(|| format!("[{};", self.kind.0.to_string()))
            ));
            Outcome::Err
        } else {
            Outcome::Ok
        }
    }
}

/// See `GrammarNode::try_parse`.
pub fn attempt<Err: ParseError, O: Grammar<Err>>(once: O) -> Attempt<Err, O> {
    Attempt {
        errtype: PhantomData,
        once
    }
}

/// Represents the return type of `attempt(grammar)`.
pub struct Attempt<Err: ParseError, O: Grammar<Err>> {
    errtype: PhantomData<Err>,
    once: O,
}
impl<Err, O> Grammar<Err> for Attempt<Err, O>
where
    Err: ParseError,
    O: Grammar<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        self.once.try_parse(p).ok();
        Outcome::Ok
    }
}

// TODO: Docs
//
// N.B. optional parsing that _tries_ to avoid backtracking via lookahead
//
// Doesn't emit errors nor consume tokens if parsing fails.
pub fn optional<Err: ParseError, O: PredictiveGrammar<Err>>(once: O) -> Optional<Err, O> {
    Optional {
        errtype: PhantomData,
        once
    }
}
pub struct Optional<Err: ParseError, O: PredictiveGrammar<Err>> {
    errtype: PhantomData<Err>,
    once: O,
}
impl<Err, O> Grammar<Err> for Optional<Err, O>
where
    Err: ParseError,
    O: PredictiveGrammar<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        if self.once.predicate().contains(&p.current()) {
            self.once.try_parse(p).ok();
        }
        Outcome::Ok
    }
}

// TODO: Docs
pub fn many1<Err: ParseError, O: Grammar<Err>>(once: O) -> Many1<Err, O> {
    Many1 {
        errtype: PhantomData,
        once
    }
}

/// Represents the return type of `many1(grammar)`.
pub struct Many1<Err: ParseError, O: Grammar<Err>> {
    errtype: PhantomData<Err>,
    once: O,
}
impl<Err, O> Many1<Err, O>
where
    Err: ParseError,
    O: Grammar<Err>,
{
    // TODO: Docs
    pub fn sep_by<S: PredictiveGrammar<Err>>(self, sep: S) -> SepBy1<Err, O, S> {
        SepBy1 {
            errtype: PhantomData,
            once: self.once,
            sep,
        }
    }
}
impl<Err, O> Grammar<Err> for Many1<Err, O>
where
    Err: ParseError,
    O: Grammar<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.once.parse(p));
        while let Ok(()) = self.once.try_parse(p) {
            // noop
        }
        Outcome::Ok
    }
}

/// Represents the return type of `many1(grammar).sep_by(sep)`.
pub struct SepBy1<Err: ParseError, O: Grammar<Err>, S: PredictiveGrammar<Err>> {
    errtype: PhantomData<Err>,
    once: O,
    sep: S,
}
impl<Err, O, S> Grammar<Err> for SepBy1<Err, O, S>
where
    Err: ParseError,
    O: Grammar<Err>,
    S: PredictiveGrammar<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.once.parse(p));
        while self.sep.predicate().contains(&p.current()) {
            match self.sep.try_parse(p) {
                Ok(()) => parse_ok!(self.once.parse(p)),
                Err(_) => break,
            }
        }
        Outcome::Ok
    }
}
