use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::{map_res, opt, recognize},
    sequence::{preceded, tuple},
    IResult,
};

use super::integer::decimal;

pub fn float_value(input: &str) -> IResult<&str, f64> {
    map_res(float, |out: &str| str::replace(out, "_", "").parse())(input)
}

fn float(input: &str) -> IResult<&str, &str> {
    alt((
        // Case one: .42
        recognize(tuple((
            char('.'),
            decimal,
            opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
        ))), // Case two: 42e42 and 42.42e42
        recognize(tuple((
            decimal,
            opt(preceded(char('.'), decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal,
        ))), // Case three: 42. and 42.42
        recognize(tuple((decimal, char('.'), opt(decimal)))),
    ))(input)
}
