use std::fmt::Debug;
use std::sync::Arc;

use super::general::*;
use nom::{branch::alt, combinator::map, multi::many0, IResult};

#[derive(PartialEq, Debug)]
pub enum Expression {
    MacroName(String),
    ValueName(String),
}

pub fn macro_name(input: &str) -> IResult<&str, Arc<Expression>> {
    map(titlecase_word, |name| {
        Arc::new(Expression::MacroName(name)) as Arc<Expression>
    })(input)
}

pub fn value_name(input: &str) -> IResult<&str, Arc<Expression>> {
    map(lower_start_word, |name| {
        Arc::new(Expression::ValueName(name)) as Arc<Expression>
    })(input)
}

pub fn program(input: &str) -> IResult<&str, Vec<Arc<Expression>>> {
    many0(alt((macro_name, value_name)))(input)
}
