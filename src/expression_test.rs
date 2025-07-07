use crate::expressions::{print, Environment, RuntimeExpression};
use crate::process::Process;
use im::vector;
use std::sync::Arc;
use RuntimeExpression::*;

#[test]
fn test_print_builtin_function() {
    fn dummy(_: im::Vector<RuntimeExpression>) -> Process<RuntimeExpression> {
        panic!("not called")
    }
    let expr = BuiltinFunction(dummy);
    assert_eq!(print(expr), "BuiltinFunction(..)");
}

#[test]
fn test_print_function() {
    let expr = Function(
        vector!["x".to_string(), "y".to_string()],
        Environment::new(),
        vector![],
    );
    assert_eq!(print(expr), "Function([x y] _)");
}

#[test]
fn test_print_builtin_macro() {
    fn dummy(_: im::Vector<RuntimeExpression>, _: Environment) -> Process<RuntimeExpression> {
        panic!("not called")
    }
    let expr = BuiltinMacro(vector!["x".to_string(), "y".to_string()], dummy);
    assert_eq!(print(expr), "BuiltinMacro([x y] _)");
}

#[test]
fn test_print_definition() {
    let expr = Definition("foo".to_string(), Arc::new(Number(1)));
    assert_eq!(print(expr), "Definition(foo 1)");
}

#[test]
fn test_print_hole() {
    let expr = Hole;
    assert_eq!(print(expr), "_");
}

#[test]
fn test_print_keyword() {
    let expr = Keyword("foo".to_string());
    assert_eq!(print(expr), ":foo");
}

#[test]
fn test_print_list() {
    let expr = List(vector![Number(1), Number(2)]);
    assert_eq!(print(expr), "[1 2]");
}

#[test]
fn test_print_macro() {
    let expr = Macro(
        vector!["a".to_string(), "b".to_string()],
        Environment::new(),
        vector![],
    );
    assert_eq!(print(expr), "Macro([a b] _)");
}

#[test]
fn test_print_macro_call() {
    let expr = MacroCall("foo".to_string(), vector![Number(1), Number(2)]);
    assert_eq!(print(expr), "foo(1 2)");
}

#[test]
fn test_print_number() {
    let expr = Number(42);
    assert_eq!(print(expr), "42");
}

#[test]
fn test_print_nstring() {
    let expr = String("bar".to_string());
    assert_eq!(print(expr), "\"bar\"");
}

#[test]
fn test_print_symbol() {
    let expr = Symbol("baz".to_string());
    assert_eq!(print(expr), "baz");
}

#[test]
fn test_print_tagged_tuple() {
    let tag = Arc::new(Symbol("tag".to_string()));
    let expr = TaggedTuple(tag, vector![Number(1), Number(2)]);
    assert_eq!(print(expr), "tag(1 2)");
}
