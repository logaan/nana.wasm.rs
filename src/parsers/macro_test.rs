use std::collections::HashMap;

use super::fizzbuzz_test::FIZZBUZZ;
use super::macros::build_macros;
use super::nana::program;

use super::nana::Expression;

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

fn expected() -> Vec<Expression> {
    vec![Expression::Hole]
}

#[test]
fn parses_macros() {
    let result = program(FIZZBUZZ).and_then(|(_, es)| Ok(build_macros(es, create_macro_map())));
    assert_eq!(Ok(expected()), result);
}
