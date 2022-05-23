use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{peek, recognize, value},
    sequence::terminated,
    IResult,
};

use super::token::TokenKind;

pub fn parse_keyword(input: &str) -> IResult<&str, TokenKind> {
    alt((parse_simple_keyword, parse_if, parse_else))(input)
}

fn parse_simple_keyword(input: &str) -> IResult<&str, TokenKind> {
    terminated(
        alt((
            value(TokenKind::DefinitionToken, tag("let")),
            value(TokenKind::ReturnToken, tag("ret")),
        )),
        space1,
    )(input)
}

fn parse_if(input: &str) -> IResult<&str, TokenKind> {
    terminated(
        value(TokenKind::IfToken, tag("if")),
        alt((peek(recognize(tag("("))), space1)),
    )(input)
}

fn parse_else(input: &str) -> IResult<&str, TokenKind> {
    terminated(
        value(TokenKind::ElseToken, tag("else")),
        alt((peek(recognize(tag("{"))), space1)),
    )(input)
}
