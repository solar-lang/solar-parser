pub mod block;
pub mod closure;
pub mod let_in;
pub mod literal;
pub mod string;
pub mod when;
pub use string::*;
pub mod full;
pub use block::BlockExpression;
pub use full::FullExpression;
pub use literal::Literal;

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, terminated},
};

use crate::ast::identifier::{Identifier, IdentifierPath};
use crate::{ast::*, parse::*, util::*};
use when::When;

/// Expressions
/// The main element of the solar language

//  pub span: &'a str,
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression<'a> {
    FunctionCall(FunctionCall<'a>),
    Value(Value<'a>),
}

impl<'a> Expression<'a> {
    pub fn span(&'a self) -> &'a str {
        match self {
            Expression::FunctionCall(f) => f.span,
            Expression::Value(v) => v.span(),
        }
    }
}

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((
            map(FunctionCall::parse, Expression::FunctionCall),
            map(Value::parse, Expression::Value),
        ))(input)
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;

    fn parse(input: &str) -> Value {
        Value::parse_ws(input).unwrap().1
    }

    #[test]
    fn exponent1() {
        let input = "a^b ";
        let (rest, value) = Value::parse(input).unwrap();
        assert_eq!(
            value,
            Value::Power(Power {
                span: "a^b",
                value: Box::new(parse("a")),
                exponent: Box::new(parse("b")),
            })
        );

        assert_eq!(rest, " ");
    }

    #[test]
    fn exponent2() {
        // a^b^c must equal a^(b^c)
        let input = "a^b^2 ";
        let (rest, value) = Value::parse(input).unwrap();
        assert_eq!(
            value,
            Value::Power(Power {
                span: "a^b^2",
                value: Box::new(parse("a")),
                exponent: Box::new(parse("b^2")),
            })
        );

        assert_eq!(rest, " ");
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value<'a> {
    Literal(Literal<'a>),
    IString(IString<'a>),
    // TODO add field access later?
    FullIdentifier(IdentifierPath<'a>),
    Closure(closure::Closure<'a>),
    Array(Array<'a>),
    Abs(Abs<'a>),
    /// Tuples are, more often than not, just expressions wrapped in Parenthesis
    Tuple(Tuple<'a>),
    When(When<'a>),
    BlockExpression(BlockExpression<'a>),

    // √x^2    == (√x)^2
    // -x^2    == (-x)^2
    // !x^2    == (!x)^2
    Sqrt(Sqrt<'a>),
    Not(Not<'a>),

    Power(Power<'a>),
}

impl<'a> Value<'a> {
    pub fn span(&'a self) -> &'a str {
        match self {
            Self::Literal(l) => l.span(),
            Self::IString(l) => l.span,
            Self::FullIdentifier(l) => l.span,
            Self::Closure(l) => l.span,
            Self::Array(l) => l.span,
            Self::Tuple(l) => l.span,
            Self::Abs(l) => l.span,
            Self::BlockExpression(l) => l.span,
            Self::Sqrt(l) => l.span,
            Self::Not(l) => l.span,
            Self::Power(l) => l.span,
            Self::When(l) => l.span,
        }
    }
}

impl<'a> Parse<'a> for Value<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, value) = alt((
            map(Literal::parse, Value::Literal),
            map(IString::parse, Value::IString),
            map(IdentifierPath::parse, Value::FullIdentifier),
            map(closure::Closure::parse, Value::Closure),
            map(Array::parse, Value::Array),
            map(Abs::parse, Value::Abs),
            map(Tuple::parse, Value::Tuple),
            map(When::parse, Value::When),
            map(BlockExpression::parse, Value::BlockExpression),
            // unary expressions
            map(Sqrt::parse, Value::Sqrt),
            map(Not::parse, Value::Not),
        ))(input)?;

        // TODO move to SQRT,NEGATE,NOT
        // There's an exponent coming
        if let Ok((rest, _)) = keywords::Power::parse_ws(rest) {
            let (rest, exponent) = Value::parse_ws(rest)?;
            let span = unsafe { from_to(input, rest) };

            let value = Box::new(value);
            let exponent = Box::new(exponent);

            return Ok((
                rest,
                Value::Power(Power {
                    span,
                    value,
                    exponent,
                }),
            ));
        }

        Ok((rest, value))
    }
}

// Parsing is implemented implicitly in Value
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Power<'a> {
    pub span: &'a str,
    pub value: Box<Value<'a>>,
    pub exponent: Box<Value<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Not<'a> {
    pub span: &'a str,
    pub expr: Box<Value<'a>>,
}

impl<'a> Parse<'a> for Not<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Not::parse(input)?;
        let (rest, expr) = Value::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };
        let expr = Box::new(expr);

        Ok((rest, Not { span, expr }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sqrt<'a> {
    pub span: &'a str,
    pub expr: Box<Value<'a>>,
}

impl<'a> Parse<'a> for Sqrt<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Sqrt::parse(input)?;
        let (rest, expr) = Value::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };
        let expr = Box::new(expr);

        Ok((rest, Self { span, expr }))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Abs<'a> {
    pub span: &'a str,
    pub expr: FullExpression<'a>,
}

impl<'a> Parse<'a> for Abs<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, expr) = delimited(
            keywords::Abs::parse,
            FullExpression::parse_ws,
            keywords::Abs::parse_ws,
        )(input)?;
        let span = unsafe { from_to(input, rest) };

        Ok((rest, Abs { span, expr }))
    }
}

// may as well just be some parenthesis
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tuple<'a> {
    pub span: &'a str,
    pub values: Vec<FullExpression<'a>>,
}

impl<'a> Parse<'a> for Tuple<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, values) = delimited(
            keywords::ParenOpen::parse,
            separated_list0(keywords::Comma::parse_ws, FullExpression::parse_ws),
            keywords::ParenClose::parse_ws,
        )(input)?;
        let span = unsafe { from_to(input, rest) };

        Ok((rest, Tuple { span, values }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Array<'a> {
    pub span: &'a str,
    pub values: Vec<FullExpression<'a>>,
}

impl<'a> Parse<'a> for Array<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, values) = delimited(
            keywords::BracketOpen::parse,
            terminated(
                separated_list0(keywords::Comma::parse_ws, FullExpression::parse_ws),
                opt(keywords::Comma::parse_ws),
            ),
            keywords::BracketClose::parse_ws,
        )(input)?;
        let span = unsafe { from_to(input, rest) };

        Ok((rest, Array { span, values }))
    }
}

// Note: may as well be a variable instaed of a function name
// Note: may be field access. Currently there is no distinction in the parser.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionCall<'a> {
    pub span: &'a str,
    pub function_name: IdentifierPath<'a>,
    pub args: Vec<FunctionArg<'a>>,
}

impl<'a> Parse<'a> for FunctionCall<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, function_name) = IdentifierPath::parse(input)?;
        let (rest, args) = many0(FunctionArg::parse_ws)(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            FunctionCall {
                span,
                function_name,
                args,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionArg<'a> {
    pub span: &'a str,
    pub name: Option<Identifier<'a>>,
    pub value: Value<'a>,
}

impl<'a> Parse<'a> for FunctionArg<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, name) = opt(terminated(Identifier::parse, keywords::Assign::parse_ws))(input)?;
        let (rest, value) = Value::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, FunctionArg { span, name, value }))
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
    derive_tests!(Abs, abs_expr, ["|x|", "|[1, 2, 3]|"]);

    derive_tests!(Array, arrays, ["[]", "[1]", "[ 1,2,3 ]", "[1, 2, ]"]);
}
