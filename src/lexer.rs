mod atom;
mod comment;
mod function_identifier;
mod identifier;
mod keyword;
mod literal;
pub mod token;

use nom::{
    branch::alt,
    character::complete::space0,
    combinator::{all_consuming, value},
    multi::{many0, many1},
    sequence::preceded,
    Finish, IResult,
};

use self::{
    atom::parse_atom, comment::parse_comment, function_identifier::parse_function_identifier,
    identifier::parse_identifier, keyword::parse_keyword, literal::parse_literal, token::TokenKind,
};

pub(crate) fn parse(input: &str) -> Result<Vec<TokenKind>, nom::error::Error<&str>> {
    all_consuming(parse_tokens)(input)
        .finish()
        .map(|(_, tokens)| tokens)
}

fn parse_tokens(input: &str) -> IResult<&str, Vec<TokenKind>> {
    many0(alt((
        value(Vec::<TokenKind>::new(), many1(parse_comment)),
        many1(parse_token),
    )))(input)
    .map(|(tail, tokenvecs)| (tail, tokenvecs.into_iter().flatten().collect()))
}

fn parse_token(input: &str) -> IResult<&str, TokenKind> {
    preceded(
        space0,
        alt((
            parse_keyword,
            parse_function_identifier,
            parse_identifier,
            parse_literal,
            parse_atom,
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::lexer::TokenKind;

    use super::{parse, parse_tokens};

    #[test]
    fn identifier() {
        assert_eq!(
            parse_tokens("n"),
            Ok(("", vec![TokenKind::Identifier("n".to_string())]))
        )
    }

    #[test]
    fn simple_assignment() {
        let result = Ok(vec![
            TokenKind::DefinitionToken,
            TokenKind::Identifier("n".to_string()),
            TokenKind::Colon,
            TokenKind::Identifier("i64".to_string()),
            TokenKind::AssignmentToken,
            TokenKind::FunctionIdentifier("fib".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::IntegerLiteral(5),
            TokenKind::ParenthesesClosed,
        ]);
        assert_eq!(parse("let n : i64 = fib(5)"), result);
    }

    #[test]
    fn fib_program() {
        let result = Ok(vec![
            TokenKind::DefinitionToken,
            TokenKind::FunctionIdentifier("main".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::ParenthesesClosed,
            TokenKind::AccoladeOpen,
            TokenKind::NewLine,
            TokenKind::DefinitionToken,
            TokenKind::Identifier("n".to_string()),
            TokenKind::Colon,
            TokenKind::Identifier("i64".to_string()),
            TokenKind::AssignmentToken,
            TokenKind::FunctionIdentifier("fib".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::IntegerLiteral(5),
            TokenKind::ParenthesesClosed,
            TokenKind::NewLine,
            TokenKind::AccoladeClosed,
            TokenKind::NewLine,
            TokenKind::NewLine,
            TokenKind::DefinitionToken,
            TokenKind::FunctionIdentifier("fib".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::Identifier("n".to_string()),
            TokenKind::ParenthesesClosed,
            TokenKind::Colon,
            TokenKind::ParenthesesOpen,
            TokenKind::Identifier("i64".to_string()),
            TokenKind::ParenthesesClosed,
            TokenKind::Arrow,
            TokenKind::Identifier("i64".to_string()),
            TokenKind::AccoladeOpen,
            TokenKind::NewLine,
            TokenKind::ReturnToken,
            TokenKind::IfToken,
            TokenKind::Identifier("n".to_string()),
            TokenKind::LEq,
            TokenKind::IntegerLiteral(1),
            TokenKind::AccoladeOpen,
            TokenKind::NewLine,
            TokenKind::Identifier("n".to_string()),
            TokenKind::NewLine,
            TokenKind::AccoladeClosed,
            TokenKind::ElseToken,
            TokenKind::AccoladeOpen,
            TokenKind::NewLine,
            TokenKind::FunctionIdentifier("fib".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::Identifier("n".to_string()),
            TokenKind::Minus,
            TokenKind::IntegerLiteral(1),
            TokenKind::ParenthesesClosed,
            TokenKind::Plus,
            TokenKind::FunctionIdentifier("fib".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::Identifier("n".to_string()),
            TokenKind::Minus,
            TokenKind::IntegerLiteral(2),
            TokenKind::ParenthesesClosed,
            TokenKind::NewLine,
            TokenKind::AccoladeClosed,
            TokenKind::NewLine,
            TokenKind::AccoladeClosed,
        ]);
        assert_eq!(
            parse(
                r#"let main() {
            let n : i64 = fib(5)
        }
        
        let fib(n) : (i64) -> i64 {
            ret if n <= 1 {
                n
            } else {
                fib(n-1) + fib(n-2)
            }
        }"#
            ),
            result
        )
    }

    #[test]
    fn fib_program_with_comment() {
        let result = Ok(vec![
            TokenKind::DefinitionToken,
            TokenKind::FunctionIdentifier("main".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::ParenthesesClosed,
            TokenKind::AccoladeOpen,
            TokenKind::NewLine,
            TokenKind::DefinitionToken,
            TokenKind::Identifier("n".to_string()),
            TokenKind::Colon,
            TokenKind::Identifier("i64".to_string()),
            TokenKind::AssignmentToken,
            TokenKind::FunctionIdentifier("fib".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::IntegerLiteral(5),
            TokenKind::ParenthesesClosed,
            TokenKind::NewLine,
            TokenKind::AccoladeClosed,
            TokenKind::NewLine,
            TokenKind::NewLine,
            TokenKind::DefinitionToken,
            TokenKind::FunctionIdentifier("fib".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::Identifier("n".to_string()),
            TokenKind::ParenthesesClosed,
            TokenKind::Colon,
            TokenKind::ParenthesesOpen,
            TokenKind::Identifier("i64".to_string()),
            TokenKind::ParenthesesClosed,
            TokenKind::Arrow,
            TokenKind::Identifier("i64".to_string()),
            TokenKind::AccoladeOpen,
            TokenKind::NewLine,
            TokenKind::ReturnToken,
            TokenKind::IfToken,
            TokenKind::Identifier("n".to_string()),
            TokenKind::LEq,
            TokenKind::IntegerLiteral(1),
            TokenKind::AccoladeOpen,
            TokenKind::NewLine,
            TokenKind::Identifier("n".to_string()),
            TokenKind::NewLine,
            TokenKind::AccoladeClosed,
            TokenKind::ElseToken,
            TokenKind::AccoladeOpen,
            TokenKind::NewLine,
            TokenKind::FunctionIdentifier("fib".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::Identifier("n".to_string()),
            TokenKind::Minus,
            TokenKind::IntegerLiteral(1),
            TokenKind::ParenthesesClosed,
            TokenKind::Plus,
            TokenKind::FunctionIdentifier("fib".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::Identifier("n".to_string()),
            TokenKind::Minus,
            TokenKind::IntegerLiteral(2),
            TokenKind::ParenthesesClosed,
            TokenKind::NewLine,
            TokenKind::AccoladeClosed,
            TokenKind::NewLine,
            TokenKind::AccoladeClosed,
            TokenKind::NewLine,
            TokenKind::NewLine,
            TokenKind::DefinitionToken,
            TokenKind::FunctionIdentifier("print_int".to_string()),
            TokenKind::ParenthesesOpen,
            TokenKind::Identifier("n".to_string()),
            TokenKind::Comma,
            TokenKind::Identifier("m".to_string()),
            TokenKind::ParenthesesClosed,
            TokenKind::Colon,
            TokenKind::ParenthesesOpen,
            TokenKind::Identifier("i64".to_string()),
            TokenKind::Comma,
            TokenKind::Identifier("i64".to_string()),
            TokenKind::ParenthesesClosed,
            TokenKind::Arrow,
            TokenKind::SemiColon,
            TokenKind::AccoladeOpen,
            TokenKind::NewLine,
            TokenKind::NewLine,
            TokenKind::AccoladeClosed,
        ]);

        assert_eq!(
            parse(
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
        
        let print_int(n, m) : (i64, i64) -> ; {
            // no implementation
        }"#
            ),
            result
        )
    }
}
