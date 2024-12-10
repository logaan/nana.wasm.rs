use super::nana::*;
use crate::expressions::LexicalExpression;

#[test]
fn test_macro_name() {
    assert_eq!(
        macro_name("Hello"),
        Ok(("", LexicalExpression::MacroName("Hello".to_string())))
    );
    assert_eq!(
        macro_name("Hello World"),
        Ok((" World", LexicalExpression::MacroName("Hello".to_string())))
    );
    assert_eq!(
        macro_name("Greetings::English/Hello-World"),
        Ok((
            "",
            LexicalExpression::MacroName("Greetings::English/Hello-World".to_string())
        ))
    );
    assert!(macro_name("hello").is_err());
    assert!(macro_name("123Hello").is_err());
}

#[test]
fn test_value_name() {
    assert_eq!(
        value_name("hello"),
        Ok(("", LexicalExpression::ValueName("hello".to_string())))
    );
    assert_eq!(
        value_name("greetings::english/hello-world"),
        Ok((
            "",
            LexicalExpression::ValueName("greetings::english/hello-world".to_string())
        ))
    );
    assert!(value_name("Hello").is_err());
    assert!(value_name("123hello").is_err());
}
