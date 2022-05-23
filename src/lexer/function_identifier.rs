use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, peek, recognize},
    multi::many0_count,
    sequence::{pair, terminated},
    IResult,
};

use super::token::TokenKind;

pub fn parse_function_identifier(input: &str) -> IResult<&str, TokenKind> {
    map(
        terminated(
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0_count(alt((alphanumeric1, tag("_")))),
            )),
            peek(tag("(")),
        ),
        |cc: &str| TokenKind::FunctionIdentifier(cc.to_string()),
    )(input)
}