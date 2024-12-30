use im::{vector, Vector};

use crate::eval::{execute, read_code};
use crate::expressions::RuntimeExpression::{
    self, Function, List, Macro, Number, String as NString, Symbol, TaggedTuple,
};
use crate::s;
use crate::standard_library::standard_library;

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
    let results = execute(code, standard_library());
    // println!("#*# Results: {:?}", results);
    let stripped = strip_functions(results);
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

        // Defmacro
        Number(2),

    ];

    assert_eq!(expected, stripped);
}
