use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, recognize},
    multi::many0_count,
    sequence::pair,
    IResult,
};

use super::token::TokenKind;

pub fn parse_identifier(input: &str) -> IResult<&str, TokenKind> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        )),
        |cc: &str| TokenKind::Identifier(cc.to_string()),
    )(input)
}
