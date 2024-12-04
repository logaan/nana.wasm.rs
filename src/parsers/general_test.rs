use super::general::*;

#[test]
fn parse_lowercase() {
    assert_eq!(lowercase_char("f"), Ok(("", 'f')));
    assert_eq!(lowercase_char("foo"), Ok(("oo", 'f')));
    assert_eq!(lower_start_word("foo"), Ok(("", "foo".to_string())));
    assert_eq!(lower_start_word("foo Bar"), Ok((" Bar", "foo".to_string())));
    assert!(lower_start_word("Foo Bar").is_err());
}

#[test]
fn parse_uppercase() {
    assert_eq!(uppercase_char("F"), Ok(("", 'F')));
    assert_eq!(uppercase_char("Foo"), Ok(("oo", 'F')));
    assert_eq!(titlecase_word("Foo"), Ok(("", "Foo".to_string())));
    assert_eq!(titlecase_word("Foo Bar"), Ok((" Bar", "Foo".to_string())));
    assert!(titlecase_word("foo Bar").is_err());
}
