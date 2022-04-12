use nom::{
    branch::alt,
    combinator::{map, opt},
    sequence::preceded,
};

use crate::{
    util::{from_to, joined_by},
    Parse,
};

use super::identifier::Identifier;
use crate::ast::keywords::{Comma, FatArrow, Function, ParenClose, ParenOpen};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type<'a> {
    Normal {
        span: &'a str,
        name: Identifier<'a>,
        generic: Vec<Type<'a>>,
    },
    Function {
        span: &'a str,
        args: Vec<Type<'a>>,
        ret: Option<Box<Type<'a>>>,
    },
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(input: &'a str) -> crate::parse::Res<'a, Self> {
        alt((parse_function, parse_normal))(input)
    }
}

fn parse_function<'a>(input: &'a str) -> crate::parse::Res<'a, Type> {
    let (rest, _) = Function::parse(input)?;
    let (rest, _) = ParenOpen::parse_ws(rest)?;

    // Int, Int
    let (rest, args) = joined_by(Type::parse_ws, Comma::parse_ws)(rest)?;

    let (rest, _) = ParenClose::parse_ws(rest)?;

    let (rest, ret) = opt(map(preceded(FatArrow::parse_ws, Type::parse_ws), Box::new))(rest)?;

    let span = unsafe { from_to(input, rest) };

    Ok((rest, Type::Function { span, args, ret }))
}

fn parse_normal<'a>(input: &'a str) -> crate::parse::Res<'a, Type> {
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
        Type::Normal {
            span,
            name,
            generic,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_generic() {
        let input = "List Int";
        let (_, ty) = Type::parse(input).unwrap();

        let name = Identifier {
            span: "List",
            value: "List",
        };

        let generic = vec![Type::Normal {
            span: "Int",
            name: Identifier {
                span: "Int",
                value: "Int",
            },
            generic: Vec::new(),
        }];

        assert_eq!(
            ty,
            Type::Normal {
                span: input,
                name,
                generic,
            }
        )
    }

    #[test]
    fn simple_function() {
        let input = "fun(Int, Int) =>  fun  (Int)";
        let (_, ty) = Type::parse(input).unwrap();

        let int_type = Type::Normal {
            span: "Int",
            name: Identifier {
                span: "Int",
                value: "Int",
            },
            generic: Vec::new(),
        };

        assert_eq!(
            ty,
            Type::Function {
                span: input,
                args: vec![int_type.clone(), int_type.clone()],
                ret: Some(Box::new(Type::Function {
                    span: "fun  (Int)",
                    args: vec![int_type],
                    ret: None
                })),
            }
        )
    }
}
