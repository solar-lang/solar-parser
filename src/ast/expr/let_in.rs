use crate::{Parse, ast::{identifier::Identifier, keywords}, parse::Res, util::{from_to, joined_by1}};

use super::FullExpression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LetExpression<'a> {
    pub span: &'a str,
    pub definitions: Vec<(Identifier<'a>, FullExpression<'a>)>,
    pub body: FullExpression<'a>,
}

impl<'a> Parse<'a> for LetExpression<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Let::parse(input)?;

        fn item(s: &str) -> Res<'_, (Identifier<'_>, FullExpression<'_>)> {
            let (rest, ident) = Identifier::parse_ws(s)?;
            let (rest, _) = keywords::Assign::parse_ws(rest)?;
            let (rest, expr) = FullExpression::parse_ws(rest)?;

            Ok((rest, (ident, expr)))
        }

        let (rest, definitions) = joined_by1(item, keywords::Comma::parse_ws)(rest)?;
        
        let (rest, _) = keywords::In::parse_ws(rest)?;
        
        let (rest, body) = FullExpression::parse_ws(rest)        ?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, LetExpression { span, definitions, body }))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_in_test1() {
        let input = "let square = x*x in {something}";
        let (rest, let_in_expression) = LetExpression::parse(input).unwrap();

        assert_eq!(rest, "", "all input consumed");
        assert_eq!(
            let_in_expression,
            LetExpression {
                span: input,
                definitions: vec![(Identifier::parse_ws(" square").unwrap().1, FullExpression::parse_ws(" x*x").unwrap().1)],
                body: FullExpression::parse_ws(" {something}").unwrap().1,
            }
        );
    }
}
