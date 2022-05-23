use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::space0,
    combinator::value,
    sequence::{pair, preceded, tuple},
    IResult,
};

pub fn parse_comment(input: &str) -> IResult<&str, ()> {
    preceded(space0, alt((parse_inline_comment, parse_eol_comment)))(input)
}
fn parse_inline_comment(input: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away.
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
    )(input)
}

fn parse_eol_comment(input: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away.
        pair(tag("//"), is_not("\r\n")),
    )(input)
}
