pub mod definition;
pub mod expression;
pub mod expression_tree;
pub mod function_body;
pub mod function_head;
pub mod identifier;
pub mod literal;
pub mod primitive_type;
pub mod root;
pub mod statement;
pub mod term;
pub mod type_identifier;
pub mod typed_variable;
pub mod binary_operator;
pub mod unary_operator;
mod util;

pub use util::{ParseInput, ParseResult};

use crate::{error::ParserError, lexer::token::TokenKind};

use self::root::Root;

pub fn parse(tokens: &[TokenKind]) -> Result<Root, ParserError> {
    Root::parse(tokens).map(|(_, root)| root)
}

pub trait Node
where
    Self: Sized,
{
    fn parse(input: &[TokenKind]) -> ParseResult<Self>;
}
