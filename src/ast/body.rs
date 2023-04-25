use crate::ast::expr::{BlockExpression, StringLiteral};

use crate::ast::*;
use crate::parse::*;
use crate::util::*;

use nom::combinator::cut;
use nom::{branch::alt, combinator::map};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FunctionOrTypeOrTest<'a> {
    Function(function::Function<'a>),
    TypeDecl(TypeDecl<'a>),
    Test(Test<'a>),
}

impl<'a> FunctionOrTypeOrTest<'a> {
    pub fn span(&self) -> &str {
        use FunctionOrTypeOrTest::*;
        match self {
            Function(f) => f.span,
            TypeDecl(t) => t.span,
            Test(t) => t.span,
        }
    }
}

impl<'a> Parse<'a> for FunctionOrTypeOrTest<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((
            map(Test::parse, FunctionOrTypeOrTest::Test),
            map(TypeDecl::parse, FunctionOrTypeOrTest::TypeDecl),
            map(function::Function::parse, FunctionOrTypeOrTest::Function),
        ))(input)
    }
}

/// test "equals 2" {
///     assert (1+1) 2
/// }
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Test<'a> {
    pub span: &'a str,
    pub name: StringLiteral<'a>,
    pub instructions: BlockExpression<'a>,
}

impl<'a> Parse<'a> for Test<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Test::parse(input)?;

        let (rest, name) = cut(expr::StringLiteral::parse_ws)(rest)?;
        let (rest, instructions) = cut(expr::BlockExpression::parse_ws)(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            Test {
                span,
                name,
                instructions,
            },
        ))
    }
}
