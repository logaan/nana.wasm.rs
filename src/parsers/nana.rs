use std::fmt::Debug;

use super::general::*;
use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, none_of},
    multi::many0,
    sequence::{delimited, tuple},
    IResult, Parser,
};

#[derive(PartialEq, Debug)]
pub enum Expression {
    Macro(String, Vec<String>, Vec<Expression>),
    MacroName(String),
    ValueName(String),
    FunctionCall(String, Vec<Expression>),
    List(Vec<Expression>),
    Number(u8),
    String(String),
    Hole,
}

pub fn macro_name(input: &str) -> IResult<&str, Expression> {
    titlecase_word.map(Expression::MacroName).parse(input)
}

pub fn value_name(input: &str) -> IResult<&str, Expression> {
    lower_start_word.map(Expression::ValueName).parse(input)
}

pub fn function_call(input: &str) -> IResult<&str, Expression> {
    tuple((lower_start_word, char('('), many0(expression), char(')')))
        .map(|(name, _, args, _)| Expression::FunctionCall(name, args))
        .parse(input)
}

pub fn list(input: &str) -> IResult<&str, Expression> {
    delimited(char('['), many0(expression), char(']'))
        .map(Expression::List)
        .parse(input)
}

pub fn number(input: &str) -> IResult<&str, Expression> {
    digit1
        .map(|s: &str| s.parse().unwrap())
        .map(Expression::Number)
        .parse(input)
}

pub fn string(input: &str) -> IResult<&str, Expression> {
    delimited(char('"'), many0(none_of("\"")), char('"'))
        .map(|chars| chars.iter().collect())
        .map(Expression::String)
        .parse(input)
}

pub fn hole(input: &str) -> IResult<&str, Expression> {
    char('_').map(|_| Expression::Hole).parse(input)
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    let expressions = alt((
        macro_name,
        function_call,
        value_name,
        hole,
        number,
        string,
        list,
    ));
    delimited(multispace0, expressions, multispace0).parse(input)
}

pub fn program(input: &str) -> IResult<&str, Vec<Expression>> {
    many0(expression).parse(input)
}
