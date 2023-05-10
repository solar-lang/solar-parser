use nom::IResult;

use crate::comment::parse_comment;

pub type Res<'a, T> = IResult<&'a str, T>;

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(input: &'a str) -> Res<'a, Self>;

    fn parse_ws(input: &'a str) -> Res<'a, Self> {
        let (input, _comment) = parse_comment(input)?;
        Self::parse(input)
    }

    fn from_str(input: &'a str) -> Self {
        Self::parse_ws(input).unwrap().1
    }
}

pub trait Combinator<O> {
    fn ws(self) -> O;
}

pub fn ws<'a, T>(f: impl Fn(&'a str) -> Res<'a, T>) -> impl Fn(&'a str) -> Res<'a, T> {
    move |input: &str| {
        let (input, _whitespace) =
            nom::bytes::complete::take_while(|c| c == ' ' || c == '\n')(input)?;
        f(input)
    }
}
