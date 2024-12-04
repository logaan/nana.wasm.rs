use super::general::*;
use nom::{combinator::map, IResult};

pub struct MacroName {
    name: String,
}

pub fn macro_name(input: &str) -> IResult<&str, MacroName> {
    map(titlecase_word, |name| MacroName { name })(input)
}

pub struct ValueName {
    name: String,
}

pub fn value_name(input: &str) -> IResult<&str, ValueName> {
    map(lower_start_word, |name| ValueName { name })(input)
}
