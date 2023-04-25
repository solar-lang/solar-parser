use crate::{
    parse::Res,
};

pub unsafe fn from_to<'a>(start: &'a str, end: &'a str) -> &'a str {
    // TODO implement safety measures. Panic
    let length = end.as_ptr() as usize - start.as_ptr() as usize;

    let bytes = std::slice::from_raw_parts(start.as_ptr(), length);

    std::str::from_utf8_unchecked(bytes)
}

/// applies a parser and in between a separator parser.
/// Allows trailing separator at the end (long as at least one successfull parse has been applied
pub fn joined_by<'a, I, T>(
    parser: impl Fn(&'a str) -> Res<'a, I>,
    separator: impl Fn(&'a str) -> nom::IResult<&'a str, T>,
) -> impl Fn(&'a str) -> Res<'a, Vec<I>> {
    move |input: &'a str| {
        let mut res: Vec<I> = Vec::new();

        // apply the parse a first time
        let step1 = parser(input);

        // if it didn't work, return an empty array
        if step1.is_err() {
            return Ok((input, res));
        }

        let (mut rest, elem) = step1.unwrap();
        res.push(elem);

        loop {
            rest = match (separator)(rest) {
                Ok((new_rest, _)) => new_rest,
                _ => break,
            };

            match parser(rest) {
                Ok((new_rest, elem)) => {
                    res.push(elem);
                    rest = new_rest;
                }
                _ => break,
            }
        }

        Ok((rest, res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join() {
        use nom::character::complete::char;
        let input = "1,1,1";
        let (rest, result) = joined_by(char('1'), char(','))(input).unwrap();
        assert_eq!(result, vec!['1', '1', '1']);
        assert_eq!(rest, "");
    }

    #[test]
    fn join0() {
        use nom::character::complete::char;
        let input = "1,1,1,";
        let (rest, result) = joined_by(char('1'), char(','))(input).unwrap();
        assert_eq!(result, vec!['1', '1', '1']);
        assert_eq!(rest, "");
    }

    #[test]
    fn join1() {
        use nom::character::complete::char;
        let input = "1,1,1,,";
        let (rest, result) = joined_by(char('1'), char(','))(input).unwrap();
        assert_eq!(result, vec!['1', '1', '1']);
        assert_eq!(rest, ",");
    }

    #[test]
    fn join_will_only_take_sep_after_item() {
        use nom::character::complete::char;
        let input = ",";
        let (rest, result) = joined_by(char('1'), char(','))(input).unwrap();
        assert_eq!(rest, ",");
        assert_eq!(result, vec![]);
    }
}

