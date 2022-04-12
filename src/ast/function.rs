
use nom::multi::many0;

use nom::sequence::preceded;

use nom::combinator::opt;

use crate::Parse;
use crate::util::from_to;
use crate::{ast::expr::FullExpression, parse::Res};

use crate::ast::identifier::Identifier;

use super::{Type, keywords};

/// e.g.
/// export fun fib(n :: Int) = { if (n == 0) 0 if (n == 1) 1 fib (n-1) + fib (n-2)}
/// fun string(person) = person.name
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function<'a> {
    pub span: &'a str,
    pub exported: bool,
    pub name: Identifier<'a>,
    pub args: Vec<(Identifier<'a>, Option<Type<'a>>)>,
    pub body: FullExpression<'a>,
}

impl<'a> Parse<'a> for Function<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        // export
        let (rest, exported) = opt(keywords::Export::parse)(input)?;
        let exported = exported.is_some();

        // fun
        let (rest, _) = keywords::Function::parse_ws(rest)?;

        // fib
        let (rest, name) = Identifier::parse_ws(rest)?;

        let (rest, _) = keywords::ParenOpen::parse_ws(rest)?;

        let args = |input| {
            let (rest, ident) = Identifier::parse_ws(input)?;
            let (rest, ty) = opt(preceded(keywords::TypeHint::parse_ws, Type::parse_ws))(rest)?;

            Ok((rest, (ident, ty)))
        };

        let (rest, args) = many0(args)(rest)?;
        let (rest, _) = keywords::ParenClose::parse_ws(rest)?;

        let (rest, body) = FullExpression::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            Function {
                span,
                exported,
                name,
                args,
                body,
            },
        ))
    }
}
