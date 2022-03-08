use crate::{ast::identifier::Identifier, ast::*, parse::*, util::*};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import<'a> {
    pub span: &'a str,
    /// If true, find the import in some library, otherwise imported from the projects root file
    pub is_lib: bool,
    /// Path pointing to where to find the code
    /// e.g. collection.array
    pub path: Vec<Identifier<'a>>,
    /// Items that are supposed to be imported from the path
    /// e.g.    (Array, sort, binarySearch)
    ///         *
    pub items: Selection<'a>,
}

impl<'a> Parse<'a> for Import<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use nom::combinator::opt;

        let (rest, _) = keywords::Use::parse(input)?;

        let (rest, is_lib) = opt(keywords::At::parse_ws)(rest)?;
        let is_lib = is_lib.is_some();

        let (rest, path) = joined_by(Identifier::parse_ws, keywords::Dot::parse_ws)(rest)?;

        let (rest, items) = Selection::parse_ws(rest)?;

        let span = unsafe { from_to(input, rest) };

        Ok((
            rest,
            Import {
                span,
                is_lib,
                path,
                items,
            },
        ))
    }
}

/// Selection of imported items
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Selection<'a> {
    All,
    Items(Vec<Identifier<'a>>),
}

impl<'a> Parse<'a> for Selection<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        use nom::branch::alt;
        use nom::combinator::map;

        let all = map(keywords::Star::parse, |_| Selection::All);

        let items = |input| {
            let (rest, _) = keywords::ParenOpen::parse(input)?;
            let (rest, items) = joined_by(Identifier::parse_ws, keywords::Comma::parse_ws)(rest)?;
            let (rest, _) = keywords::ParenClose::parse_ws(rest)?;
            Ok((rest, Selection::Items(items)))
        };

        alt((all, items))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports() {
        let input = "use std.collections (hashmap, vector, util)  ";
        let imports = Import::parse(input);
        assert!(imports.is_ok());
        let (rest, imports) = imports.unwrap();
        assert_eq!(rest, "  ");
        assert_eq!(imports.span, &input[..(input.len() - 2)]);
    }

    // testing full imports with `use` keyword at the start
    #[test]
    fn full_imports() {
        let input = "use @std.collections.hashmap * ";
        let (rest, import) = Import::parse(input).unwrap();
        assert_eq!(
            import,
            Import {
                span: &input[..(input.len()-1)],
                is_lib: true,
                path: "std.collections.hashmap"
                    .split('.')
                    .map(|value| Identifier { span: value, value })
                    .collect(),
                items: Selection::All,
            }
        );
        assert_eq!(rest, " ");
    }
}
