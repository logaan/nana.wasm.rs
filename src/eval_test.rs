use im::{hashmap, vector, HashMap};

use crate::{
    eval::eval,
    expressions::RuntimeExpression::{
        self, BuiltinFunction, FunctionCall, List, Macro, Number, ValueName,
    },
    process::Process,
};

pub fn create_macro_map() -> HashMap<String, RuntimeExpression> {
    hashmap! {
        String::from("life") => Number(42),
        String::from("Package") => Macro(
            "Package".to_string(),
            vector!["name".to_string()],
            vector![],
        ),
        String::from("foo") => BuiltinFunction(|_args| {
            Process::Complete(RuntimeExpression::String(String::from("bar")))
        }),
    }
}

#[test]
fn test_scalar_literals() {
    let result = eval(Number(1), create_macro_map()).run_until_complete();
    assert_eq!(result, Number(1));

    let result = eval(
        RuntimeExpression::String(String::from("foo")),
        create_macro_map(),
    )
    .run_until_complete();
    assert_eq!(result, RuntimeExpression::String(String::from("foo")));
}

#[test]
fn test_value_names() {
    let result = eval(ValueName(String::from("life")), create_macro_map()).run_until_complete();
    assert_eq!(result, Number(42));
}

#[test]
fn test_lists() {
    let expression = List(vector![Number(1), ValueName(String::from("life"))]);
    let result = eval(expression, create_macro_map()).run_until_complete();
    assert_eq!(result, List(vector![Number(1), Number(42)]));
}

#[test]
fn test_builtin_function_call() {
    let expression = FunctionCall(String::from("foo"), vector![]);
    let result = eval(expression, create_macro_map()).run_until_complete();
    assert_eq!(result, RuntimeExpression::String(String::from("bar")))
}
