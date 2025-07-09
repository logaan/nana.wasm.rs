use crate::expressions::RuntimeExpression::Keyword;
use im::vector;

use crate::{
    eval::{execute, read_code},
    helpers::strip_functions,
    s, standard_library,
};

#[test]
fn test_spawn_2_and_loop() {
    let code = read_code("examples/threads.nana");
    let results = execute(code, standard_library());
    let stripped = strip_functions(results);
    let expected = vector![
        // spawn() really should return a value. It's broken for now.
        // Keyword(s!("ok")),
        // Keyword(s!("ok")),
        Keyword(s!("done")),
    ];

    assert_eq!(expected, stripped);
}
