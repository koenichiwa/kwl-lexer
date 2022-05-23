use anyhow::{Error, Result};

use crate::error::LexerError;

use super::{r#type::TypeType, TokenType};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum IdentifierType {
    Identifier(String),
}

impl TokenType for IdentifierType {
    fn parse(input: &str) -> Result<(Self, &str)> {
        Self::parse_from_pattern(input, r"^\s*([[:alpha:]][[:word:]]*)", "Identifier", |s| {
            IdentifierType::Identifier(s.to_string())
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum IdentifierDefinitionType {
    Identifier(String),
    TypedIdentifier(String, TypeType),
}

impl TokenType for IdentifierDefinitionType {
    fn parse(input: &str) -> Result<(Self, &str)> {
        if let Ok(result) = Self::parse_typed_identifier(input) {
            Ok(result)
        } else if let Ok(result) = Self::parse_identifier(input) {
            Ok(result)
        } else {
            Err(Error::new(LexerError::matches(
                "IdentifierDefinition",
                input,
            )))
        }
    }
}

impl IdentifierDefinitionType {
    fn parse_identifier(input: &str) -> Result<(Self, &str)> {
        Self::parse_from_pattern(
            input,
            r"^\s*let\s+([[:alpha:]][[:word:]]*)",
            "IdentifierDefinition",
            |s| IdentifierDefinitionType::Identifier(s.to_string()),
        )
    }

    fn parse_typed_identifier(input: &str) -> Result<(Self, &str)> {
        Self::get_captures(
            input,
            r"^\s*let\s+([[:alpha:]][[:word:]]*)",
            "TypedIdentifierDefinition",
        )
        .and_then(|cap| {
            TypeType::parse(&input[cap[0].as_bytes().len()..]).map(|(type_token, rest)| {
                let identifier_token = cap[1].to_string();
                (
                    IdentifierDefinitionType::TypedIdentifier(identifier_token, type_token),
                    rest,
                )
            })
        })
    }
}
