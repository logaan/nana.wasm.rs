use super::general::*;
use nom::{combinator::map, IResult};

#[derive(PartialEq, Debug)]
pub struct MacroName {
    pub name: String,
}

pub fn macro_name(input: &str) -> IResult<&str, MacroName> {
    map(titlecase_word, |name| MacroName { name })(input)
}

#[derive(PartialEq, Debug)]
pub struct ValueName {
    pub name: String,
}

pub fn value_name(input: &str) -> IResult<&str, ValueName> {
    map(lower_start_word, |name| ValueName { name })(input)
}
