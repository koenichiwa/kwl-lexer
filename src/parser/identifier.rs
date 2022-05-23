use crate::{error::ParserError, lexer::token::TokenKind};

use super::Node;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub id: String,
}

impl Node for Identifier {
    fn parse(input: super::ParseInput) -> super::ParseResult<Self> {
        match input.get(0) {
            Some(TokenKind::FunctionIdentifier(id)) => {
                Ok((&input[1..], Self { id: id.to_owned() }))
            } //TODO maybe don't lose this information?
            Some(TokenKind::Identifier(id)) => Ok((&input[1..], Self { id: id.to_owned() })),
            Some(token) => Err(ParserError::InvalidToken(token.clone())),
            None => Err(ParserError::NoTokenFound),
        }
    }
}
