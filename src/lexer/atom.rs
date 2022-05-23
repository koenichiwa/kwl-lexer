use nom::{
    branch::alt, bytes::complete::tag, character::complete::line_ending, combinator::value, IResult, multi::many1,
};

use super::token::TokenKind;

pub fn parse_atom(input: &str) -> IResult<&str, TokenKind> {
    alt((
        value(TokenKind::Arrow, tag("->")),
        value(TokenKind::LEq, tag("<=")),
        value(TokenKind::ParenthesesOpen, tag("(")),
        value(TokenKind::ParenthesesClosed, tag(")")),
        value(TokenKind::AccoladeOpen, tag("{")),
        value(TokenKind::AccoladeClosed, tag("}")),
        value(TokenKind::Colon, tag(":")),
        value(TokenKind::SemiColon, tag(";")),
        value(TokenKind::Comma, tag(",")),
        value(TokenKind::Plus, tag("+")),
        value(TokenKind::Minus, tag("-")),
        value(TokenKind::AssignmentToken, tag("=")),
        value(TokenKind::NewLine, (line_ending)),
    ))(input)
}
