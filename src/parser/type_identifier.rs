use crate::{error::ParserError, lexer::token::TokenKind};

use super::{primitive_type::PrimitiveType, Node};

#[derive(Debug, PartialEq)]
pub enum TypeIdentifier {
    Nothing,
    Primitive(PrimitiveType),
    Complex(String),
}

impl Node for TypeIdentifier {
    fn parse(input: super::ParseInput) -> super::ParseResult<Self> {
        match input.get(0) {
            Some(TokenKind::Identifier(identifier)) => {
                Self::parse_type_identifier(&input, identifier)
            }
            Some(token) => Err(ParserError::InvalidToken(token.to_owned())),
            None => Err(ParserError::NoTokenFound),
        }
    }
}

impl TypeIdentifier {
    fn parse_type_identifier<'a>(
        input: super::ParseInput<'a>,
        identifier: &'a String,
    ) -> super::ParseResult<'a, Self> {
        match PrimitiveType::parse(input) {
            Ok((tail, primitive)) => Ok((tail, Self::Primitive(primitive))),
            Err(ParserError::UnknownPrimitive(_)) => {
                Ok((&input[1..], Self::Complex(identifier.to_owned())))
            }
            Err(e) => Err(e),
        }
    }
}
