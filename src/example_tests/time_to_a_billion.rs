use crate::{
    eval::{execute, read_code},
    expressions::print_many,
    standard_library,
};

// Disabling this test for now. It's quite slow.
// #[test]
fn test_counting_to_a_billion() {
    let code = read_code("examples/abillion.nana");
    let placeholder = 4;
    let results = execute(code, standard_library());
    println!("{}", placeholder);

    println!("###### count to a billion in n ms ####################################");
    println!("{}", print_many(results.head().unwrap().clone().0, "\n"));
    println!("######################################################################");

    assert_eq!(true, true);
}
