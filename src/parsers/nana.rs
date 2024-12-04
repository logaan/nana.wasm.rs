use std::fmt::Debug;

use super::general::*;
use nom::{branch::alt, combinator::map, multi::many0, IResult};

#[derive(PartialEq, Debug)]
pub enum Expression {
    MacroName(String),
    ValueName(String),
}

pub fn macro_name(input: &str) -> IResult<&str, Expression> {
    map(titlecase_word, |name| {
        Expression::MacroName(name) as Expression
    })(input)
}

pub fn value_name(input: &str) -> IResult<&str, Expression> {
    map(lower_start_word, |name| {
        Expression::ValueName(name) as Expression
    })(input)
}

pub fn program(input: &str) -> IResult<&str, Vec<Expression>> {
    many0(alt((macro_name, value_name)))(input)
}
