use im::hashmap;

use crate::expressions::RuntimeExpression::Number;
use crate::standard_library::standard_library;
use crate::{eval::execute, s};

#[test]
fn test_fn_macro() {
    let environment = hashmap! {
      s!("second") => execute(s!("Fn [a b] b"), standard_library()),
    };

    let expected = Number(2);
    let actual = execute(s!("second(1 2)"), environment);
    assert_eq!(expected, actual);
}

#[test]
fn test_match_macro() {
    let environment = hashmap! {
      s!("result") => execute(s!("Match 3 [1 2 3 4 5 6]"), standard_library()),
    };

    let expected = Number(4);
    let actual = execute(s!("result"), environment);
    assert_eq!(expected, actual);
}
