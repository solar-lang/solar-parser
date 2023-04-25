use std::iter::repeat_with;

use nom::{multi::separated_list1, combinator::cut};

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

        let (rest, path) = separated_list1(keywords::Dot::parse_ws, Identifier::parse_ws)(rest)?;

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
    This,
    Items(Vec<Identifier<'a>>),
}

impl<'a> Parse<'a> for Selection<'a> {
    fn parse(input: &'a str) -> Res<'a, Self> {
        if let Ok((rest, _)) = keywords::Spread::parse(input) {
            return Ok((rest, Selection::All));
        }

        if let Ok((rest, _)) = keywords::Dot::parse(input) {
            let (rest, _) = cut(keywords::ParenOpen::parse_ws)(rest)?;
            let (rest, items) = joined_by(Identifier::parse_ws, keywords::Comma::parse_ws)(rest)?;
            let (rest, _) = cut(keywords::ParenClose::parse_ws)(rest)?;

            return Ok((rest, Selection::Items(items)));
        }

        Ok((input, Selection::This))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports() {
        let input = "use collections.(hashmap, vector, util)  ";
        let imports = Import::parse(input);
        assert!(imports.is_ok());
        let (rest, imports) = imports.unwrap();
        assert_eq!(rest, "  ");
        assert_eq!(imports.span, &input[..(input.len() - 2)]);
    }

    // testing full imports with `use` keyword at the start
    #[test]
    fn full_imports() {
        let input = "use @std.collections.hashmap.. ";
        let (rest, import) = Import::parse(input).unwrap();
        assert_eq!(
            import,
            Import {
                span: input.trim_end(),
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

    #[test]
    fn list_imports() {
        let input = "use @std.(io)";
        let (rest, import) = Import::parse(input).unwrap();
        assert_eq!(
            import,
            Import {
                span: input,
                is_lib: true,
                path: "std"
                    .split('.')
                    .map(|value| Identifier { span: value, value })
                    .collect(),
                items: Selection::Items(vec![Identifier::parse("io").unwrap().1]),
            }
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn this_imports() {
        let input = "use @std.io";
        let (rest, import) = Import::parse(input).unwrap();
        assert_eq!(
            import,
            Import {
                span: input,
                is_lib: true,
                path: "std.io"
                    .split('.')
                    .map(|value| Identifier { span: value, value })
                    .collect(),
                items: Selection::This,
            }
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn complex_imports() {
        let input = [
            "use std.collections.(
        HashMap,
        BTreeMap,
        Array,   
    )",
            "use std.(io, collections, networking)",
            "use std.collections.HashMap",
            "use std..",
            "use std.io..",
            "use std.debug..",
        ];

        for i in input {
            let (rest, _import) = Import::parse_ws(i).unwrap();

            assert_eq!(rest, "", "expect to parse '{i}' without rest");
        }
    }
}
