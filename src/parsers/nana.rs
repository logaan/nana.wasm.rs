use super::general::*;
use crate::expressions::LexicalExpression;
use im::Vector;
use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, none_of},
    multi::many0,
    sequence::{delimited, tuple},
    IResult, Parser,
};

pub fn comment(input: &str) -> IResult<&str, LexicalExpression> {
    delimited(char('#'), many0(none_of("\n")), char('\n'))
        .map(|_| LexicalExpression::Comment)
        .parse(input)
}

pub fn macro_name(input: &str) -> IResult<&str, LexicalExpression> {
    titlecase_word
        .map(LexicalExpression::MacroName)
        .parse(input)
}

pub fn value_name(input: &str) -> IResult<&str, LexicalExpression> {
    lower_start_word.map(LexicalExpression::Symbol).parse(input)
}

pub fn function_call(input: &str) -> IResult<&str, LexicalExpression> {
    tuple((lower_start_word, char('('), many0(expression), char(')')))
        .map(|(name, _, args, _)| LexicalExpression::TaggedTuple(name, args.into()))
        .parse(input)
}

pub fn list(input: &str) -> IResult<&str, LexicalExpression> {
    delimited(char('['), many0(expression), char(']'))
        .map(|v| LexicalExpression::List(v.into()))
        .parse(input)
}

pub fn number(input: &str) -> IResult<&str, LexicalExpression> {
    digit1
        .map(|s: &str| s.parse().unwrap())
        .map(LexicalExpression::Number)
        .parse(input)
}

pub fn string(input: &str) -> IResult<&str, LexicalExpression> {
    delimited(char('"'), many0(none_of("\"")), char('"'))
        .map(|chars| chars.iter().collect())
        .map(LexicalExpression::String)
        .parse(input)
}

pub fn hole(input: &str) -> IResult<&str, LexicalExpression> {
    char('_').map(|_| LexicalExpression::Hole).parse(input)
}

pub fn expression(input: &str) -> IResult<&str, LexicalExpression> {
    let expressions = alt((
        comment,
        function_call,
        hole,
        list,
        macro_name,
        number,
        string,
        value_name,
    ));
    delimited(multispace0, expressions, multispace0).parse(input)
}

pub fn program(input: &str) -> IResult<&str, Vector<LexicalExpression>> {
    many0(expression).map(|v| v.into()).parse(input)
}
