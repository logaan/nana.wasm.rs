use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::eval::execute;
use crate::expressions::RuntimeExpression::Number;
use crate::standard_library::standard_library;

fn read_code() -> String {
    let path = Path::new("examples/learn_x_in_y_minutes.nana");
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn test_learn_x_in_y_minutes() {
    let result = execute(read_code(), standard_library());
    assert_eq!(Number(42), result);
}
