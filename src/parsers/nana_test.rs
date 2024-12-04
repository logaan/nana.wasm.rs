use std::sync::Arc;

use super::nana::*;

#[test]
fn test_macro_name() {
    assert_eq!(
        macro_name("Hello"),
        Ok(("", Arc::new(Expression::MacroName("Hello".to_string()))))
    );
    assert_eq!(
        macro_name("Hello World"),
        Ok((
            " World",
            Arc::new(Expression::MacroName("Hello".to_string()))
        ))
    );
    assert_eq!(
        macro_name("Greetings::English/Hello-World"),
        Ok((
            "",
            Arc::new(Expression::MacroName(
                "Greetings::English/Hello-World".to_string()
            ))
        ))
    );
    assert!(macro_name("hello").is_err());
    assert!(macro_name("123Hello").is_err());
}

#[test]
fn test_value_name() {
    assert_eq!(
        value_name("hello"),
        Ok(("", Arc::new(Expression::ValueName("hello".to_string()))))
    );
    assert_eq!(
        value_name("greetings::english/hello-world"),
        Ok((
            "",
            Arc::new(Expression::ValueName(
                "greetings::english/hello-world".to_string()
            ))
        ))
    );
    assert!(value_name("Hello").is_err());
    assert!(value_name("123hello").is_err());
}
