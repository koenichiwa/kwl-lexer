#![feature(iterator_try_collect)]
use error::ParserError;
use lexer::token::TokenKind;
use parser::root::Root;

pub mod error;
pub mod lexer;
pub mod parser;

pub struct PipeLine<'a> {
    input: &'a str,
}

impl<'a> PipeLine<'a> {
    pub fn run(self) -> Result<Root, ParserError> {
        let tokens: &Vec<TokenKind> = &lexer::parse(self.input).unwrap();
        parser::parse(tokens)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer,
        parser::{
            self, definition::Definition, expression::Expression, identifier::Identifier,
            literal::Literal, root::Root, statement::Statement, term::Term,
        },
    };

    #[test]
    fn simple_function() {
        let tokens = lexer::parse(
            r#"let n : i64 = 5 + 4 - 3 + 2 <= 1"#
        ).unwrap();
        println!("{:#?}", parser::parse(tokens.as_slice()));
    }

    #[test]
    fn fib_program_with_comment() {
        let tokens = lexer::parse(
            r#"let main() {
                let n : i64 = fib(5)
            }
            
            let fib(n) : (i64) -> i64 {
                ret if n <= 1 {
                    n
                } else {
                    fib(n-1) + fib(n-2)
                }
            }
            
            let print_int(n) : (i64) -> ; {
                // no implementation
            }"#,
        )
        .unwrap();
        println!("{:#?}", parser::parse(tokens.as_slice()));
    }

    #[test]
    fn definition() {
        let tokens = lexer::parse(
            r"let a = 1
            let a = 2",
        )
        .unwrap();
        println!("{tokens:#?}");
        let result: Root = Root {
            statements: vec![
                Statement::DefinitionStatement(Definition::VariableDefinition(
                    Identifier {
                        id: "a".to_string(),
                    },
                    Expression::Simple(Term::Literal(Literal::Integer(1))),
                )),
                Statement::DefinitionStatement(Definition::VariableDefinition(
                    Identifier {
                        id: "a".to_string(),
                    },
                    Expression::Simple(Term::Literal(Literal::Integer(2))),
                )),
            ],
        };
        let tree = parser::parse(tokens.as_slice()).unwrap();

        assert_eq!(tree, result);
    }

    #[test]
    fn assignment() {
        let result = Root {
            statements: vec![Statement::Assignment(
                Identifier {
                    id: "a".to_string(),
                },
                Expression::Simple(Term::Literal(Literal::Float(1.2))),
            )],
        };
        let tokens = lexer::parse(r"a = 1.2").unwrap();
        let tree = parser::parse(tokens.as_slice()).unwrap();

        assert_eq!(tree, result);
    }
}
