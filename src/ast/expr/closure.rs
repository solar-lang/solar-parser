use nom::{
    branch::alt,
    combinator::{cut, map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded},
};

use crate::ast::identifier::Identifier;
use crate::{ast::*, parse::*, util::*};

use super::Expression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Closure<'a> {
    pub span: &'a str,
    pub arguments: ClosureArgs<'a>,
    pub ret: Option<Type<'a>>,
    pub body: Box<expr::FullExpression<'a>>,
}

impl<'a> Parse<'a> for Closure<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        // fun
        let (rest, _) = keywords::Function::parse(input)?;

        // (x)
        let (rest, arguments) = ClosureArgs::parse(rest)?;

        // -> Int
        let (rest, ret) = opt(preceded(keywords::ThinArrow::parse, cut(Type::parse_ws)))(rest)?;

        // =
        let (rest, _) = keywords::Assign::parse_ws(rest)?;
        // x^2
        let (rest, body) = map(expr::FullExpression::parse_ws, Box::new)(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            Closure {
                span,
                arguments,
                ret,
                body,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosureArgs<'a> {
    pub span: &'a str,
    pub args: Vec<(Identifier<'a>, Option<ty::Type<'a>>)>,
}

impl<'a> Parse<'a> for ClosureArgs<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::*;
        let (rest, args) = delimited(
            ParenOpen::parse,
            separated_list0(
                Comma::parse_ws,
                pair(
                    Identifier::parse_ws,
                    opt(preceded(keywords::TypeHint::parse_ws, cut(ty::Type::parse_ws))),
                ),
            ),
            ParenClose::parse_ws,
        )(input)?;

        let span = unsafe { from_to(input, rest) };
        Ok((rest, ClosureArgs { span, args }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! derive_tests {
        ($ty:ty, $testname:ident, $list:tt) => {
            #[test]
            fn $testname() {
                let input = $list;
                for i in input.iter() {
                    let (rest, _) = <$ty>::parse(i).unwrap();
                    assert_eq!(rest, "");
                }
            }
        };
    }
    derive_tests!(
        ClosureArgsKind,
        closure_arguments,
        [
            "(x)",
            "x",
            "(x, y)",
            "(x: Float, y: Float)",
            "(x: Float, y: Float, info)"
        ]
    );
}
