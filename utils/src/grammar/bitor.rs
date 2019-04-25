use crate::grammar::*;
use std::ops::BitOr;

impl<Err, L, R> BitOr<R> for Either<Err, L, R>
where
    Err: ParseError,
    L: PredictiveGrammar<Err>,
    R: PredictiveGrammar<Err>,
{
    type Output = Either<Err, Self, R>;
    fn bitor(self, rhs: R) -> Self::Output { self.or(rhs) }
}

impl<Err, L, R> BitOr<R> for EitherNode<Err, L, R>
where
    Err: ParseError,
    L: PredictiveGrammarNode<Err>,
    R: PredictiveGrammarNode<Err>,
{
    type Output = EitherNode<Err, Self, R>;
    fn bitor(self, rhs: R) -> Self::Output { self.or(rhs) }
}

impl<Err, R> BitOr<R> for Expect<Err>
where
    Err: ParseError,
    R: PredictiveGrammar<Err>,
{
    type Output = Either<Err, Self, R>;
    fn bitor(self, rhs: R) -> Self::Output { self.or(rhs) }
}

impl<Err, G, R> BitOr<R> for Committed<Err, G>
where
    Err: ParseError,
    G: PredictiveGrammarNode<Err>,
    R: PredictiveGrammar<Err>,
{
    type Output = Either<Err, Self, R>;
    fn bitor(self, rhs: R) -> Self::Output { self.or(rhs) }
}

impl<Err, G, R> BitOr<R> for Uncommitted<Err, G>
where
    Err: ParseError,
    G: PredictiveGrammar<Err>,
    R: PredictiveGrammarNode<Err>,
{
    type Output = EitherNode<Err, Self, R>;
    fn bitor(self, rhs: R) -> Self::Output { self.or(rhs) }
}