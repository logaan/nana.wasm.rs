use im::{hashmap, vector, HashMap};

use crate::{
    eval::eval,
    expressions::RuntimeExpression::{
        self, BuiltinFunction, BuiltinMacro, Function, FunctionCall, List, Macro, MacroCall,
        Number, ValueName,
    },
    process::Process,
    s,
};

pub fn environment() -> HashMap<String, RuntimeExpression> {
    hashmap! {
        s!("life") => Number(42),
        s!("Package") => Macro(
            vector![s!("name")],
            hashmap!{},
            vector![],
        ),
        s!("foo") => BuiltinFunction(|_args| {
            Process::Complete(RuntimeExpression::String(s!("bar")))
        }),
        s!("swap") => BuiltinMacro(vector![s!("left"), s!("right")], |args| {
            let first = args.head().unwrap().clone();
            let last = args.last().unwrap().clone();
            Process::Complete(List(vector![last, first]))
        }),
        s!("ignore") => Macro(vector![s!("expression")], hashmap!{}, vector![
            Number(42)
        ])
    }
}

pub fn environment_with_fn() -> HashMap<String, RuntimeExpression> {
    hashmap! {
        s!("life") => Number(2),
        s!("list-nums") => Function(
            vector![s!("n")],
            environment(),
            vector![
                Number(0),
                List(vector![
                    Number(1),
                    ValueName(s!("n")),
                    Number(3),
                    ValueName(s!("life")),
                ])
            ]
        )
    }
}

#[test]
fn test_scalar_literals() {
    let result = eval(Number(1), environment()).run_until_complete();
    assert_eq!(result, Number(1));

    let result = eval(RuntimeExpression::String(s!("foo")), environment()).run_until_complete();
    assert_eq!(result, RuntimeExpression::String(s!("foo")));
}

#[test]
fn test_value_names() {
    let result = eval(ValueName(s!("life")), environment()).run_until_complete();
    assert_eq!(result, Number(42));
}

#[test]
fn test_lists() {
    let expression = List(vector![Number(1), ValueName(s!("life"))]);
    let result = eval(expression, environment()).run_until_complete();
    assert_eq!(result, List(vector![Number(1), Number(42)]));
}

#[test]
fn test_builtin_function_call() {
    let expression = FunctionCall(s!("foo"), vector![]);
    let result = eval(expression, environment()).run_until_complete();
    assert_eq!(result, RuntimeExpression::String(s!("bar")))
}

#[test]
fn test_user_defined_function_call() {
    let expression = FunctionCall(s!("list-nums"), vector![ValueName(s!("life"))]);
    let actual = eval(expression, environment_with_fn()).run_until_complete();
    let expected = List(vector![Number(1), Number(2), Number(3), Number(42)]);
    assert_eq!(expected, actual)
}

#[test]
fn test_builtin_macro_call() {
    let expression = MacroCall(s!("swap"), vector![Number(1), Number(2)]);
    let actual = eval(expression, environment()).run_until_complete();
    let expected = List(vector![Number(2), Number(1)]);
    assert_eq!(expected, actual);
}

#[test]
fn test_user_defined_macro_call() {
    let expression = MacroCall(s!("ignore"), vector![Number(1), Number(2)]);
    let actual = eval(expression, environment()).run_until_complete();
    let expected = Number(42);
    assert_eq!(expected, actual);
}
