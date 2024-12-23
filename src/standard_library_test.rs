use im::hashmap;

use crate::environment::Environment;
use crate::expressions::RuntimeExpression::{Number, String as NString};
use crate::standard_library::standard_library;
use crate::{eval::execute, s};

#[test]
fn test_fn_macro() {
    let environment = Environment::from(hashmap! {
      s!("second") => execute(s!("Fn [a b] b"), standard_library()).unwrap(),
    });

    let expected = Some(Number(2));
    let actual = execute(s!("second(1 2)"), environment);
    assert_eq!(expected, actual);
}

#[test]
fn test_match_macro() {
    let environment = Environment::from(hashmap! {
      s!("result") => execute(s!("Match 3 [1 2 3 4 5 6]"), standard_library()).unwrap(),
    });

    let expected = Some(Number(4));
    let actual = execute(s!("result"), environment);
    assert_eq!(expected, actual);
}

#[test]
fn test_match_eval() {
    let environment = standard_library().union(Environment::from(hashmap! {
     s!("foo") => Number(1),
     s!("bar") => Number(3)
    }));
    let program = s!("Match foo [1 bar]");
    let expected = Some(Number(3));
    let actual = execute(program, environment);
    assert_eq!(expected, actual);
}

#[test]
fn test_match_binding() {
    let environment = standard_library();
    let program = s!("Match 1 [num num]");
    let expected = Some(Number(1));
    let actual = execute(program, environment);
    assert_eq!(expected, actual);
}

#[test]
fn test_value_definitions() {
    let program = s!("Def result 1 result");
    let actual = execute(program, standard_library());
    let expected = Some(Number(1));
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
    let actual = execute(String::from(program), standard_library());
    let expected = Some(NString(s!("done")));
    assert_eq!(expected, actual);
}
