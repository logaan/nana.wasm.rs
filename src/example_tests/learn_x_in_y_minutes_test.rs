use std::fs::File;
use std::io::Read;
use std::path::Path;

use im::vector;

use crate::eval::execute_with_all_results;
use crate::expressions::RuntimeExpression::{List, Number, String as NString};
use crate::s;
use crate::standard_library::standard_library;

pub fn read_code(path: &str) -> String {
    let mut file = File::open(Path::new(path)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[test]
fn test_learn_x_in_y_minutes() {
    let code = read_code("examples/learn_x_in_y_minutes.nana");
    let result = execute_with_all_results(code, standard_library());
    let expected = vector![
        Number(123),
        NString(s!("This is a single line string.")),
        NString(s!("This is a multi line string.\nIt's worth noting that any indentation will be preserved.")),
        List(vector![Number(1), Number(2), Number(3)]),
        NString(s!("rd")),
        Number(8),
        Number(2),
        NString(s!("Match")),
    ];
    assert_eq!(expected, result);
}
