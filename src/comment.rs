use nom::bytes::complete::take_while;

use crate::util::from_to;

/// Parses a comment from solar source code
pub fn parse_comment(input: &str) -> crate::parse::Res<'_, &str> {
    use nom::character::complete::char;
    let comment_start = input.trim_start();

    let mut cursor = comment_start;

    loop {
        match cursor.strip_prefix('#') {
            Some(rest) => {
                // take until new line
                // "comment" contains the _actuall_ comment
                let (rest, _comment) = take_while(|c| c != '\n')(rest)?;
                cursor = rest.trim_start();
            }
            None => {
                let comment = unsafe { from_to(comment_start, cursor) };

                return Ok((cursor, comment));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_comment() {
        let input = "
            # hey, this is a comment
            let x = 8
        ";
        let expected = "# hey, this is a comment
            ";

        let (_rest, comment) = super::parse_comment(input).unwrap();
        assert_eq!(comment, expected);
    }

    #[test]
    fn end_of_input() {
        let input = "
            # hey, this is a comment";
        let expected = "# hey, this is a comment";

        let (_rest, comment) = super::parse_comment(input).unwrap();
        assert_eq!(comment, expected);
    }
}
