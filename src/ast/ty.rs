use nom::{
    branch::alt,
    combinator::{map, opt},
};

use crate::{
    util::{from_to, joined_by},
    Parse,
};

use super::identifier::Identifier;
use crate::ast::keywords::{Comma, ParenClose, ParenOpen};

/// TODO Type might also be a Function!
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type<'a> {
    pub span: &'a str,
    pub name: Identifier<'a>,
    pub generic: Vec<Type<'a>>,
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(input: &'a str) -> crate::parse::Res<'a, Self> {
        let generic_1 = map(Type::parse_ws, |t| vec![t]);
        let generic_many = |input| {
            let (rest, _) = ParenOpen::parse_ws(input)?;
            let (rest, items) = joined_by(Type::parse_ws, Comma::parse_ws)(rest)?;
            let (rest, _) = ParenClose::parse_ws(rest)?;

            Ok((rest, items))
        };

        let (rest, name) = Identifier::parse(input)?;
        let (rest, generic) = opt(alt((generic_1, generic_many)))(rest)?;
        let generic = generic.unwrap_or_else(Vec::new);

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            Type {
                span,
                name,
                generic,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_generic() {
        let input = "List Int";
        let (_, ty) = Type::parse(input).unwrap();

        assert_eq!(
            ty,
            Type {
                span: input,
                name: Identifier {
                    span: "List",
                    value: "List"
                },
                generic: vec![Type {
                    span: "Int",
                    name: Identifier {
                        span: "Int",
                        value: "Int"
                    },
                    generic: Vec::new(),
                }],
            }
        )
    }
}
