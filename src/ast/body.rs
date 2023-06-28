use crate::ast::expr::{BlockExpression, StringLiteral};

use crate::ast::*;
use crate::parse::*;
use crate::util::*;

use nom::combinator::{cut, opt};
use nom::sequence::{delimited, pair};
use nom::{branch::alt, combinator::map};

use super::expr::FullExpression;
use super::identifier::Identifier;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildinTypeDecl<'a> {
    pub span: &'a str,
    pub name: Identifier<'a>,
    pub generic_symbols: Option<GenericSymbols<'a>>,
}

impl<'a> Parse<'a> for BuildinTypeDecl<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::BuildinType::parse(input)?;
        let (rest, name) = Identifier::parse_ws(rest)?;
        let (rest, generic_symbols) = opt(GenericSymbols::parse_ws)(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            BuildinTypeDecl {
                span,
                name,
                generic_symbols,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BodyItem<'a> {
    Function(function::Function<'a>),
    TypeDecl(TypeDecl<'a>),
    BuildinTypeDecl(BuildinTypeDecl<'a>),
    Test(Test<'a>),
    Let(Let<'a>),
}

impl<'a> BodyItem<'a> {
    pub fn span(&self) -> &str {
        use BodyItem::*;
        match self {
            Function(f) => f.span,
            TypeDecl(t) => t.span,
            BuildinTypeDecl(t) => t.span,
            Test(t) => t.span,
            Let(l) => l.span,
        }
    }
}

impl<'a> Parse<'a> for BodyItem<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((
            map(Test::parse, BodyItem::Test),
            map(TypeDecl::parse, BodyItem::TypeDecl),
            map(BuildinTypeDecl::parse, BodyItem::BuildinTypeDecl),
            map(function::Function::parse, BodyItem::Function),
            map(Let::parse, BodyItem::Let),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Let<'a> {
    pub span: &'a str,
    pub identifier: Identifier<'a>,
    pub expr: FullExpression<'a>,
}

impl<'a> Parse<'a> for Let<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, (identifier, expr)) = pair(
            delimited(
                keywords::Let::parse,
                Identifier::parse_ws,
                keywords::Assign::parse_ws,
            ),
            FullExpression::parse_ws,
        )(input)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            Let {
                span,
                identifier,
                expr,
            },
        ))
    }
}
