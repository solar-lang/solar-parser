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

use crate::comment::parse_comment;

/// Tree representation of the syntax of a solar file
#[derive(Clone, Debug)]
pub struct Ast<'a> {
    pub span: &'a str,
    pub imports: Vec<Import<'a>>,
    pub items: Vec<BodyItem<'a>>,
}

pub type NomErr<'a> = nom::Err<nom::error::Error<&'a str>>;
impl<'a> Ast<'a> {
    /// Parses the source code into a valid Ast
    /// while making sure the entire source code is getting consumed.
    pub fn from_source_code(source_code: &'a str) -> Result<Ast<'a>, NomErr<'a>> {
        use crate::parse::Parse;
        use nom::combinator::map;
        let (rest, ast) = Ast::parse_ws(source_code)?;

        // Now, let's see, if there is any code in the input remaining.
        // for that, we will parse all the remaining whitespace AND comments.
        let (rest, _) = parse_comment(rest)?;
        // apparently more syntax elements are occuring.
        // return the appropriate error
        if !rest.is_empty() {
            // this will yield an error
            let Err(e) = nom::branch::alt((
                // problem might have occured within the imports
                map(Import::parse, drop),
                // or in any regular syntax element.
                // The distinction is soley,
                // because we want imports to appear in the beginning
                map(BodyItem::parse, drop),
            ))(rest) else {
                unreachable!("The parser should have returned with an error on remaining input '{}'", rest);
            };

            return Err(e);
        }

        Ok(ast)
    }
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
