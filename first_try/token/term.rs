use anyhow::{Error, Result};

use crate::error::LexerError;

use super::{identifier::IdentifierType, literal::LiteralType, TokenType};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum TermType {
    Literal(LiteralType),
    Identifier(IdentifierType),
}

impl TokenType for TermType {
    fn parse(input: &str) -> Result<(Self, &str)> {
        // Self::choose(input, vec![
        //     &IdentifierType::parse,
        //     &LiteralType::parse,
        // ])

        if let Ok((token, rest)) = IdentifierType::parse(input) {
            Ok((TermType::Identifier(token), rest))
        } else if let Ok((token, rest)) = LiteralType::parse(input) {
            Ok((TermType::Literal(token), rest))
        } else {
            Err(Error::new(LexerError::choose(vec![]))) // TODO
        }
    }
}
