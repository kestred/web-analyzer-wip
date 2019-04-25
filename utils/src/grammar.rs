mod impls;
mod outcome;
mod predictive;
mod token_set;

use crate::parser::{Parser, ParseError};
use crate::parse_ok;
use rowan::SyntaxKind;
use std::marker::PhantomData;

pub use self::outcome::Outcome;
pub use self::predictive::Predictive;
pub use self::token_set::TokenSet;

// TODO: Docs
pub trait Grammar<Err: ParseError = String> {
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
    fn commit(self) -> Node<Err, Self> where Self: Sized {
        Node {
            errtype: PhantomData,
            grammar: self,
        }
    }
}

// TODO: Docs
pub trait GrammarLike<Err: ParseError = String> {
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
    fn commit(self, kind: SyntaxKind) -> Node<Err, NodeLike<Err, Self>> where Self: Sized {
        self.is(kind).commit()
    }

    // TODO: Docs
    fn then<G: GrammarLike<Err>>(self, next: G) -> (Self, G) where Self: Sized {
       (self, next)
    }

    // TODO: Docs
    fn is(self, kind: SyntaxKind) -> NodeLike<Err, Self> where Self: Sized {
        NodeLike {
            errtype: PhantomData,
            grammar: self,
            kind
        }
    }
}

/// Represents the return type of `grammar.commit(kind)`.
pub struct Node<Err: ParseError, G: Grammar<Err>> {
    errtype: PhantomData<Err>,
    grammar: G,
}
impl<Err, G> GrammarLike<Err> for Node<Err, G>
where
    Err: ParseError,
    G: Grammar<Err>
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        p.eval(&self.grammar).into()
    }
}

/// Represents the return type of `grammar.is(kind)`.
pub struct NodeLike<Err: ParseError, G: GrammarLike<Err>> {
    errtype: PhantomData<Err>,
    grammar: G,
    kind: SyntaxKind,
}
impl<Err, G> Grammar<Err> for NodeLike<Err, G>
where
    Err: ParseError,
    G: GrammarLike<Err>
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
impl<Err> GrammarLike<Err> for Expect<Err>
where
    Err: ParseError,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        if !p.eat(self.kind) {
            p.error(format!("expected {:?}", self.kind));
            Outcome::Err
        } else {
            Outcome::Ok
        }
    }
}

/// See `Grammar::try_parse`.
pub fn attempt<Err: ParseError, O: GrammarLike<Err>>(once: O) -> Attempt<Err, O> {
    Attempt {
        errtype: PhantomData,
        once
    }
}

/// Represents the return type of `attempt(grammar)`.
pub struct Attempt<Err: ParseError, O: GrammarLike<Err>> {
    errtype: PhantomData<Err>,
    once: O,
}
impl<Err, O> GrammarLike<Err> for Attempt<Err, O>
where
    Err: ParseError,
    O: GrammarLike<Err>,
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
pub fn optional<Err: ParseError, O: Predictive<Err>>(once: O) -> Optional<Err, O> {
    Optional {
        errtype: PhantomData,
        once
    }
}
pub struct Optional<Err: ParseError, O: Predictive<Err>> {
    errtype: PhantomData<Err>,
    once: O,
}
impl<Err, O> GrammarLike<Err> for Optional<Err, O>
where
    Err: ParseError,
    O: Predictive<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        if self.once.predicate().contains(&p.current()) {
            self.once.try_parse(p).ok();
        }
        Outcome::Ok
    }
}

// TODO: Docs
pub fn many1<Err: ParseError, O: GrammarLike<Err>>(once: O) -> Many1<Err, O> {
    Many1 {
        errtype: PhantomData,
        once
    }
}

/// Represents the return type of `many1(grammar)`.
pub struct Many1<Err: ParseError, O: GrammarLike<Err>> {
    errtype: PhantomData<Err>,
    once: O,
}
impl<Err, O> Many1<Err, O>
where
    Err: ParseError,
    O: GrammarLike<Err>,
{
    // TODO: Docs
    pub fn sep_by<S: Predictive<Err>>(self, sep: S) -> SepBy1<Err, O, S> {
        SepBy1 {
            errtype: PhantomData,
            once: self.once,
            sep,
        }
    }
}
impl<Err, O> GrammarLike<Err> for Many1<Err, O>
where
    Err: ParseError,
    O: GrammarLike<Err>,
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
pub struct SepBy1<Err: ParseError, O: GrammarLike<Err>, S: Predictive<Err>> {
    errtype: PhantomData<Err>,
    once: O,
    sep: S,
}
impl<Err, O, S> GrammarLike<Err> for SepBy1<Err, O, S>
where
    Err: ParseError,
    O: GrammarLike<Err>,
    S: Predictive<Err>,
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
