use crate::{error::ParserError, lexer::token::TokenKind};

use super::{statement::Statement, util::InputUtil, Node};

#[derive(Debug, PartialEq)]
pub struct Root {
    pub statements: Vec<Statement>,
}

impl Node for Root {
    fn parse(input: &[TokenKind]) -> Result<(&[TokenKind], Self), ParserError> {
        Self::parse_statements(input).map(|(output, statements)| (output, Root { statements }))
    }
}

impl Root {
    fn parse_statements(
        input: &[TokenKind],
    ) -> Result<(&[TokenKind], Vec<Statement>), ParserError> {
        if input.is_empty() {
            Ok((input, Vec::<Statement>::new()))
        } else {
            let mut statements = Vec::<Statement>::new();
            let mut cursor = input.trim_start(TokenKind::NewLine); // strip_newlines(input);
            while !cursor.is_empty() {
                let (tail, result) = Statement::parse(cursor)?;
                statements.push(result);
                cursor = tail.trim_start(TokenKind::NewLine);
            }
            Ok((cursor, statements))
        }
    }
}
