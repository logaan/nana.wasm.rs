#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest as Command;

use nom::{
    character::complete::{alpha0, satisfy},
    combinator::map,
    sequence::tuple,
    IResult,
};

fn uppercase_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_uppercase())(input)
}

fn titlecase_word(input: &str) -> IResult<&str, String> {
    map(tuple((uppercase_char, alpha0)), |(first, rest)| {
        format!("{}{}", first, rest)
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_uppercase() {
        assert_eq!(uppercase_char("F"), Ok(("", 'F')));
        assert_eq!(uppercase_char("Foo"), Ok(("oo", 'F')));
        assert_eq!(titlecase_word("Foo"), Ok(("", "Foo".to_string())));
        assert_eq!(titlecase_word("Foo Bar"), Ok((" Bar", "Foo".to_string())));
        assert_eq!(
            titlecase_word("foo Bar"),
            Err(nom::Err::Error(nom::error::Error {
                input: "foo Bar",
                code: nom::error::ErrorKind::Satisfy
            }))
        );
    }
}

struct Component;

impl Command for Component {
    fn run() -> Result<(), ()> {
        println!("Hello world");
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
