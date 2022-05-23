use crate::{error::ParserError, lexer::token::TokenKind};

use super::Node;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(u64),
    Float(f64),
    String(String),
}

impl Node for Literal {
    fn parse(input: super::ParseInput) -> super::ParseResult<Self> {
        match input.get(0) {
            Some(TokenKind::IntegerLiteral(int)) => Ok((&input[1..], Self::Integer(*int))),
            Some(TokenKind::FloatLiteral(float)) => Ok((&input[1..], Self::Float(*float))),
            Some(TokenKind::StringLiteral(string)) => {
                Ok((&input[1..], Self::String(string.to_owned())))
            }
            Some(token) => Err(ParserError::InvalidToken(token.to_owned())),
            None => Err(ParserError::NoTokenFound),
        }
    }
}
