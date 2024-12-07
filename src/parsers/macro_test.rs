use std::collections::HashMap;

use super::macros::build_macros;
use super::nana::program;

use super::nana::Expression;
use super::nana::Expression::MacroCall;

pub fn create_macro_map() -> HashMap<String, Expression> {
    let mut macros = HashMap::new();

    macros.insert(
        String::from("Package"),
        Expression::Macro("Package".to_string(), vec!["name".to_string()], vec![]),
    );

    macros.insert(
        String::from("World"),
        Expression::Macro(
            "World".to_string(),
            vec!["name".to_string(), "body".to_string()],
            vec![],
        ),
    );

    macros.insert(
        String::from("Import"),
        Expression::Macro("Import".to_string(), vec!["name".to_string()], vec![]),
    );

    macros.insert(
        String::from("Export"),
        Expression::Macro(
            "Export".to_string(),
            vec![
                "name".to_string(),
                "args".to_string(),
                "return_type".to_string(),
            ],
            vec![],
        ),
    );

    macros.insert(
        String::from("Func"),
        Expression::Macro(
            "Func".to_string(),
            vec![
                "name".to_string(),
                "args".to_string(),
                "return_type".to_string(),
                "body".to_string(),
            ],
            vec![],
        ),
    );

    macros.insert(
        String::from("Match"),
        Expression::Macro(
            "Match".to_string(),
            vec!["condition".to_string(), "branches".to_string()],
            vec![],
        ),
    );

    macros.insert(
        String::from("Let"),
        Expression::Macro(
            "Let".to_string(),
            vec!["bindings".to_string(), "body".to_string()],
            vec![],
        ),
    );

    macros.insert(
        String::from("For"),
        Expression::Macro(
            "For".to_string(),
            vec!["binding".to_string(), "body".to_string()],
            vec![],
        ),
    );

    macros
}

#[test]
fn parses_basic_macro() {
    let result =
        program("Package \"foo\"").and_then(|(_, es)| Ok(build_macros(es, create_macro_map())));
    assert_eq!(
        Ok((
            MacroCall(
                "Package".to_string(),
                vec![Expression::String("foo".to_string())],
            ),
            vec![],
        )),
        result
    );
}

#[test]
fn parses_nested_macros() {
    let result = program("Package Package \"foo\"")
        .and_then(|(_, es)| Ok(build_macros(es, create_macro_map())));
    assert_eq!(
        Ok((
            MacroCall(
                "Package".to_string(),
                vec![MacroCall(
                    "Package".to_string(),
                    vec![Expression::String("foo".to_string())],
                )],
            ),
            vec![],
        )),
        result
    );
}

#[test]
fn parses_macros_in_lists() {
    assert_eq!(
        Ok((
            Expression::List(vec![
                Expression::Number(1),
                MacroCall(
                    "Package".to_string(),
                    vec![MacroCall(
                        "Package".to_string(),
                        vec![Expression::String("two".to_string())],
                    )],
                ),
                Expression::Number(3)
            ]),
            vec![],
        )),
        program("[1 Package Package \"two\" 3]")
            .and_then(|(_, es)| Ok(build_macros(es, create_macro_map())))
    )
}

#[test]
fn parses_macros_in_args_to_functions() {
    assert_eq!(
        Ok((
            Expression::FunctionCall(
                "println".to_string(),
                vec![
                    Expression::Number(1),
                    MacroCall(
                        "Package".to_string(),
                        vec![MacroCall(
                            "Package".to_string(),
                            vec![Expression::String("two".to_string())],
                        )],
                    ),
                    Expression::Number(3)
                ]
            ),
            vec![],
        )),
        program("println(1 Package Package \"two\" 3)")
            .and_then(|(_, es)| Ok(build_macros(es, create_macro_map())))
    )
}
