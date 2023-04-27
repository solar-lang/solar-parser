pub mod body;
pub mod expr;
mod function;
pub mod identifier;
pub mod import;
pub mod keywords;
mod structs;
pub mod ty;
use body::BodyItem;
pub use function::*;
use import::Import;
pub use structs::*;
pub use ty::Type;

/// Tree representation of the syntax of a solar file
#[derive(Clone, Debug)]
pub struct Ast<'a> {
    pub span: &'a str,
    pub imports: Vec<Import<'a>>,
    pub items: Vec<BodyItem<'a>>,
}

impl<'a> crate::parse::Parse<'a> for Ast<'a> {
    fn parse(input: &'a str) -> crate::parse::Res<'a, Self> {
        use nom::multi::many0;

        let (rest, imports) = many0(Import::parse_ws)(input)?;
        let (rest, functions_and_types_and_tests) = many0(BodyItem::parse_ws)(rest)?;

        let span = unsafe { crate::util::from_to(input, rest) };

        Ok((
            rest,
            Ast {
                span,
                imports,
                items: functions_and_types_and_tests,
            },
        ))
    }
}
