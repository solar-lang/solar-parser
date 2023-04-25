pub mod ast;
pub(crate) mod util;

mod parse;
pub use ast::Ast;
pub use parse::Parse;

#[cfg(test)]
mod syntax_tests {
    use super::*;

    #[test]
    fn all_files() {
        let testdir = "./syntax-tests";

        let entries = std::fs::read_dir(testdir).expect("to find directory with syntax test files");

        for e in entries {
            let e = e.expect("read dir entry");

            let filename = e.file_name().into_string().unwrap();

            if !filename.ends_with(".sol") {
                continue;
            }

            let content = std::fs::read_to_string(e.path())
                .unwrap_or_else(|_| panic!("read content of file: {}", filename));

            let result = Ast::parse_ws(&content);

            let (rest, _ast) = result.expect("To parse Ast");

            let rest = rest.trim();

            assert_eq!(rest, "", "expect parser to consume input");
        }
    }
}
