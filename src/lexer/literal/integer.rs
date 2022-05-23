use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many1},
    sequence::{preceded, terminated},
    IResult,
};

pub fn parse_integer(input: &str) -> IResult<&str, u64> {
    alt((hexadecimal_value, octal_value, binary_value, decimal_value))(input)
}

fn hexadecimal_value(input: &str) -> IResult<&str, u64> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        |out: &str| u64::from_str_radix(&str::replace(out, "_", ""), 16),
    )(input)
}

fn octal_value(input: &str) -> IResult<&str, u64> {
    map_res(
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(many1(terminated(one_of("01234567"), many0(char('_'))))),
        ),
        |out: &str| u64::from_str_radix(&str::replace(out, "_", ""), 8),
    )(input)
}

fn binary_value(input: &str) -> IResult<&str, u64> {
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(many1(terminated(one_of("01"), many0(char('_'))))),
        ),
        |out: &str| u64::from_str_radix(&str::replace(out, "_", ""), 8),
    )(input)
}

fn decimal_value(input: &str) -> IResult<&str, u64> {
    map_res(decimal, |out: &str| {
        u64::from_str_radix(&str::replace(out, "_", ""), 10)
    })(input)
}

pub(super) fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}
