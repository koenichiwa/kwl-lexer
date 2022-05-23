use crate::{error::ParserError, lexer::token::TokenKind};

use super::Node;

#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    I64,
    F64,
    U8,
}

impl Node for PrimitiveType {
    fn parse(input: super::ParseInput) -> super::ParseResult<Self> {
        match input.get(0) {
            Some(TokenKind::Identifier(name)) => Self::try_parse_primitive(input, name),
            Some(token) => Err(ParserError::InvalidToken(token.to_owned())),
            None => Err(ParserError::NoTokenFound),
        }
    }
}

impl PrimitiveType {
    fn try_parse_primitive<'a>(
        input: super::ParseInput<'a>,
        name: &str,
    ) -> super::ParseResult<'a, Self> {
        println!("Primitive: {input:?}");
        match name {
            "i64" => Ok((&input[1..], PrimitiveType::I64)),
            "f64" => Ok((&input[1..], PrimitiveType::F64)),
            "u8" => Ok((&input[1..], PrimitiveType::U8)),
            string => Err(ParserError::UnknownPrimitive(string.to_owned())),
        }
    }
}
