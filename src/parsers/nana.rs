use std::fmt::Debug;

use super::general::*;
use nom::{
    branch::alt, character::complete::multispace0, multi::many0, sequence::delimited, IResult,
    Parser,
};

#[derive(PartialEq, Debug)]
pub enum Expression {
    MacroName(String),
    ValueName(String),
}

pub fn macro_name(input: &str) -> IResult<&str, Expression> {
    titlecase_word.map(Expression::MacroName).parse(input)
}

pub fn value_name(input: &str) -> IResult<&str, Expression> {
    lower_start_word.map(Expression::ValueName).parse(input)
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    let expressions = alt((macro_name, value_name));
    delimited(multispace0, expressions, multispace0).parse(input)
}

pub fn program(input: &str) -> IResult<&str, Vec<Expression>> {
    many0(expression).parse(input)
}
