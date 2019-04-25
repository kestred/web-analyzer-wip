use crate::grammar::{GrammarNode, Grammar, Outcome};
use crate::parser::{ParseError, Parser};
use crate::parse_ok;
use rowan::SyntaxKind;

impl<Err, Func, Return> Grammar<Err> for Func
where
    Err: ParseError,
    Func: Fn(&mut Parser<Err>) -> Return,
    Return: Into<Outcome>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        self(p).into()
    }
}

impl<Err, Func> GrammarNode<Err> for Func
where
    Func: Fn(&mut Parser<Err>) -> SyntaxKind,
    Err: ParseError,
{
    fn parse(&self, p: &mut Parser<Err>) -> SyntaxKind {
        self(p)
    }
}

impl<Err, A> Grammar<Err> for (A,)
where
    Err: ParseError,
    A: Grammar<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        self.0.parse(p)
    }
}

impl<Err, A, B> Grammar<Err> for (A, B)
where
    Err: ParseError,
    A: Grammar<Err>,
    B: Grammar<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        self.1.parse(p)
    }
}

impl<Err, A, B, C> Grammar<Err> for (A, B, C)
where
    Err: ParseError,
    A: Grammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        parse_ok!(self.1.parse(p));
        self.2.parse(p)
    }
}

impl<Err, A, B, C, D> Grammar<Err> for (A, B, C, D)
where
    Err: ParseError,
    A: Grammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
    D: Grammar<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        parse_ok!(self.1.parse(p));
        parse_ok!(self.2.parse(p));
        self.3.parse(p)
    }
}

impl<Err, A, B, C, D, E> Grammar<Err> for (A, B, C, D, E)
where
    Err: ParseError,
    A: Grammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
    D: Grammar<Err>,
    E: Grammar<Err>,
{
    fn parse(&self, p: &mut Parser<Err>) -> Outcome {
        parse_ok!(self.0.parse(p));
        parse_ok!(self.1.parse(p));
        parse_ok!(self.2.parse(p));
        parse_ok!(self.3.parse(p));
        self.4.parse(p)
    }
}

impl<Err, A, B, C, D, E, F> Grammar<Err> for (A, B, C, D, E, F)
where
    Err: ParseError,
    A: Grammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
    D: Grammar<Err>,
    E: Grammar<Err>,
    F: Grammar<Err>,
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

impl<Err, A, B, C, D, E, F, G> Grammar<Err> for (A, B, C, D, E, F, G)
where
    Err: ParseError,
    A: Grammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
    D: Grammar<Err>,
    E: Grammar<Err>,
    F: Grammar<Err>,
    G: Grammar<Err>,
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

impl<Err, A, B, C, D, E, F, G, H> Grammar<Err> for (A, B, C, D, E, F, G, H)
where
    Err: ParseError,
    A: Grammar<Err>,
    B: Grammar<Err>,
    C: Grammar<Err>,
    D: Grammar<Err>,
    E: Grammar<Err>,
    F: Grammar<Err>,
    G: Grammar<Err>,
    H: Grammar<Err>,
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