use anyhow::{Error, Result};
use regex::Regex;

use crate::error::LexerError;

use super::{
    atom::EqualsAtom,
    expression::ExpressionType,
    identifier::{IdentifierDefinitionType, IdentifierType},
    TokenType,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum AssignmentType {
    Definition(IdentifierDefinitionType, EqualsAtom, ExpressionType),
    Reassignment(IdentifierType, EqualsAtom, ExpressionType),
}

impl TokenType for AssignmentType {
    fn parse(input: &str) -> anyhow::Result<(Self, &str)> {
        if let Ok(result) = Self::parse_definition(input) {
            Ok(result)
        } else if let Ok(result) = Self::parse_reassignment(input) {
            Ok(result)
        } else {
            Err(Error::new(LexerError::matches("Assignment", input)))
        }
    }
}

impl AssignmentType {
    fn parse_assignment(input: &str) -> Result<((EqualsAtom, ExpressionType), &str)> {
        Self::chain(input, EqualsAtom::parse, ExpressionType::parse)
    }

    fn parse_reassignment(input: &str) -> Result<(Self, &str)> {
        Self::chain(input, IdentifierType::parse, Self::parse_assignment).map(
            |((identifier_token, (equals_atom, expression_token)), tail)| {
                (
                    AssignmentType::Reassignment(identifier_token, equals_atom, expression_token),
                    tail,
                )
            },
        )

        // let (identifier_token, tail) = IdentifierType::parse(input)?;
        // Self::parse_assignment(tail).map(|(equals_atom, expression_token, rest)| {
        //     (
        //         AssignmentType::Reassignment(identifier_token, equals_atom, expression_token),
        //         rest,
        //     )
        // })
    }

    fn parse_definition(input: &str) -> anyhow::Result<(Self, &str)> {
        IdentifierDefinitionType::parse(input).and_then(|(identifier_token, tail)| {
            Self::parse_assignment(tail).map(|((equals_atom, expression_token), rest)| {
                (
                    AssignmentType::Definition(identifier_token, equals_atom, expression_token),
                    rest,
                )
            })
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::{token::{assignment::AssignmentType, atom::EqualsAtom, literal::LiteralType, term::TermType, expression::ExpressionType, r#type::{TypeType, BaseType}, identifier::{IdentifierDefinitionType, IdentifierType}, TokenType}};

    #[test]
    fn definition() {
        assert_eq!(AssignmentType::parse(r#"let a : i64 = 1"#).unwrap(),
            (
                AssignmentType::Definition(
                    IdentifierDefinitionType::TypedIdentifier(
                        "a".to_string(), 
                        TypeType::Base(BaseType::I64)
                    ), 
                    EqualsAtom {}, 
                    ExpressionType::Term(
                        TermType::Literal(
                            LiteralType::Integer(1)
                        )
                    )
                ), 
                ""
            )
        )
    }

    #[test]
    fn reassignment() {
        assert_eq!(AssignmentType::parse(r#"a = 1"#).unwrap(),
            (
                AssignmentType::Reassignment(
                    IdentifierType::Identifier(
                        "a".to_string()
                    ), 
                    EqualsAtom {}, 
                    ExpressionType::Term(
                        TermType::Literal(
                            LiteralType::Integer(1)
                        )
                    )
                ), 
                ""
            )
        )
    }
}   