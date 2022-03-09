use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
};

use crate::{parse::Res, util::from_to, Parse};

use super::{identifier::Identifier, keywords, Type};

/// type Either (a, b)
/// | Left :: a
/// | Right :: b
///
/// type Maybe x
/// | Some :: x
/// | None
///
/// type Person
/// - name  :: String
/// - age   :: Int
/// - likesBread :: Boolean
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeDecl<'a> {
    pub span: &'a str,
    pub name: Identifier<'a>,
    pub generic_symbols: Option<GenericSymbols<'a>>,
    pub fields: EnumOrStructFields<'a>,
}

impl<'a> Parse<'a> for TypeDecl<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        let (rest, _) = keywords::Type::parse(input)?;
        let (rest, name) = Identifier::parse_ws(rest)?;
        let (rest, generic_args_decl) = opt(GenericSymbols::parse_ws)(rest)?;
        let (rest, fields) = EnumOrStructFields::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            TypeDecl {
                span,
                name,
                generic_symbols: generic_args_decl,
                fields,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericSymbols<'a> {
    pub span: &'a str,
    pub symbols: Vec<Identifier<'a>>,
}

impl<'a> Parse<'a> for GenericSymbols<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::*;
        let (rest, generic_arguments) = alt((
            map(Identifier::parse, |i| vec![i]),
            delimited(
                ParenOpen::parse,
                separated_list1(Comma::parse_ws, Identifier::parse_ws),
                ParenClose::parse_ws,
            ),
        ))(input)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            GenericSymbols {
                span,
                symbols: generic_arguments,
            },
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnumOrStructFields<'a> {
    EnumFields(Vec<EnumField<'a>>),
    StructFields(Vec<StructField<'a>>),
}

impl<'a> Parse<'a> for EnumOrStructFields<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        alt((
            map(many1(EnumField::parse_ws), EnumOrStructFields::EnumFields),
            map(
                many1(StructField::parse_ws),
                EnumOrStructFields::StructFields,
            ),
        ))(input)
    }
}

/// | Some :: x
///
/// | Lightgrey
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumField<'a> {
    pub span: &'a str,
    pub name: Identifier<'a>,
    pub ty: Option<Type<'a>>,
}

impl<'a> Parse<'a> for EnumField<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        //      |
        let (rest, _) = keywords::Abs::parse(input)?;
        let (rest, name) = Identifier::parse_ws(rest)?;
        let (rest, ty) = opt(preceded(keywords::TypeHint::parse_ws, Type::parse_ws))(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((rest, EnumField { span, name, ty }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructField<'a> {
    pub span: &'a str,
    pub public: bool,
    pub mutable: bool,
    pub name: Identifier<'a>,
    pub ty: Type<'a>,
}

impl<'a> Parse<'a> for StructField<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use keywords::{Minus, Mut, Plus, TypeHint};

        // -
        // +
        let (rest, public) = alt((
            // if the field starts with a -, it will be private
            map(Minus::parse, |_| false),
            // Otherwise it will be public
            map(Plus::parse, |_| true),
        ))(input)?;

        // mut
        let (rest, mutable) = opt(Mut::parse_ws)(rest)?;
        let mutable = mutable.is_some();

        // name
        let (rest, name) = Identifier::parse_ws(rest)?;

        // :: String
        let (rest, _) = TypeHint::parse_ws(rest)?;
        let (rest, ty) = Type::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            StructField {
                span,
                public,
                mutable,
                name,
                ty,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn type_declarations() {
        let input = [
            "type TrafficLight | Red | Yellow | Green | RedYellow",
            "type Gender | Female | Male | Other :: String",
            "type Option t | Some :: t | None",
            "type Result (r, e) | Ok :: r | Err :: e",

            "type Point
            - x ::        Float
            - y :: Float",
            // public fields
            "type Point 
            + mut x :: Float
            + mut y :: Float",
            "type Point t 
            - x :: t
            - y :: t",
            "type Person 
            - birthday :: Date
            - name :: String",
        ];

        for i in &input {
            let (rest, value) = TypeDecl::parse(i).unwrap();
            // here we just test if the entire input was consumed
            assert_eq!(rest, "");
            assert_eq!(&value.span, i);
        }
    }
}
