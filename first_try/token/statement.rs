use super::{
    assignment::AssignmentType, 
    TokenType
};

pub enum StatementType {
    Assignment(AssignmentType)
}

impl TokenType for StatementType {
    fn parse(input: &str) -> anyhow::Result<(Self, &str)> {
        if let Ok((assignment_token, rest)) = AssignmentType::parse(input) {
            Ok(Assignment(assignment_token), rest)
        } else {
            
        }
    }
}

