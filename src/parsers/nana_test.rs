use super::nana::*;

#[test]
fn test_macro_name() {
    assert_eq!(
        macro_name("Hello"),
        Ok((
            "",
            MacroName {
                name: "Hello".to_string()
            }
        ))
    );
    assert_eq!(
        macro_name("HelloWorld"),
        Ok((
            "",
            MacroName {
                name: "HelloWorld".to_string()
            }
        ))
    );
    assert!(macro_name("hello").is_err());
    assert!(macro_name("123Hello").is_err());
}

#[test]
fn test_value_name() {
    assert_eq!(
        value_name("hello"),
        Ok((
            "",
            ValueName {
                name: "hello".to_string()
            }
        ))
    );
    assert_eq!(
        value_name("helloWorld"),
        Ok((
            "",
            ValueName {
                name: "helloWorld".to_string()
            }
        ))
    );
    assert!(value_name("Hello").is_err());
    assert!(value_name("123hello").is_err());
}
