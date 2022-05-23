use crate::{error::ParserError, lexer::token::TokenKind};

use super::Node;

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Minus
}

impl Node for UnaryOperator {
    fn parse(input: &[crate::lexer::token::TokenKind]) -> super::ParseResult<Self> {
        match input.get(0) {
            Some(TokenKind::Minus) => Ok((&input[1..], UnaryOperator::Minus)),
            Some(token) => Err(ParserError::InvalidToken(token.to_owned())),
            None => Err(ParserError::NoTokenFound),
        }
    }
}