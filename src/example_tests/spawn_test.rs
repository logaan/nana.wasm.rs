use crate::expressions::RuntimeExpression::{Keyword, List};
use im::{vector, Vector};

use crate::{
    eval::{execute, read_code},
    helpers::strip_functions,
    s, standard_library,
};

#[test]
fn test_spawn_2_and_loop() {
    let code = read_code("examples/spawn.nana");
    let results = execute(code, standard_library());
    let stripped = results
        .into_iter()
        .map(|(r, _e)| strip_functions(r))
        .collect::<Vector<_>>();
    let expected = vector![
        vector![List(vector![Keyword(s!("b")), Keyword(s!("done"))])],
        vector![List(vector![Keyword(s!("a")), Keyword(s!("done"))])],
        vector![
            Keyword(s!("process-spawned")),
            Keyword(s!("process-spawned")),
            List(vector![Keyword(s!("c")), Keyword(s!("done"))]),
        ]
    ];

    assert_eq!(expected, stripped);
}
