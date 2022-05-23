use anyhow::Result;

use super::TokenType;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct EqualsAtom {}

impl TokenType for EqualsAtom {
    fn parse(input: &str) -> Result<(Self, &str)> {
        Self::parse_empty_from_pattern(input, r"^\s*=", "Equals", || Self {})
    }
}

#[derive(Debug, PartialEq)]
struct DefinitionAtom {}

impl TokenType for DefinitionAtom {
    fn parse(input: &str) -> Result<(Self, &str)> {
        Self::parse_empty_from_pattern(input, r"^\s*let", "Definition", || Self {})
    }
}
