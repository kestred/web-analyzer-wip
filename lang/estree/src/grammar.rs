use crate::ast::*;
use combine::{ParseError, Parser, RangeStream};
use combine::parser::char::{alpha_num, space, string};
use combine::parser::choice::{choice, optional};
use combine::parser::combinator::recognize;
use combine::parser::item::{satisfy, token};
use combine::parser::repeat::{escaped, many, sep_by1, skip_many, skip_many1};
use combine::parser::sequence::between;

pub fn code_block<I>() -> impl Parser<Input = I, Output = Vec<Interface>>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("```"),
        string("js"),
        whitespace(),
        many(interface().skip(whitespace())),
        string("```"),
    )
    .map(|(_, _, _, interfaces, _)| interfaces)
}

pub fn interface<I>() -> impl Parser<Input = I, Output = Interface>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(string("extend").skip(whitespace())),
        string("interface").skip(whitespace()),
        ident().skip(whitespace()),
        inherits().skip(whitespace()),
        token('{').skip(whitespace()),
        many::<Vec<_>, _>(field().skip(whitespace())),
        token('}'),
    )
    .map(|(extend_kw, _, name, parents, _, fields, _)| {
        Interface {
            name,
            parents,
            fields,
            extend: extend_kw.is_some(),
        }
    })
}

pub fn inherits<I>() -> impl Parser<Input = I, Output = Vec<String>>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    optional(
        (
            string("<:").skip(whitespace()),
            sep_by1(ident().skip(whitespace()), token(',').skip(whitespace()))
        )
        .map(|(_, x)| x)
    )
    .map(|x| x.unwrap_or_default())
}

pub fn field<I>() -> impl Parser<Input = I, Output = Field>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        ident().skip(whitespace()),
        token(':').skip(whitespace()),
        type_expr().skip(whitespace()),
        token(';')
    )
    .map(|(name, _, type_, _)| Field { name, type_ })
}

pub fn type_expr<I>() -> impl Parser<Input = I, Output = Type>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        type_union(),
        between(token('['), token(']'), type_union()).map(|x| Type::Array(Box::new(x))),
    ))
}

pub fn type_union<I>() -> impl Parser<Input = I, Output = Type>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    sep_by1(type_atom().skip(whitespace()), token('|').skip(whitespace()))
        .map(|x: Vec<Type>| {
            if x.len() > 1 {
                Type::Union(x)
            } else {
                x.into_iter().next().unwrap()
            }
        })
}

pub fn type_atom<I>() -> impl Parser<Input = I, Output = Type>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        ident().map(Type::Named),
        str_literal().map(Type::StringLiteral),
    ))
}

fn ident<I>() -> impl Parser<Input = I, Output = String>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize::<String, _>(skip_many1(alpha_num().or(token('_'))))
}

fn str_literal<I>() -> impl Parser<Input = I, Output = String>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    recognize((
        token('"'),
        escaped(
            skip_many1(satisfy(|c| c != '"' && c != '\\')),
            '\\',
            token('"')
        ),
    ))
}

fn whitespace<I>() -> impl Parser<Input = I>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let comment = (string("//"), skip_many(satisfy(|c| c != '\n'))).map(|_| ());
    skip_many(skip_many1(space()).or(comment))
}

#[cfg(test)]
mod test {
    use super::*;
    use combine::stream::state::State;
    use combine::stream::StreamOnce;

    #[test]
    fn test_parse_str_literal() {
        let example = r#"
"This is a string literal."
"#.trim();
        let result = str_literal().easy_parse(State::new(example));
        assert!(result.is_ok(), "\n\n{}\n", result.unwrap_err());
        let (_, mut state) = result.unwrap();
        assert!(state.uncons().is_err());
    }

    #[test]
    fn test_parse_interface() {
        let example = r#"
interface AssignmentProperty <: Property, OtherInterface {
    type: "Property"; // inherited
    value: Pattern | Other;
    kind: "init";
    method: false;
}
"#.trim();
        let result = interface().easy_parse(State::new(example));
        assert!(result.is_ok(), "\n\n{}\n", result.unwrap_err());
        let (data, mut state) = result.unwrap();
        assert!(state.uncons().is_err());
        assert_eq!(data.name, "AssignmentProperty");
        assert_eq!(data.parents.len(), 2);
    }
}
