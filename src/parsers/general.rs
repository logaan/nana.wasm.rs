use std::char;

use nom::{
    branch::alt,
    character::complete::{one_of, satisfy},
    combinator::map,
    multi::many0,
    sequence::tuple,
    IResult,
};

pub fn uppercase_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_uppercase())(input)
}

pub fn lowercase_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_lowercase())(input)
}

fn nana_name_char(input: &str) -> IResult<&str, char> {
    alt((satisfy(|c| c.is_alphanumeric()), one_of(":/@-.<>")))(input)
}

pub fn nana_name0(input: &str) -> IResult<&str, String> {
    map(many0(nana_name_char), |chars| chars.iter().collect())(input)
}

pub fn titlecase_word(input: &str) -> IResult<&str, String> {
    map(tuple((uppercase_char, nana_name0)), |(first, rest)| {
        format!("{}{}", first, rest)
    })(input)
}

pub fn lower_start_word(input: &str) -> IResult<&str, String> {
    map(tuple((lowercase_char, nana_name0)), |(first, rest)| {
        format!("{}{}", first, rest)
    })(input)
}
