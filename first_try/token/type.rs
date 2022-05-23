use anyhow::{Error, Result};
use regex::Regex;

use crate::error::LexerError;

use super::TokenType;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum TypeType {
    Base(BaseType),
    Complex(String),
}

impl TokenType for TypeType {
    fn parse(input: &str) -> Result<(Self, &str)> {
        Self::choose(input, vec![&Self::parse_base, &Self::parse_complex])
    }
}

impl TypeType {
    fn parse_complex(input: &str) -> Result<(Self, &str)> {
        Self::parse_from_pattern(input, r"^\s*:\s*([[:alpha:]][[:word:]]*)", "Type", |i| {
            TypeType::Complex(i.to_string())
        })
    }

    fn parse_base(input: &str) -> Result<(Self, &str)> {
        BaseType::parse(input).map(|(bt, tail)| (Self::Base(bt), tail))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BaseType {
    I64,
    F64,
    Bool,
}

impl TokenType for BaseType {
    fn parse(input: &str) -> Result<(Self, &str)> {
        Self::choose(input, vec![&Self::parse_i64])
    }
}

impl BaseType {
    fn parse_i64(input: &str) -> Result<(Self, &str)> {
        let type_name = "I64";
        Regex::new(r"^\s*:\s*i64")
            .expect(format!("{type_name} Regex failed to compile").as_str())
            .find(input)
            .ok_or_else(|| Error::new(LexerError::matches(type_name, input)))
            .map(|m| (BaseType::I64, &input[m.as_str().len()..]))
    }
}
