mod float;
mod integer;
mod string;

use nom::{branch::alt, combinator::map, IResult};

use self::{float::float_value, integer::parse_integer};

use super::TokenKind;

pub fn parse_literal(input: &str) -> IResult<&str, TokenKind> {
    alt((
        parse_float_literal,
        parse_integer_literal,
        parse_string_literal,
    ))(input)
}

fn parse_integer_literal(input: &str) -> IResult<&str, TokenKind> {
    map(parse_integer, TokenKind::IntegerLiteral)(input)
    // map(u64, |out|
    //     Token::IntegerLiteral(out)
    // )(input)
}

fn parse_string_literal(input: &str) -> IResult<&str, TokenKind> {
    map(string::parse_string, TokenKind::StringLiteral)(input)
}

fn parse_float_literal(input: &str) -> IResult<&str, TokenKind> {
    map(float_value, TokenKind::FloatLiteral)(input)
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::TokenKind;

    use super::{parse_float_literal, parse_string_literal};

    #[test]
    fn string_literal() {
        assert_eq!(
            parse_string_literal(r#""n""#),
            Ok(("", TokenKind::StringLiteral("n".to_string())))
        );
        assert!(parse_string_literal("n").is_err());
    }

    #[test]
    fn float_literal() {
        assert_eq!(
            parse_float_literal("0.567"),
            Ok(("", TokenKind::FloatLiteral(0.567)))
        )
    }
}
