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
    ( ws()
    , string("grammar").skip(ws())
    , ident().skip(ws())
    , token(';').skip(ws())
    , many(import().skip(ws()))
    , many1::<Vec<_>, _>(rule().skip(ws()))
    ).map(|(_, _, name, _, imports, rules)| {
        Grammar {
            name,
            rules: rules.into_iter().map(Rc::new).collect(),
            imports,
        }
    })
});

parser!(fn import() -> Import {
    ( string("import").skip(ws())
    , ident().skip(ws())
    , optional((string("from").skip(ws()), string_literal()).map(|(_, path)| path))
    , token(';')
    ).map(|(_, name, path, _)| Import { name, path })
});

parser!(fn rule() -> Rule {
    ( optional(many((token('#'), token('['), attribute(), token(']')).map(|(_, _, a, _)| a)).skip(ws()))
    , ident().skip(ws())
    , token(':').skip(ws())
    , pattern().skip(ws())
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
    , optional((token('('), sep_by1(attribute().skip(ws()), token(',').skip(ws())), token(')')))
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
        ( series().skip(ws())
        , optional(
            ( token('#').skip(ws())
            , ident().skip(ws())
            ).map(|(_, name)| name))
        )
        .map(|(series, node)| match node {
            Some(kind) => Pattern::Node(kind, Box::new(series)),
            None => series,
        }),
        token('|').skip(ws())
    )
    .map(Pattern::Choice)
    .map(Pattern::flatten_once)
    .parse_stream(input)
});

parser!(fn series() -> Pattern {
    many1::<Vec<_>, _>(atom().skip(ws()))
        .map(Pattern::Series)
        .map(Pattern::flatten_once)
});

parser!(fn atom() -> Pattern {
    ( combine::parser(atom_recursive).skip(ws())
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
        contextual(),
    ))
    .parse_stream(input)
});

parser!(fn group() -> Pattern {
    ( token('(').skip(ws())
    , pattern().skip(ws())
    , token(')').skip(ws())
    ).map(|(_, pattern, _)| pattern.flatten_once())
});

parser!(fn contextual() -> Pattern {
    ( token('@')
    , choice((
        token_literal().map(|lit| {
            let chars = lit.chars().collect();
            Pattern::Compound(chars)
        }),
        string_literal().map(|lit| {
            let keyword = lit[1 .. lit.len()-1].to_string();
            Pattern::Keyword(keyword)
        }),
    ))
    ).map(|(_, pat)| pat)
});

parser!(fn repeat() -> Repeat {
    choice((
        token('?').map(|_| Repeat::ZeroOrOne),
        token('*').map(|_| Repeat::ZeroOrMore),
        token('+').map(|_| Repeat::OneOrMore),
    ))
});

parser!(fn predicate() -> Pattern {
    ( token('{').skip(ws())
    , predicate_expression()
    , token('}')
    , token('?').skip(ws())
    , optional(series())
    ).map(|(_, expr, _, _, tail)| Pattern::Predicate(expr, Box::new(tail.unwrap_or(Pattern::Empty))))
});

parser!(fn predicate_expression() -> PredicateExpression {
    combine::parser(predicate_expression_recursive)
});

parser!(fn predicate_expression_recursive(input: &mut Input) -> PredicateExpression {
    ( optional(token('!').skip(ws()))
    , call_expression().skip(ws())
    , optional((string("&&").skip(ws()), predicate_expression()))
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
    , token('(').skip(ws())
    , sep_by(call_argument().skip(ws()), token(',').skip(ws()))
    , token(')').skip(ws())
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

fn ws<I>() -> impl Parser<Input = I>
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
