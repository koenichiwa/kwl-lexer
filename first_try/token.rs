pub mod assignment;
pub mod atom;
pub mod expression;
pub mod identifier;
pub mod literal;
pub mod operator;
pub mod term;
pub mod r#type;
// pub mod statement;

use crate::error::LexerError;

use anyhow::{Error, Result};
use regex::Regex;
use std::fmt::Debug;

pub trait TokenType
where
    Self: Sized + Debug + PartialEq,
{
    fn parse(input: &str) -> Result<(Self, &str)>;

    fn parse_empty_from_pattern<'a, T>(
        input: &'a str,
        pattern: &'static str,
        type_name: &'static str,
        f: fn() -> T,
    ) -> TokenResult<'a, T> {
        Regex::new(pattern)
            .expect(format!("{type_name} Regex failed to compile").as_str())
            .find(input)
            .ok_or_else(|| Error::new(LexerError::matches(type_name, input)))
            .map(|m| (f(), &input[m.as_str().len()..]))
    }

    fn get_captures<'a>(
        input: &'a str,
        pattern: &'static str,
        type_name: &'static str,
    ) -> Result<regex::Captures<'a>, anyhow::Error> {
        // static PATTERN: OnceCell<Regex> = OnceCell::new();
        // PATTERN.get_or_init(|| {
        //     Regex::new(pattern)
        //         .expect(format!("{type_name} Regex failed to compile").as_str())
        // }).captures(input)
        //     .ok_or_else(|| Error::new(LexerError::matches(type_name, input)))

        Regex::new(pattern)
            .expect(format!("{type_name} Regex failed to compile").as_str())
            .captures(input)
            .ok_or_else(|| Error::new(LexerError::matches(type_name, input)))
    }

    fn parse_from_pattern<'a>(
        input: &'a str,
        pattern: &'static str,
        type_name: &'static str,
        f: fn(parsed: &str) -> Self,
    ) -> TokenResult<'a, Self> {
        Self::get_captures(input, pattern, type_name)
            .map(|cap| (f(&cap[1]), &input[cap[0].as_bytes().len()..]))
    }

    fn try_parse_from_pattern<'a>(
        input: &'a str,
        pattern: &'static str,
        type_name: &'static str,
        f: fn(parsed: &str) -> Result<Self>,
    ) -> TokenResult<'a, Self> {
        Self::get_captures(input, pattern, type_name)
            .and_then(|cap| Ok((f(&cap[1])?, &input[cap[0].as_bytes().len()..])))
    }

    fn choose<'a>(
        input: &'a str,
        functions: Vec<&dyn Fn(&'a str) -> Result<(Self, &'a str)>>,
    ) -> TokenResult<'a, Self> {
        let mut errs = Vec::new();
        for function in functions {
            match function(input) {
                Ok(result) => return Ok(result),
                Err(e) => errs.push(e),
            }
        }
        Err(Error::new(LexerError::choose(errs)))
    }

    fn chain<'a, F, G>(
        input: &'a str,
        f: fn(parsed: &str) -> Result<(F, &str)>,
        g: fn(parsed: &str) -> Result<(G, &str)>,
    ) -> Result<((F, G), &'a str)> {
        f(input).and_then(|(fres, tail)| g(tail).map(|(gres, rest)| ((fres, gres), rest)))
    }
}

type TokenResult<'a, T> = Result<(T,&'a str), anyhow::Error>;
