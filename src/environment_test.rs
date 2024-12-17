use crate::{
    environment::{add, create_environment, get, prepare, provide},
    expressions::RuntimeExpression::Number,
    s,
};

#[test]
pub fn test_environment() {
    let env = create_environment();

    assert_eq!(provide(&env, s!("one"), Number(1)), None);

    let env_one = prepare(env, s!("one"));
    provide(&env_one, s!("one"), Number(1)).expect("Value was not prepared");

    let env_two = add(env_one, s!("two"), Number(2));

    assert_eq!(get(&env_two, s!("one")), Some(Number(1)));
    assert_eq!(get(&env_two, s!("two")), Some(Number(2)));
    assert_eq!(get(&env_two, s!("three")), None);
}
