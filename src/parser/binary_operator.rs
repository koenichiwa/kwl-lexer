use crate::{lexer::token::TokenKind, error::ParserError};

use super::Node;

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    LEq,
}

impl Node for BinaryOperator {
    fn parse(input: &[crate::lexer::token::TokenKind]) -> super::ParseResult<Self> {
        match input.get(0) {
            Some(TokenKind::Minus) => Ok((&input[1..], BinaryOperator::Minus)),
            Some(TokenKind::Plus) => Ok((&input[1..], BinaryOperator::Plus)),
            Some(TokenKind::LEq) => Ok((&input[1..], BinaryOperator::LEq)),
            Some(token) => Err(ParserError::InvalidToken(token.to_owned())),
            None => Err(ParserError::NoTokenFound),
        }
    }
}