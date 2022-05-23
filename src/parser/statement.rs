use crate::{error::ParserError, lexer::token::TokenKind};

use super::{definition::Definition, expression::Expression, identifier::Identifier, Node};

#[derive(Debug, PartialEq)]
pub enum Statement {
    DefinitionStatement(Definition),
    Assignment(Identifier, Expression),
    ExpressionStatement(Expression),
}

impl Node for Statement {
    fn parse(input: &[TokenKind]) -> Result<(&[TokenKind], Self), ParserError> {
        match input.get(0) {
            Some(TokenKind::DefinitionToken) => Self::parse_definition(input),
            Some(TokenKind::Identifier(_)) => Self::parse_expression_or_assignment(input),
            Some(token) => Err(ParserError::InvalidToken(token.clone())),
            None => Err(ParserError::NoTokenFound),
        }
    }
}

//TODO: Cleanup. Match with 2 is smart but not clean
// impl Node for Statement {
//     fn parse(input: &[TokenKind]) -> Result<(&[TokenKind], Self), ParserError> {
//         match (input.get(0), input.get(1)) {
//             (Some(TokenKind::DefinitionToken), Some(_)) => Self::parse_definition(input),
//             (Some(TokenKind::Identifier(_)), Some(TokenKind::AssignmentToken)) => {
//                 Self::parse_assignment(input)
//             }
//             (Some(TokenKind::Identifier(_)), Some(_)) => Self::parse_expression(input),
//             (Some(token), _) => Err(ParserError::InvalidToken(token.clone())),
//             (None, _) => Err(ParserError::NoTokenFound),
//         }
//     }
// }

impl Statement {
    fn parse_expression_or_assignment(input: super::ParseInput) -> super::ParseResult<Self> {
        match input.get(1) {
            Some(TokenKind::AssignmentToken) => Self::parse_assignment(input),
            Some(_) => Self::parse_expression(input),
            None => Err(ParserError::NoTokenFound),
        }
    }

    fn parse_definition(input: &[TokenKind]) -> Result<(&[TokenKind], Self), ParserError> {
        if let Some(TokenKind::DefinitionToken) = input.get(0) {
            let (tail, definition) = Definition::parse(&input[1..])?;
            Ok((tail, Self::DefinitionStatement(definition)))
        } else {
            Err(ParserError::InvalidToken(input[0].clone()))
        }
    }

    fn parse_assignment(input: &[TokenKind]) -> Result<(&[TokenKind], Self), ParserError> {
        let (tail, identifier) = Identifier::parse(input)?;
        if let Some(TokenKind::AssignmentToken) = tail.get(0) {
            let (tail, expression) = Expression::parse(&tail[1..])?;
            Ok((tail, Self::Assignment(identifier, expression)))
        } else {
            Err(ParserError::InvalidToken(tail[0].clone()))
        }
    }

    fn parse_expression(input: &[TokenKind]) -> Result<(&[TokenKind], Self), ParserError> {
        let (tail, token) = Expression::parse(input)?;
        Ok((tail, Self::ExpressionStatement(token)))
    }
}
