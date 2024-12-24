use std::fs::File;
use std::io::Read;
use std::path::Path;

use im::{vector, Vector};

use crate::eval::execute_with_all_results;
use crate::expressions::RuntimeExpression::{
    self, Function, List, Macro, Number, String as NString, Symbol, TaggedTuple,
};
use crate::s;
use crate::standard_library::standard_library;

pub fn read_code(path: &str) -> String {
    let mut file = File::open(Path::new(path)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

// Asserting against functions will cause a stack overflow because functions
// have a reference to themselves via their closed over environment.
fn strip_functions(expressions: Vector<RuntimeExpression>) -> Vector<RuntimeExpression> {
    expressions
        .into_iter()
        .filter(|e| match e {
            Function(..) => false,
            Macro(..) => false,
            _ => true,
        })
        .collect()
}

#[test]
fn test_learn_x_in_y_minutes() {
    let code = read_code("examples/learn_x_in_y_minutes.nana");
    let results = strip_functions(execute_with_all_results(code, standard_library()));
    let expected = vector![
        Number(123),
        NString(s!("This is a single line string.")),
        NString(s!("This is a multi line string.\nIt's worth noting that any indentation will be preserved.")),
        List(vector![Number(1), Number(2), Number(3)]),
        NString(s!("rd")),
        Number(8),
        Number(2),
        NString(s!("Matching pairs")),
        Number(42),
        Number(42),
        NString(s!("done")),

        // Macros
        Number(1),
        Symbol(s!("life")),
        TaggedTuple(s!("dec"), vector![Symbol(s!("life"))]),
        TaggedTuple(s!("foo"), vector![Symbol(s!("bar"))]),

    ];

    assert_eq!(expected, results);
}
