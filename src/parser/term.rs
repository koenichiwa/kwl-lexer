use crate::{error::ParserError, lexer::token::TokenKind};

use super::{identifier::Identifier, literal::Literal, Node, expression::Expression};

#[derive(Debug, PartialEq)]
pub enum Term {
    FunctionExcecution(Identifier, Vec<Expression>),
    Identifier(Identifier),
    Literal(Literal),
}

impl Node for Term {
    fn parse(input: super::ParseInput) -> super::ParseResult<Self> {
        match input.get(0) {
            Some(TokenKind::FunctionIdentifier(_)) => {
                Self::parse_function(input)
            }
            Some(TokenKind::Identifier(_)) => {
                let (tail, identifier) = Identifier::parse(input)?;
                Ok((tail, Self::Identifier(identifier)))
            }
            Some(TokenKind::FloatLiteral(_))
            | Some(TokenKind::IntegerLiteral(_))
            | Some(TokenKind::StringLiteral(_)) => {
                Literal::parse(input).map(|(tail, literal)| (tail, Self::Literal(literal)))
            }
            Some(token) => Err(ParserError::InvalidToken(token.clone())),
            None => Err(ParserError::NoTokenFound),
        }
    }
}

impl Term {
    fn parse_function(input: super::ParseInput) -> super::ParseResult<Self> {
        let (tail, identifier) = Identifier::parse(input)?;
        let (tail, input_areas) = find_area_parentheses(tail)?;
        let function_inputs = input_areas.iter()
            .map(|area| {
                let (tail, expression) = Expression::parse(area)?;
                if !tail.is_empty() {
                    todo!()
                }
                Ok(expression)
            })
            .try_collect()?;
        Ok((tail, Self::FunctionExcecution(identifier, function_inputs)))
    }
}

fn find_area_parentheses(input: super::ParseInput) -> super::ParseResult<Vec<super::ParseInput>> {
    println!("Find parentheses {input:?}");
    let mut count = 0;
    let mut cursor = 0;
    let mut start = 1;
    let mut areas = Vec::<super::ParseInput>::new();
    loop {
        match input.get(cursor) {
            Some(TokenKind::ParenthesesOpen) => count += 1,
            Some(TokenKind::ParenthesesClosed) => count -= 1,
            Some(TokenKind::Comma) if count == 1 => {
                areas.push(&input[start..cursor]);
                start = cursor+1;
            }
             Some(_) => {}
            None => return Err(ParserError::NoTokenFound),
        }
        if count == 0 {
            println!("Find parentheses returns {:?}", &input[1..cursor]);
            return Ok((&input[cursor+1..], areas));
        }
        if count < 0 {
            unreachable!();
        }
        cursor += 1;
    }
}