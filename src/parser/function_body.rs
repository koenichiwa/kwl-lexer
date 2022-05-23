use crate::{error::ParserError, lexer::token::TokenKind};

use super::{expression::Expression, statement::Statement, util::InputUtil, Node};

#[derive(Debug, PartialEq)]
pub enum FunctionBody {
    LineBody(Expression),
    BlockBody(Vec<Statement>),
}

impl Node for FunctionBody {
    fn parse(input: super::ParseInput) -> super::ParseResult<Self> {
        match input.get(0) {
            Some(TokenKind::AccoladeOpen) => {
                let (tail, statements) = Self::parse_statements(input)?;
                Ok((tail, Self::BlockBody(statements)))
            }
            Some(TokenKind::AssignmentToken) => {
                let (tail, expression) = Expression::parse(input)?;
                Ok((tail, Self::LineBody(expression)))
            }
            Some(token) => return Err(ParserError::InvalidToken(token.to_owned())),
            None => return Err(ParserError::NoTokenFound),
        }
    }
}

impl FunctionBody {
    fn parse_statements(
        input: &[TokenKind],
    ) -> Result<(&[TokenKind], Vec<Statement>), ParserError> {
        let area = Self::find_area(input)?;
        let area_length = area.len() + 2; // add starting and closing accolades back
        println!("AREA \n{area:?}");
        if area.is_empty() {
            Ok((input, Vec::<Statement>::new()))
        } else {
            let mut statements = Vec::<Statement>::new();
            let mut cursor = area.trim_start(TokenKind::NewLine); // strip_newlines(input);
            while !cursor.is_empty() {
                println!("THE CURSOR: {cursor:?}");
                let (tail, result) = Statement::parse(cursor)?;
                statements.push(result);
                cursor = tail.trim_start(TokenKind::NewLine);
            }
            Ok((&input[area_length..], statements))
        }
    }

    fn find_area(input: super::ParseInput) -> Result<super::ParseInput, ParserError> {
        let mut count = 0;
        let mut cursor = 0;
        loop {
            match input.get(cursor) {
                Some(TokenKind::AccoladeOpen) => count += 1,
                Some(TokenKind::AccoladeClosed) => count -= 1,
                Some(_) => {}
                None => return Err(ParserError::NoTokenFound),
            }
            if count == 0 {
                return Ok(&input[1..cursor]);
            }
            if count < 0 {
                todo!()
            }
            cursor += 1;
        }
    }
}
