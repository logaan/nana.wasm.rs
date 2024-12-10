use im::{hashmap, vector, HashMap};

use crate::{
    eval::eval,
    expressions::RuntimeExpression::{
        self, BuiltinFunction, Function, FunctionCall, List, Macro, Number, ValueName,
    },
    process::Process,
};

pub fn environment() -> HashMap<String, RuntimeExpression> {
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

pub fn environment_with_fn() -> HashMap<String, RuntimeExpression> {
    hashmap! {
        String::from("list-nums") => Function(
            vector![String::from("n")],
            environment(),
            vector![
                Number(0),
                List(vector![
                    Number(1),
                    ValueName(String::from("n")),
                    Number(3),
                    ValueName(String::from("life")),
                ])
            ]
        )
    }
}

#[test]
fn test_scalar_literals() {
    let result = eval(Number(1), environment()).run_until_complete();
    assert_eq!(result, Number(1));

    let result = eval(
        RuntimeExpression::String(String::from("foo")),
        environment(),
    )
    .run_until_complete();
    assert_eq!(result, RuntimeExpression::String(String::from("foo")));
}

#[test]
fn test_value_names() {
    let result = eval(ValueName(String::from("life")), environment()).run_until_complete();
    assert_eq!(result, Number(42));
}

#[test]
fn test_lists() {
    let expression = List(vector![Number(1), ValueName(String::from("life"))]);
    let result = eval(expression, environment()).run_until_complete();
    assert_eq!(result, List(vector![Number(1), Number(42)]));
}

#[test]
fn test_builtin_function_call() {
    let expression = FunctionCall(String::from("foo"), vector![]);
    let result = eval(expression, environment()).run_until_complete();
    assert_eq!(result, RuntimeExpression::String(String::from("bar")))
}

#[test]
fn test_user_defined_function_call() {
    let expression = FunctionCall(String::from("list-nums"), vector![Number(2)]);
    let actual = eval(expression, environment_with_fn()).run_until_complete();
    let expected = List(vector![Number(1), Number(2), Number(3), Number(42)]);
    assert_eq!(expected, actual)
}
