use crate::ast::*;
use crate::parser;
use combine::{ParseError, Parser, Stream};
use combine::parser::char::{alpha_num, space, string};
use combine::parser::choice::{choice, optional};
use combine::parser::combinator::{attempt, recognize};
use combine::parser::item::{satisfy, token};
use combine::parser::repeat::{escaped, many, many1, sep_by, sep_by1, skip_many, skip_many1};
use std::rc::Rc;

parser!(fn grammar() -> Grammar {
    ( whitespace()
    , string("grammar").skip(whitespace())
    , ident().skip(whitespace())
    , token(';').skip(whitespace())
    , many1::<Vec<_>, _>(rule().skip(whitespace()))
    ).map(|(_, _, name, _, rules)| Grammar { name, rules: rules.into_iter().map(Rc::new).collect() })
});

parser!(fn rule() -> Rule {
    ( optional(many((token('#'), token('['), attribute(), token(']')).map(|(_, _, a, _)| a)).skip(whitespace()))
    , ident().skip(whitespace())
    , token(':').skip(whitespace())
    , pattern().skip(whitespace())
    , token(';')
    ).map(|(attrs, name, _, pattern, _)| {
        Rule {
            name,
            pattern,
            attributes: attrs.unwrap_or_default()
        }
    })
});

parser!(fn attribute() -> Attribute {
    combine::parser(attribute_recursive)
});

parser!(fn attribute_recursive(input: &mut Input) -> Attribute {
    ( ident()
    , optional((token('('), sep_by1(attribute().skip(whitespace()), token(',').skip(whitespace())), token(')')))
    )
    .map(|(word, list)| match list {
        Some((_, attrs, _)) => Attribute::Group(word, attrs),
        None => Attribute::Word(word)
    })
    .parse_stream(input)
});

parser!(fn pattern() -> Pattern {
    combine::parser(pattern_recursive)
});

parser!(fn pattern_recursive(input: &mut Input) -> Pattern {
    sep_by(
        ( series().skip(whitespace())
        , optional(
            ( token('#').skip(whitespace())
            , ident().skip(whitespace())
            ).map(|(_, name)| name))
        )
        .map(|(series, node)| match node {
            Some(kind) => Pattern::Node(kind, Box::new(series)),
            None => series,
        }),
        token('|').skip(whitespace())
    )
    .map(Pattern::Choice)
    .map(Pattern::flatten_once)
    .parse_stream(input)
});

parser!(fn series() -> Pattern {
    many1::<Vec<_>, _>(atom().skip(whitespace()))
        .map(Pattern::Series)
        .map(Pattern::flatten_once)
});

parser!(fn atom() -> Pattern {
    ( combine::parser(atom_recursive).skip(whitespace())
    , optional(repeat())
    ).map(|(atom, repeat)| {
        match repeat {
            Some(repeat) => Pattern::Repeat(Box::new(atom), repeat),
            None => atom,
        }
    })
});

parser!(fn atom_recursive(input: &mut Input) -> Pattern {
    choice((
        ident().map(Pattern::Ident),
        token_literal().map(Pattern::Literal),
        group(),
        predicate(),
    ))
    .parse_stream(input)
});

parser!(fn group() -> Pattern {
    ( token('(').skip(whitespace())
    , pattern().skip(whitespace())
    , token(')').skip(whitespace())
    ).map(|(_, pattern, _)| pattern.flatten_once())
});

parser!(fn repeat() -> Repeat {
    choice((
        token('?').map(|_| Repeat::ZeroOrOne),
        token('*').map(|_| Repeat::ZeroOrMore),
        token('+').map(|_| Repeat::OneOrMore),
    ))
});

parser!(fn predicate() -> Pattern {
    ( token('{').skip(whitespace())
    , predicate_expression()
    , token('}')
    , token('?').skip(whitespace())
    , optional(series())
    ).map(|(_, expr, _, _, tail)| Pattern::Predicate(expr, Box::new(tail.unwrap_or(Pattern::Empty))))
});

parser!(fn predicate_expression() -> PredicateExpression {
    combine::parser(predicate_expression_recursive)
});

parser!(fn predicate_expression_recursive(input: &mut Input) -> PredicateExpression {
    ( optional(token('!').skip(whitespace()))
    , call_expression().skip(whitespace())
    , optional((string("&&").skip(whitespace()), predicate_expression()))
    )
    .map(|(unary, expr, binary)| {
        let left = if let Some(oper) = unary {
            PredicateExpression::Unary { oper, expr: Box::new(expr) }
        } else {
            expr
        };
        if let Some((_, right)) = binary {
            PredicateExpression::Binary { left: Box::new(left), oper: "&&", right: Box::new(right) }
        } else {
            left
        }
    })
    .parse_stream(input)
});

parser!(fn call_expression() -> PredicateExpression {
    ( ident()
    , token('(').skip(whitespace())
    , sep_by(call_argument().skip(whitespace()), token(',').skip(whitespace()))
    , token(')').skip(whitespace())
    ).map(|(method, _, args, _)| PredicateExpression::Call { method, args })
});

parser!(fn call_argument() -> String {
    choice((string_literal(), ident()))
});

parser!(fn token_literal() -> String {
    recognize(
        ( token('\'')
        , escaped(skip_many1(satisfy(|c| c != '\'' && c != '\\')), '\\', token('\''))
        )
    ).map(|lit: String| lit[1 .. lit.len()-1].to_string())
});

parser!(fn string_literal() -> String {
    recognize(
        ( token('"')
        , escaped(skip_many1(satisfy(|c| c != '"' && c != '\\')), '\\', token('"'))
        )
    )
});

parser!(fn ident() -> String {
    recognize::<String, _>(skip_many1(alpha_num().or(token('_'))))
});

fn whitespace<I>() -> impl Parser<Input = I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let comment = (
        token('/'),
        choice((
            (token('/'), skip_many(satisfy(|c| c != '\n'))).map(|_| ()),
            (
                token('*'),
                skip_many(choice((
                    satisfy(|c| c != '*').map(|_| ()),
                    attempt((token('*'), satisfy(|c| c != '/'))).map(|_| ())
                ))),
                string("*/"),
            ).map(|_| ()),
        ))
    ).map(|_| ());
    skip_many(skip_many1(space()).or(comment))
}
