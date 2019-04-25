use crate::ast::*;
use crate::parser;
use combine::{ParseError, Parser, RangeStream};
use combine::parser::char::{alpha_num, space, string};
use combine::parser::choice::{choice, optional};
use combine::parser::combinator::recognize;
use combine::parser::item::{satisfy, token};
use combine::parser::repeat::{escaped, many, sep_by, sep_by1, skip_many, skip_many1};

parser!(fn code_block() -> Vec<Definition> {
    ( string("```")
    , string("js")
    , whitespace()
    , many(definition().skip(whitespace()))
    , string("```")
    ).map(|(_, _, _, definitions, _)| definitions)
});

parser!(fn definition() -> Definition {
    combine::parser(definition_impl_)
});

parser!(fn definition_impl_(input: &mut Input) -> Definition {
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
});

parser!(fn enum_defn() -> Enum {
    ( string("enum").skip(whitespace())
    , ident().skip(whitespace())
    , token('{').skip(whitespace())
    , sep_by(str_literal().skip(whitespace()), token('|').skip(whitespace()))
    , token('}')
    ).map(|(_, name, _, literals, _)| {
        Enum {
            name,
            literals,
            is_extension: false,
        }
    })
});

parser!(fn interface_defn() -> Interface {
    ( string("interface").skip(whitespace())
    , ident().skip(whitespace())
    , inherits().skip(whitespace())
    , token('{').skip(whitespace())
    , many::<Vec<_>, _>(field().skip(whitespace()))
    , token('}'),
    )
    .map(|(_, name, parents, _, fields, _)| {
        Interface {
            name,
            parents,
            fields,
            is_extension: false,
        }
    })
});

parser!(fn inherits() -> Vec<String> {
    optional(
        ( string("<:").skip(whitespace())
        , sep_by1(ident().skip(whitespace()), token(',').skip(whitespace()))
        ).map(|(_, x)| x)
    )
    .map(|x| x.unwrap_or_default())
});

parser!(fn field() -> Field {
    ( ident().skip(whitespace())
    , token(':').skip(whitespace())
    , type_expr().skip(whitespace())
    , token(';')
    ).map(|(name, _, type_, _)| Field { name, type_ })
});

parser!(fn type_expr() -> Type {
    combine::parser(type_impl_)
});

parser!(fn type_impl_(input: &mut Input) -> Type {
    choice((
        type_union(),
        type_array(),
        type_object(),
    ))
    .parse_stream(input)
});

parser!(fn type_atom() -> Type {
    choice((
        ident().map(Type::Named),
        str_literal().map(Type::StringLiteral)
    ))
});

parser!(fn type_array() -> Type {
    ( token('[').skip(whitespace())
    , type_expr().skip(whitespace())
    , token(']')
    ).map(|(_, type_, _)| Type::Array(Box::new(type_)))
});

parser!(fn type_object() -> Type {
    ( token('{').skip(whitespace())
    , many(field().skip(whitespace()))
    , token('}')
    ).map(|(_, fields, _)| Type::Object(fields))
});

parser!(fn type_union() -> Type {
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
});

parser!(fn str_literal() -> String {
    recognize((
        token('"'),
        escaped(
            skip_many1(satisfy(|c| c != '"' && c != '\\')),
            '\\',
            token('"')
        ),
    ))
});

parser!(fn ident() -> String {
    recognize::<String, _>(skip_many1(alpha_num().or(token('_'))))
});

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
