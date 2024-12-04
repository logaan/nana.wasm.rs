use nom::{
    character::complete::{alpha0, satisfy},
    combinator::map,
    sequence::tuple,
    IResult,
};

pub fn uppercase_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_uppercase())(input)
}

pub fn lowercase_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_lowercase())(input)
}

pub fn titlecase_word(input: &str) -> IResult<&str, String> {
    map(tuple((uppercase_char, alpha0)), |(first, rest)| {
        format!("{}{}", first, rest)
    })(input)
}

pub fn lower_start_word(input: &str) -> IResult<&str, String> {
    map(tuple((lowercase_char, alpha0)), |(first, rest)| {
        format!("{}{}", first, rest)
    })(input)
}
