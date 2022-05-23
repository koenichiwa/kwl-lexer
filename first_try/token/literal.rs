use anyhow::Result;

use super::TokenType;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LiteralType {
    String(String),
    Integer(u64),
    Float(f64),
}

impl TokenType for LiteralType {
    fn parse(input: &str) -> Result<(LiteralType, &str)> {
        Self::choose(
            input,
            vec![
                &LiteralType::parse_float,
                &LiteralType::parse_integer,
                &LiteralType::parse_string,
            ],
        )

        // if let Ok(float_token) = LiteralType::parse_float(input) {
        //     Ok(float_token)
        // } else if let Ok(integer_token) = LiteralType::parse_integer(input) {
        //     Ok(integer_token)
        // } else if let Ok(string_token) = LiteralType::parse_string(input) {
        //     Ok(string_token)
        // } else {
        //     Err(Error::new(LexerError::default()))
        // }
    }
}

impl LiteralType {
    fn parse_integer(input: &str) -> Result<(LiteralType, &str)> {
        Self::try_parse_from_pattern(input, r"^\s*(\d+)", "IntegerLiteral", |s| {
            Ok(LiteralType::Integer(s.parse()?))
        })
    }

    fn parse_float(input: &str) -> Result<(LiteralType, &str)> {
        Self::try_parse_from_pattern(input, r"^\s*(\d+\.\d+)", "FloatLiteral", |s| {
            Ok(LiteralType::Float(s.parse()?))
        })
    }

    fn parse_string(input: &str) -> Result<(LiteralType, &str)> {
        Self::parse_from_pattern(input, r#"\s*"([^"]*)""#, "StringLiteral", |s| {
            LiteralType::String(s.to_string())
        })
    }
}
