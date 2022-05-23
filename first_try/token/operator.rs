use anyhow::{Error, Result};

use regex::Regex;

use crate::error::LexerError;

use super::TokenType;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum BinaryOperator {
    Plus,
    Minus,
}

impl TokenType for BinaryOperator {
    fn parse(input: &str) -> Result<(Self, &str)> {
        if let Some(m) = Regex::new(r"^\s*\+").unwrap().find(input) {
            Ok((BinaryOperator::Plus, &input[m.end()..]))
        } else if let Some(m) = Regex::new(r"^\s*-").unwrap().find(input) {
            Ok((BinaryOperator::Minus, &input[m.end()..]))
        } else {
            Err(Error::new(LexerError::matches("Binary Operator", input)))
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum UnaryOperator {
    Minus,
}

impl TokenType for UnaryOperator {
    fn parse(input: &str) -> Result<(Self, &str)> {
        if let Some(m) = Regex::new(r"^\s*-").unwrap().find(input) {
            Ok((UnaryOperator::Minus, &input[m.end()..]))
        } else {
            Err(Error::new(LexerError::matches("Unary -", input)))
        }
    }
}
