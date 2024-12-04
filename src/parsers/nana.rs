use std::fmt::Debug;

use super::general::*;
use nom::{
    branch::alt, character::complete::multispace0, combinator::map, multi::many0,
    sequence::delimited, IResult,
};

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
    many0(delimited(
        multispace0,
        alt((macro_name, value_name)),
        multispace0,
    ))(input)
}
