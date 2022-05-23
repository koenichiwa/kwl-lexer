use anyhow::Result;

use crate::token::{assignment::AssignmentType, TokenType};

pub struct Lexer {}

impl Lexer {
    pub fn run(input: &str) -> Result<(AssignmentType, &str)> {
        AssignmentType::parse(input)
    }
}
