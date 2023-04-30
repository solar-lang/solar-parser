use crate::{ast::keywords, parse::*, util::from_to};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentifierPath<'a> {
    pub span: &'a str,
    pub value: Vec<Identifier<'a>>,
}

impl<'a> Parse<'a> for IdentifierPath<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use crate::ast::keywords::Dot;
        use nom::{multi::many0, sequence::preceded};

        // TODO this can be done nicer
        let (rest, first) = Identifier::parse(input)?;
        let (rest, path) = many0(preceded(Dot::parse_ws, Identifier::parse_ws))(rest)?;
        let span = unsafe { from_to(input, rest) };

        // make an array of the first and the following paths
        let value = std::iter::once(first).chain(path.into_iter()).collect();

        Ok((rest, IdentifierPath { span, value }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Identifier<'a> {
    pub span: &'a str,
    pub value: &'a str,
}

impl<'a> PartialOrd<&str> for Identifier<'a> {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<'a> PartialEq<&str> for Identifier<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.value == *other
    }
}

fn isalpha(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase()
}

fn isnumber(c: char) -> bool {
    c.is_ascii_digit()
}

impl<'a> Parse<'a> for Identifier<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        // Accept & as a valid identifier
        if let Ok((rest, value)) = keywords::Identity::parse(input) {
            let value = value.span;
            return Ok((rest, Identifier { span: value, value }));
        }

        use nom::bytes::complete::{take_while, take_while1};
        use nom::combinator::{recognize, verify};
        use nom::sequence::pair;

        /// extended requirements to make an identifier solar compliant
        fn verify_ident(value: &str) -> bool {
            // identifiers may not be keywords
            !(is_keyword(value) ||

            // may not end with underscore
             value.ends_with('_') ||

            // may not contain double underscores __.
             value.contains("__"))
        }

        let firstpart = take_while1(isalpha);
        let secondpart = take_while(|c| isalpha(c) || isnumber(c) || c == '_');
        let (rest, value) = verify(recognize(pair(firstpart, secondpart)), verify_ident)(input)?;

        Ok((rest, Identifier { value, span: value }))
    }
}

pub fn is_keyword(word: &str) -> bool {
    [
        "and", "break", "do", "else", "false", "for", "generic", "if", "in", "is", "let", "loop",
        "mut", "next", "or", "pub", "return", "test", "then", "true", "type", "use", "when",
        "async", "await", "fun", "where",
    ]
    .contains(&word)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keyword_recognition() {
        assert!(is_keyword("let"));
        assert!(is_keyword("in"));
        assert!(is_keyword("for"));
        assert!(is_keyword("use"));
        assert!(is_keyword("type"));
        assert!(!is_keyword("x"));
        assert!(!is_keyword("y"));
        assert!(!is_keyword("point"));
    }

    #[test]
    fn idents() {
        let span = "hello.world 7";
        let res = Identifier::parse(span);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0, ".world 7");
        assert_eq!(res.1.value, "hello");
    }

    #[test]
    fn fullidents() {
        let span = "hello.world 7";
        let res = IdentifierPath::parse(span);

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0, " 7");
        assert_eq!(res.1.span, "hello.world");
    }
}
