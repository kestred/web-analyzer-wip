use crate::ast::*;
use combine::{parser, ParseError, ParseResult, Parser, RangeStream};
use combine::parser::char::{alpha_num, space, string};
use combine::parser::choice::{choice, optional};
use combine::parser::combinator::recognize;
use combine::parser::item::{satisfy, token};
use combine::parser::repeat::{escaped, many, sep_by, sep_by1, skip_many, skip_many1};

pub fn code_block<I>() -> impl Parser<Input = I, Output = Vec<Definition>>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("```"),
        string("js"),
        whitespace(),
        many(definition().skip(whitespace())),
        string("```"),
    )
    .map(|(_, _, _, definitions, _)| definitions)
}

pub fn definition<I>() -> impl Parser<Input = I, Output = Definition>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    parser(definition_inner)
}

fn definition_inner<I>(input: &mut I) -> ParseResult<Definition, I>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let checkpoint = input.checkpoint();
    let is_extension = string("extend")
        .skip(whitespace())
        .parse_stream(input)
        .is_ok();
    if !is_extension {
        input.reset(checkpoint);
    }

    choice((
        enum_defn().map(Definition::Enum),
        interface_defn().map(Definition::Interface),
    ))
    .map(|mut defn| {
        match &mut defn {
            Definition::Enum(node) => node.is_extension = is_extension,
            Definition::Interface(node) => node.is_extension = is_extension,
        }
        defn
    })
    .parse_stream(input)
}

pub fn enum_defn<I>() -> impl Parser<Input = I, Output = Enum>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("enum").skip(whitespace()),
        ident().skip(whitespace()),
        token('{').skip(whitespace()),
        sep_by(str_literal().skip(whitespace()), token('|').skip(whitespace())),
        token('}'),
    ).map(|(_, name, _, literals, _)| {
        Enum {
            name,
            literals,
            is_extension: false,
        }
    })
}

pub fn interface_defn<I>() -> impl Parser<Input = I, Output = Interface>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("interface").skip(whitespace()),
        ident().skip(whitespace()),
        inherits().skip(whitespace()),
        token('{').skip(whitespace()),
        many::<Vec<_>, _>(field().skip(whitespace())),
        token('}'),
    )
    .map(|(_, name, parents, _, fields, _)| {
        Interface {
            name,
            parents,
            fields,
            is_extension: false,
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
    parser(type_inner)
}

fn type_inner<I>(input: &mut I) -> ParseResult<Type, I>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
     choice((
        type_union(),
        type_array(),
        type_object(),
    ))
    .parse_stream(input)
}

pub fn type_atom<I>() -> impl Parser<Input = I, Output = Type>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        ident().map(Type::Named),
        str_literal().map(Type::StringLiteral)
    ))
}

pub fn type_array<I>() -> impl Parser<Input = I, Output = Type>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        token('[').skip(whitespace()),
        type_expr().skip(whitespace()),
        token(']'),
    ).map(|(_, type_, _)| Type::Array(Box::new(type_)))
}

pub fn type_object<I>() -> impl Parser<Input = I, Output = Type>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        token('{').skip(whitespace()),
        many(field().skip(whitespace())),
        token('}'),
    ).map(|(_, fields, _)| Type::Object(fields))
}

pub fn type_union<I>() -> impl Parser<Input = I, Output = Type>
where
    I: RangeStream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    sep_by1(
        choice((
            type_atom(),
            type_array(),
            type_object(),
        )).skip(whitespace()),

        // seperator
        token('|').skip(whitespace())
    )
    .map(|x: Vec<Type>| {
        if x.len() > 1 {
            Type::Union(x)
        } else {
            x.into_iter().next().unwrap()
        }
    })
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
        let result = interface_defn().easy_parse(State::new(example));
        assert!(result.is_ok(), "\n\n{}\n", result.unwrap_err());
        let (data, mut state) = result.unwrap();
        assert!(state.uncons().is_err());
        assert_eq!(data.name, "AssignmentProperty");
        assert_eq!(data.parents.len(), 2);
    }
}
