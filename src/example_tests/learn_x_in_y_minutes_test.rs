use std::sync::Arc;

use im::vector;

use crate::eval::{execute, read_code};

use crate::expressions::RuntimeExpression::{
    Keyword, List, Number, String as NString, Symbol, TaggedTuple,
};
use crate::helpers::strip_functions;
use crate::s;
use crate::standard_library::standard_library;

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
        List(vector![Keyword(s!("north")), Keyword(s!("south")), Keyword(s!("east")), Keyword(s!("west"))]),
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
        TaggedTuple(Arc::new(Symbol(s!("decrement"))), vector![Symbol(s!("life"))]),
        TaggedTuple(Arc::new(Symbol(s!("foo"))), vector![Symbol(s!("bar"))]),

        // Defmacro
        Number(2),

        // Unquote
        Number(42),
        List(vector![Number(1), Number(42), Number(3), Symbol(s!("foo"))]),
        Keyword(s!("true")),
        NString(s!("It's Tuesday!")),

    ];

    assert_eq!(expected, stripped);
}
