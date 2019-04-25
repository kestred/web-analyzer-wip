use crate::grammar::{Grammar, GrammarLike, Outcome};
use crate::parser::{ParseError, Parser};
use crate::parse_ok;
use rowan::SyntaxKind;

impl<Err, Func> Grammar<Err> for Func
where
    Func: Fn(&mut Parser<Err>) -> SyntaxKind,
    Err: ParseError,
{
    fn parse(&self, p: &mut Parser<Err>) -> SyntaxKind {
        self(p)
    }
}

impl<Err, Func, Return> GrammarLike<Err> for Func
where
    Err: ParseError,
    Func: Fn(&mut Parser<Err>) -> Return,
    Return: Into<Outcome>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        self(p).into()
    }
}

impl<Err, A> GrammarLike<Err> for (A,)
where
    Err: ParseError,
    A: GrammarLike<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        self.0.parse(p)
    }
}

impl<Err, A, B> GrammarLike<Err> for (A, B)
where
    Err: ParseError,
    A: GrammarLike<Err>,
    B: GrammarLike<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        self.1.parse(p)
    }
}

impl<Err, A, B, C> GrammarLike<Err> for (A, B, C)
where
    Err: ParseError,
    A: GrammarLike<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        parse_ok!(self.1.parse(p));
        self.2.parse(p)
    }
}

impl<Err, A, B, C, D> GrammarLike<Err> for (A, B, C, D)
where
    Err: ParseError,
    A: GrammarLike<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
    D: GrammarLike<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        parse_ok!(self.1.parse(p));
        parse_ok!(self.2.parse(p));
        self.3.parse(p)
    }
}

impl<Err, A, B, C, D, E> GrammarLike<Err> for (A, B, C, D, E)
where
    Err: ParseError,
    A: GrammarLike<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
    D: GrammarLike<Err>,
    E: GrammarLike<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        parse_ok!(self.1.parse(p));
        parse_ok!(self.2.parse(p));
        parse_ok!(self.3.parse(p));
        self.4.parse(p)
    }
}

impl<Err, A, B, C, D, E, F> GrammarLike<Err> for (A, B, C, D, E, F)
where
    Err: ParseError,
    A: GrammarLike<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
    D: GrammarLike<Err>,
    E: GrammarLike<Err>,
    F: GrammarLike<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        parse_ok!(self.1.parse(p));
        parse_ok!(self.2.parse(p));
        parse_ok!(self.3.parse(p));
        parse_ok!(self.4.parse(p));
        self.5.parse(p)
    }
}

impl<Err, A, B, C, D, E, F, G> GrammarLike<Err> for (A, B, C, D, E, F, G)
where
    Err: ParseError,
    A: GrammarLike<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
    D: GrammarLike<Err>,
    E: GrammarLike<Err>,
    F: GrammarLike<Err>,
    G: GrammarLike<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        parse_ok!(self.1.parse(p));
        parse_ok!(self.2.parse(p));
        parse_ok!(self.3.parse(p));
        parse_ok!(self.4.parse(p));
        parse_ok!(self.5.parse(p));
        self.6.parse(p)
    }
}

impl<Err, A, B, C, D, E, F, G, H> GrammarLike<Err> for (A, B, C, D, E, F, G, H)
where
    Err: ParseError,
    A: GrammarLike<Err>,
    B: GrammarLike<Err>,
    C: GrammarLike<Err>,
    D: GrammarLike<Err>,
    E: GrammarLike<Err>,
    F: GrammarLike<Err>,
    G: GrammarLike<Err>,
    H: GrammarLike<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        parse_ok!(self.1.parse(p));
        parse_ok!(self.2.parse(p));
        parse_ok!(self.3.parse(p));
        parse_ok!(self.4.parse(p));
        parse_ok!(self.5.parse(p));
        parse_ok!(self.6.parse(p));
        self.7.parse(p)
    }
}