use crate::{error::ParserError, lexer::token::TokenKind};

use super::{
    expression::Expression, function_body::FunctionBody, function_head::FunctionHead,
    identifier::Identifier, type_identifier::TypeIdentifier, Node,
};

#[derive(Debug, PartialEq)]
pub enum Definition {
    VariableDefinition(Identifier, Expression),
    TypedVariableDefinition(Identifier, TypeIdentifier, Expression),
    FunctionDefinition(Identifier, FunctionHead, FunctionBody),
}

impl Node for Definition {
    fn parse(input: super::ParseInput) -> super::ParseResult<Self> {
        match input.get(0) {
            Some(TokenKind::FunctionIdentifier(_)) => Self::parse_function_definition(input),
            Some(TokenKind::Identifier(_)) => Self::parse_definition(input),
            Some(token) => Err(ParserError::InvalidToken(token.to_owned())),
            None => Err(ParserError::NoTokenFound),
        }
    }
}

impl Definition {
    fn parse_definition(input: super::ParseInput) -> super::ParseResult<Self> {
        let (tail, identifier) = Identifier::parse(input)?;
        match tail.get(0) {
            Some(TokenKind::Colon) => Self::parse_typed_variable_definition(&tail[1..], identifier),
            Some(TokenKind::AssignmentToken) => {
                
                Self::parse_variable_definition(&tail[1..], identifier)
            }
            Some(token) => Err(ParserError::InvalidToken(token.to_owned())),
            None => Err(ParserError::NoTokenFound),
        }
    }

    fn parse_function_definition(
        input: &[TokenKind],
    ) -> Result<(&[TokenKind], Definition), ParserError> {
        let (tail, identifier) = Identifier::parse(input)?;
        let (tail, function_head) = FunctionHead::parse(tail)?;
        let (tail, function_body) = FunctionBody::parse(tail)?;
        Ok((
            tail,
            Self::FunctionDefinition(identifier, function_head, function_body),
        ))
    }

    fn parse_variable_definition(
        input: &[TokenKind],
        identifier: Identifier,
    ) -> Result<(&[TokenKind], Definition), ParserError> {
        let (tail, expression) = Expression::parse(input)?;
        Ok((tail, Self::VariableDefinition(identifier, expression)))
    }

    fn parse_typed_variable_definition(
        input: &[TokenKind],
        identifier: Identifier,
    ) -> Result<(&[TokenKind], Definition), ParserError> {
        let (tail, type_identifier) = TypeIdentifier::parse(input)?;
        println!("Definition:{tail:?}");
        let (tail, expression) = Expression::parse(&tail[1..])?;
        Ok((
            tail,
            Self::TypedVariableDefinition(identifier, type_identifier, expression),
        ))
    }
}
