use im::{hashmap, vector};

use crate::environment::Environment;
use crate::expressions::RuntimeExpression::{Macro, Number, String as NString, Symbol};
use crate::helpers::strip_functions;
use crate::standard_library::core::standard_library;
use crate::{eval::execute_once, s};

#[test]
fn test_fn_macro() {
    let environment = Environment::from(hashmap! {
      s!("second") => execute_once(s!("Fn [a b] b"), standard_library()).head().unwrap().clone(),
    });

    let expected = vector!(Number(2));
    let actual = execute_once(s!("second(1 2)"), environment);
    assert_eq!(expected, actual);
}

#[test]
fn test_match_macro() {
    let environment = Environment::from(hashmap! {
      s!("result") => execute_once(s!("Match 3 [1 2 3 4 5 6]"), standard_library()).head().unwrap().clone(),
    });

    let expected = vector!(Number(4));
    let actual = execute_once(s!("result"), environment);
    assert_eq!(expected, actual);
}

#[test]
fn test_match_eval() {
    let environment = standard_library().union(Environment::from(hashmap! {
     s!("foo") => Number(1),
     s!("bar") => Number(3)
    }));
    let program = s!("Match foo [1 bar]");
    let expected = vector!(Number(3));
    let actual = execute_once(program, environment);
    assert_eq!(expected, actual);
}

#[test]
fn test_match_binding() {
    let environment = standard_library();
    let program = s!("Match 1 [num num]");
    let expected = vector!(Number(1));
    let actual = execute_once(program, environment);
    assert_eq!(expected, actual);
}

#[test]
fn test_value_definitions() {
    let program = s!("
        Def result 1
        result
    ");
    let actual = execute_once(program, standard_library());
    let expected = vector!(Number(1), Number(1));
    assert_eq!(expected, actual);
}

#[test]
fn test_recursive_function_definitions() {
    let program = r#"
    Def recur-once
      Fn [n]
        Match n
          [1 "done"
           _ recur-once(1)]
    
    recur-once(2)"#;
    let actual = execute_once(String::from(program), standard_library())
        .back()
        .unwrap()
        .clone();
    let expected = NString(s!("done"));
    assert_eq!(expected, actual);
}

#[test]
fn test_macro() {
    let program = r#"Macro [a b] b"#;
    let mut actual = execute_once(String::from(program), standard_library());
    assert!(match actual.pop_front().unwrap() {
        Macro(args, _, body) => {
            assert_eq!(vector![s!("a"), s!("b")], args);
            assert_eq!(vector![Symbol(s!("b"))], body);
            true
        }
        _ => false,
    });
}

#[test]
fn test_func() {
    let program = r#"
    Func first [a b]
        a
    
    first(1 2)"#;
    let actual = strip_functions(execute_once(String::from(program), standard_library()));
    let expected = vector![Number(1)];
    assert_eq!(expected, actual);
}
