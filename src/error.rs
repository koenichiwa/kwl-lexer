use thiserror::Error;

use crate::lexer::token::TokenKind;

// #[derive(Debug, Error, PartialEq)]
// pub enum LexerError<I> {
//     #[error("Invalid character found")]
//     InvalidCharacter(
//         #[source]
//         // #[backtrace] nom::error::Error<I>,
//         String,
//     ),
// }

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Invalid token found: {0:#?}")]
    InvalidToken(TokenKind),
    #[error("Incomplete code")]
    NoTokenFound,
    #[error("Unknown primitive: {0}")]
    UnknownPrimitive(String),
}

// TODO: Replace with this?

pub struct KWLError {
    tokenkind: (),
}
