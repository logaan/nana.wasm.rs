use std::sync::Arc;

use im::hashmap;

use im::vector;

use super::macros::build_macros;
use super::nana::program;

use crate::expressions::Environment;
use crate::expressions::RuntimeExpression::{self, MacroCall};

pub fn create_env_with_macros() -> Environment {
    Environment::from(hashmap! {
        String::from("Package") =>
        RuntimeExpression::Macro(
            vector!["name".to_string()],
            Environment::new(),
            vector![],
        ),

        String::from("World") =>
        RuntimeExpression::Macro(
            vector!["name".to_string(), "body".to_string()],
            Environment::new(),
            vector![],
        ),

        String::from("Import") =>
        RuntimeExpression::Macro(
            vector!["name".to_string()],
            Environment::new(),
            vector![],
        ),

        String::from("Export") =>
        RuntimeExpression::Macro(
            vector![
                "name".to_string(),
                "args".to_string(),
                "return_type".to_string(),
            ],
            Environment::new(),
            vector![],
        ),

        String::from("Func") =>
        RuntimeExpression::Macro(
            vector![
                "name".to_string(),
                "args".to_string(),
                "return_type".to_string(),
                "body".to_string(),
            ],
            Environment::new(),
            vector![],
        ),

        String::from("Match") =>
        RuntimeExpression::Macro(
            vector!["condition".to_string(), "branches".to_string()],
            Environment::new(),
            vector![],
        ),

        String::from("Let") =>
        RuntimeExpression::Macro(
            vector!["bindings".to_string(), "body".to_string()],
            Environment::new(),
            vector![],
        ),

        String::from("For") =>
        RuntimeExpression::Macro(
            vector!["binding".to_string(), "body".to_string()],
            Environment::new(),
            vector![],
        ),
    })
}

#[test]
fn parses_basic_macro() {
    let result = program("Package \"foo\"")
        .and_then(|(_, es)| Ok(build_macros(&es, &create_env_with_macros())));
    assert_eq!(
        Ok((
            Some(MacroCall(
                "Package".to_string(),
                vector![RuntimeExpression::String("foo".to_string())],
            )),
            vector![],
        )),
        result
    );
}

#[test]
fn parses_nested_macros() {
    let result = program("Package Package \"foo\"")
        .and_then(|(_, es)| Ok(build_macros(&es, &create_env_with_macros())));
    assert_eq!(
        Ok((
            Some(MacroCall(
                "Package".to_string(),
                vector![MacroCall(
                    "Package".to_string(),
                    vector![RuntimeExpression::String("foo".to_string())],
                )],
            )),
            vector![],
        )),
        result
    );
}

#[test]
fn parses_macros_in_lists() {
    assert_eq!(
        Ok((
            Some(RuntimeExpression::List(vector![
                RuntimeExpression::Number(1),
                MacroCall(
                    "Package".to_string(),
                    vector![MacroCall(
                        "Package".to_string(),
                        vector![RuntimeExpression::String("two".to_string())],
                    )],
                ),
                RuntimeExpression::Number(3)
            ])),
            vector![],
        )),
        program("[1 Package Package \"two\" 3]")
            .and_then(|(_, es)| Ok(build_macros(&es, &create_env_with_macros())))
    )
}

#[test]
fn parses_macros_in_args_to_functions() {
    assert_eq!(
        Ok((
            Some(RuntimeExpression::TaggedTuple(
                Arc::new(RuntimeExpression::Symbol("println".to_string())),
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
            )),
            vector![],
        )),
        program("println(1 Package Package \"two\" 3)")
            .and_then(|(_, es)| Ok(build_macros(&es, &create_env_with_macros())))
    )
}
