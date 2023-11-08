pub mod ast;
pub mod comment;
pub(crate) mod util;

mod parse;
pub use ast::Ast;
pub use parse::Parse;
pub use util::from_to;

#[cfg(test)]
mod syntax_tests {
    use super::*;

    #[test]
    fn hello_world_programm() {
        let input = include_str!("../syntax-tests/abc.sol");

        let result = Ast::from_source_code(input);

        let ast = result.expect("To parse Ast");
    }

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

            let result = Ast::from_source_code(&content);

            let _ast = result.expect("To parse Ast");
        }
    }
}
