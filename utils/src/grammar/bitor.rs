use crate::grammar::*;
use std::ops::BitOr;

impl<Err, Rhs> BitOr<Rhs> for Never<Err>
where
    Rhs: PredictiveGrammar<Err>,
    Err: ParseError,
{
    type Output = Either<Err, Self, Rhs>;
    fn bitor(self, rhs: Rhs) -> Self::Output { self.or(rhs) }
}

impl<Err, Rhs, L, R> BitOr<Rhs> for Either<Err, L, R>
where
    Err: ParseError,
    Rhs: PredictiveGrammar<Err>,
    L: PredictiveGrammar<Err>,
    R: PredictiveGrammar<Err>,
{
    type Output = Either<Err, Self, Rhs>;
    fn bitor(self, rhs: Rhs) -> Self::Output { self.or(rhs) }
}

impl<Err, Rhs, L, R> BitOr<Rhs> for EitherNode<Err, L, R>
where
    Err: ParseError,
    Rhs: PredictiveGrammarNode<Err>,
    L: PredictiveGrammarNode<Err>,
    R: PredictiveGrammarNode<Err>,
{
    type Output = EitherNode<Err, Self, Rhs>;
    fn bitor(self, rhs: Rhs) -> Self::Output { self.or(rhs) }
}

impl<Err, Rhs> BitOr<Rhs> for Expect<Err>
where
    Err: ParseError,
    Rhs: PredictiveGrammar<Err>,
{
    type Output = Either<Err, Self, Rhs>;
    fn bitor(self, rhs: Rhs) -> Self::Output { self.or(rhs) }
}

impl<Err, Rhs, G> BitOr<Rhs> for Committed<Err, G>
where
    Err: ParseError,
    Rhs: PredictiveGrammar<Err>,
    G: PredictiveGrammarNode<Err>,
{
    type Output = Either<Err, Self, Rhs>;
    fn bitor(self, rhs: Rhs) -> Self::Output { self.or(rhs) }
}

impl<Err, Rhs, G> BitOr<Rhs> for Uncommitted<Err, G>
where
    Err: ParseError,
    Rhs: PredictiveGrammarNode<Err>,
    G: PredictiveGrammar<Err>,
{
    type Output = EitherNode<Err, Self, Rhs>;
    fn bitor(self, rhs: Rhs) -> Self::Output { self.or(rhs) }
}
