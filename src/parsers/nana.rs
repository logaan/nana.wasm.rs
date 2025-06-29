use std::sync::Arc;

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

pub fn tagged_tuple(input: &str) -> IResult<&str, LexicalExpression> {
    // TODO: Allow tagging tagged_tuples
    let taggable = alt((value_name, keyword, list, macro_name, string));
    tuple((taggable, char('('), many0(expression), char(')')))
        .map(|(expr, _, args, _)| LexicalExpression::TaggedTuple(Arc::new(expr), args.into()))
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

pub fn keyword(input: &str) -> IResult<&str, LexicalExpression> {
    tuple((char(':'), lower_start_word))
        .map(|(_, name)| LexicalExpression::Keyword(name))
        .parse(input)
}

pub fn hole(input: &str) -> IResult<&str, LexicalExpression> {
    char('_').map(|_| LexicalExpression::Hole).parse(input)
}

pub fn expression(input: &str) -> IResult<&str, LexicalExpression> {
    let expressions = alt((
        comment,
        tagged_tuple,
        keyword,
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
