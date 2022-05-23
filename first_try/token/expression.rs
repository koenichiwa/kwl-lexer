use anyhow::{Error, Result};

use crate::error::LexerError;

use super::{
    operator::{BinaryOperator, UnaryOperator},
    term::TermType,
    TokenType,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ExpressionType {
    Block(Box<ExpressionType>),
    Binary(TermType, BinaryOperator, Box<ExpressionType>),
    Unary(UnaryOperator, Box<ExpressionType>),
    Term(TermType),
}

impl TokenType for ExpressionType {
    fn parse(input: &str) -> Result<(Self, &str)> {
        Self::choose(
            input,
            vec![
                &Self::parse_block_expression,
                &Self::parse_unary_expression,
                &Self::parse_binary_expression,
                &Self::parse_term,
            ],
        )

        // if let Ok(result) = ExpressionType::parse_block_expression(input) {
        //     Ok(result)
        // } else if let Ok(result) = ExpressionType::parse_unary_expression(input) {
        //     Ok(result)
        // } else if let Ok(result) = ExpressionType::parse_binary_expression(input) {
        //     Ok(result)
        // } else if let Ok((token, string)) = TermType::parse(input) {
        //     Ok((ExpressionType::Term(token), string))
        // } else {
        //     Err(Error::new(LexerError::matches("Expression", input)))
        // }
    }
}

impl ExpressionType {
    fn parse_block_expression(input: &str) -> anyhow::Result<(Self, &str)> {
        let trimmed_input = input.trim_start();
        if !trimmed_input.starts_with('(') {
            return Err(Error::new(LexerError::matches("BlockExpression", input)));
        }
        let mut count = 0;

        for (index, char) in trimmed_input.char_indices() {
            if char == ')' {
                count -= 1;
            }
            if char == '(' {
                count += 1;
            }

            if count == 0 {
                let (expression, rest) = ExpressionType::parse(&trimmed_input[1..index])?;
                return if !rest.trim().is_empty() {
                    Err(Error::new(LexerError::matches(
                        "Rest Checking BlockExpression",
                        rest,
                    )))
                } else {
                    Ok((
                        ExpressionType::Block(expression.into()),
                        &input[index + 1..],
                    ))
                };
            }
        }
        Err(Error::new(LexerError::matches(
            "Parens Checking BlockExpression",
            trimmed_input,
        )))
    }

    fn parse_binary_expression(input: &str) -> anyhow::Result<(Self, &str)> {
        TermType::parse(input).and_then(|(term_token, rest)| {
            BinaryOperator::parse(rest).and_then(|(operator_token, rest)| {
                ExpressionType::parse(rest).map(|(expression_token, rest)| {
                    (
                        ExpressionType::Binary(term_token, operator_token, expression_token.into()),
                        rest,
                    )
                })
            })
        })
    }

    fn parse_unary_expression(input: &str) -> anyhow::Result<(Self, &str)> {
        UnaryOperator::parse(input).and_then(|(operator_token, rest)| {
            ExpressionType::parse(rest).map(|(expression_token, rest)| {
                (
                    ExpressionType::Unary(operator_token, expression_token.into()),
                    rest,
                )
            })
        })
    }

    fn parse_term(input: &str) -> anyhow::Result<(Self, &str)> {
        TermType::parse(input).map(|(token, tail)| (ExpressionType::Term(token), tail))
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn block() {
        todo!()
    }

    #[test]
    fn binary() {
        todo!()
    }

    #[test]
    fn unary() {
        todo!()
    }

    #[test]
    fn term() {
        todo!()
    }
}