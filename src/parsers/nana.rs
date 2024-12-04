use std::fmt::Debug;

use super::general::*;
use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    multi::many0,
    sequence::delimited,
    IResult, Parser,
};

#[derive(PartialEq, Debug)]
pub enum Expression {
    MacroName(String),
    ValueName(String),
    List(Vec<Expression>),
    Hole,
}

pub fn macro_name(input: &str) -> IResult<&str, Expression> {
    titlecase_word.map(Expression::MacroName).parse(input)
}

pub fn value_name(input: &str) -> IResult<&str, Expression> {
    lower_start_word.map(Expression::ValueName).parse(input)
}

pub fn list(input: &str) -> IResult<&str, Expression> {
    delimited(char('['), many0(expression), char(']'))
        .map(Expression::List)
        .parse(input)
}

pub fn hole(input: &str) -> IResult<&str, Expression> {
    char('_').map(|_| Expression::Hole).parse(input)
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    let expressions = alt((macro_name, value_name, hole, list));
    delimited(multispace0, expressions, multispace0).parse(input)
}

pub fn program(input: &str) -> IResult<&str, Vec<Expression>> {
    many0(expression).parse(input)
}
