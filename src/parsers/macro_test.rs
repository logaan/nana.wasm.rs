use im::hashmap;
use im::HashMap;

use im::vector;

use super::macros::build_macros;
use super::nana::program;

use super::macros::RuntimeExpression::{self, MacroCall};

pub fn create_macro_map() -> HashMap<String, RuntimeExpression> {
    hashmap! {
        String::from("Package") =>
        RuntimeExpression::Macro(
            "Package".to_string(),
            vector!["name".to_string()],
            vector![],
        ),

        String::from("World") =>
        RuntimeExpression::Macro(
            "World".to_string(),
            vector!["name".to_string(), "body".to_string()],
            vector![],
        ),

        String::from("Import") =>
        RuntimeExpression::Macro("Import".to_string(), vector!["name".to_string()], vector![]),

        String::from("Export") =>
        RuntimeExpression::Macro(
            "Export".to_string(),
            vector![
                "name".to_string(),
                "args".to_string(),
                "return_type".to_string(),
            ],
            vector![],
        ),

        String::from("Func") =>
        RuntimeExpression::Macro(
            "Func".to_string(),
            vector![
                "name".to_string(),
                "args".to_string(),
                "return_type".to_string(),
                "body".to_string(),
            ],
            vector![],
        ),

        String::from("Match") =>
        RuntimeExpression::Macro(
            "Match".to_string(),
            vector!["condition".to_string(), "branches".to_string()],
            vector![],
        ),

        String::from("Let") =>
        RuntimeExpression::Macro(
            "Let".to_string(),
            vector!["bindings".to_string(), "body".to_string()],
            vector![],
        ),

        String::from("For") =>
        RuntimeExpression::Macro(
            "For".to_string(),
            vector!["binding".to_string(), "body".to_string()],
            vector![],
        ),
    }
}

#[test]
fn parses_basic_macro() {
    let result =
        program("Package \"foo\"").and_then(|(_, es)| Ok(build_macros(&es, &create_macro_map())));
    assert_eq!(
        Ok((
            MacroCall(
                "Package".to_string(),
                vector![RuntimeExpression::String("foo".to_string())],
            ),
            vector![],
        )),
        result
    );
}

#[test]
fn parses_nested_macros() {
    let result = program("Package Package \"foo\"")
        .and_then(|(_, es)| Ok(build_macros(&es, &create_macro_map())));
    assert_eq!(
        Ok((
            MacroCall(
                "Package".to_string(),
                vector![MacroCall(
                    "Package".to_string(),
                    vector![RuntimeExpression::String("foo".to_string())],
                )],
            ),
            vector![],
        )),
        result
    );
}

#[test]
fn parses_macros_in_lists() {
    assert_eq!(
        Ok((
            RuntimeExpression::List(vector![
                RuntimeExpression::Number(1),
                MacroCall(
                    "Package".to_string(),
                    vector![MacroCall(
                        "Package".to_string(),
                        vector![RuntimeExpression::String("two".to_string())],
                    )],
                ),
                RuntimeExpression::Number(3)
            ]),
            vector![],
        )),
        program("[1 Package Package \"two\" 3]")
            .and_then(|(_, es)| Ok(build_macros(&es, &create_macro_map())))
    )
}

#[test]
fn parses_macros_in_args_to_functions() {
    assert_eq!(
        Ok((
            RuntimeExpression::FunctionCall(
                "println".to_string(),
                vector![
                    RuntimeExpression::Number(1),
                    MacroCall(
                        "Package".to_string(),
                        vector![MacroCall(
                            "Package".to_string(),
                            vector![RuntimeExpression::String("two".to_string())],
                        )],
                    ),
                    RuntimeExpression::Number(3)
                ]
            ),
            vector![],
        )),
        program("println(1 Package Package \"two\" 3)")
            .and_then(|(_, es)| Ok(build_macros(&es, &create_macro_map())))
    )
}
