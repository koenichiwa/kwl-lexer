use std::collections::VecDeque;

use crate::{error::ParserError, lexer::token::TokenKind};

use super::{
    identifier::Identifier, type_identifier::TypeIdentifier, typed_variable::TypedVariable, Node,
};

#[derive(Debug, PartialEq)]
pub struct FunctionHead {
    input_variables: Vec<TypedVariable>,
    output_type: TypeIdentifier,
}

impl Node for FunctionHead {
    fn parse(input: super::ParseInput) -> super::ParseResult<Self> {
        let (tail, identifiers) = Self::get_identifiers(input)?;
        let (tail, input_variables) = Self::get_typed_variables(tail, identifiers)?;

        // First case: completely empty "main() {...}"
        match tail.get(0) {
            Some(TokenKind::Arrow) => {}
            Some(TokenKind::AccoladeOpen) | Some(TokenKind::AssignmentToken) => {
                if input_variables.is_empty() {
                    return Ok((
                        tail,
                        FunctionHead {
                            input_variables,
                            output_type: TypeIdentifier::Nothing,
                        },
                    ));
                } else {
                    todo!()
                }
            }
            Some(token) => return Err(ParserError::InvalidToken(token.to_owned())),
            None => return Err(ParserError::NoTokenFound),
        }

        // Second case: only return type "fib() -> u64 {...}"
        // Third case: typed "fib() : ; -> u64 {...}"
        let tail = &tail[1..];
        match tail.get(0) {
            Some(TokenKind::SemiColon) => Ok((
                &tail[1..],
                FunctionHead {
                    input_variables,
                    output_type: TypeIdentifier::Nothing,
                },
            )),
            Some(TokenKind::Identifier(_)) => {
                let (tail, output_type) = TypeIdentifier::parse(tail)?;
                Ok((
                    tail,
                    FunctionHead {
                        input_variables,
                        output_type,
                    },
                ))
            }
            Some(token) => return Err(ParserError::InvalidToken(token.to_owned())),
            None => return Err(ParserError::NoTokenFound),
        }
    }
}

impl FunctionHead {
    fn get_identifiers(area: super::ParseInput) -> super::ParseResult<VecDeque<Identifier>> {
        match area.get(0) {
            Some(TokenKind::ParenthesesOpen) => {}
            Some(token) => return Err(ParserError::InvalidToken(token.to_owned())),
            None => return Err(ParserError::NoTokenFound),
        }
        let mut slice = &area[1..];
        let mut identifiers = VecDeque::<Identifier>::new();
        loop {
            match slice.get(0) {
                Some(TokenKind::ParenthesesClosed) => return Ok((&slice[1..], identifiers)),
                Some(TokenKind::Identifier(_)) => {
                    let (tail, identifier) = Identifier::parse(slice)?;
                    slice = tail;
                    identifiers.push_back(identifier);
                }
                Some(TokenKind::Comma) => {
                    slice = &slice[1..];
                }
                Some(token) => return Err(ParserError::InvalidToken(token.to_owned())),
                None => return Err(ParserError::NoTokenFound),
            }
        }
    }

    fn get_typed_variables(
        area: super::ParseInput,
        mut identifiers: VecDeque<Identifier>,
    ) -> super::ParseResult<Vec<TypedVariable>> {
        // First case: completely empty "main() {...}"
        // Second case: only return type "fib() -> i64"
        match area.get(0) {
            Some(TokenKind::Colon) => {}
            Some(TokenKind::AccoladeOpen)
            | Some(TokenKind::AssignmentToken)
            | Some(TokenKind::Arrow) => {
                return if identifiers.is_empty() {
                    Ok((area, Vec::new()))
                } else {
                    todo!()
                }
            }
            Some(token) => return Err(ParserError::InvalidToken(token.to_owned())),
            None => return Err(ParserError::NoTokenFound),
        }
        let area = &area[1..];

        // Third case: typed but empty "main() : ; -> ..."
        match area.get(0) {
            Some(TokenKind::ParenthesesOpen) => {}
            Some(TokenKind::SemiColon) => {
                return if identifiers.is_empty() {
                    Ok((&area[1..], Vec::new()))
                } else {
                    todo!()
                }
            }
            Some(token) => return Err(ParserError::InvalidToken(token.to_owned())),
            None => return Err(ParserError::NoTokenFound),
        }

        // Fourth case: typed "fib(a) : u64 -> ..."
        let mut slice = &area[1..];
        let mut type_variables = Vec::<TypedVariable>::new();
        loop {
            match slice.get(0) {
                Some(TokenKind::ParenthesesClosed) => {
                    return if identifiers.is_empty() {
                        Ok((&slice[1..], type_variables))
                    } else {
                        todo!()
                    }
                }
                Some(TokenKind::Identifier(_)) => {
                    let (tail, type_identifier) = TypeIdentifier::parse(slice)?;
                    slice = tail;
                    if let Some(identifier) = identifiers.pop_front() {
                        type_variables.push(TypedVariable {
                            identifier,
                            type_identifier,
                        });
                    } else {
                        todo!()
                    }
                }
                Some(TokenKind::Comma) => {
                    slice = &slice[1..];
                }
                Some(token) => return Err(ParserError::InvalidToken(token.to_owned())),
                None => return Err(ParserError::NoTokenFound),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer, parser::Node};

    use super::FunctionHead;

    #[test]
    fn simple_head() {
        let tokens = lexer::parse("(iden) : (type) -> type").unwrap();
        let result = FunctionHead::parse(&tokens);
        println!("{result:#?}");
    }

    #[test]
    fn double_head() {
        let tokens = lexer::parse("(A,B) : (a,b) -> type").unwrap();
        let result = FunctionHead::parse(&tokens);
        println!("{result:#?}");
    }

    #[test]
    fn nothing_head() {
        let tokens = lexer::parse("() : ; -> ;").unwrap();
        let result = FunctionHead::parse(&tokens);
        println!("{result:#?}");
    }

    #[test]
    fn empty_head() {
        let tokens = lexer::parse("() {}").unwrap();
        let result = FunctionHead::parse(&tokens);
        println!("{result:#?}");
    }

    #[test]
    fn only_return_head() {
        let tokens = lexer::parse("() -> i64").unwrap();
        let result = FunctionHead::parse(&tokens);
        println!("{result:#?}");
    }
}
