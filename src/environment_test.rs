use crate::{environment::Environment, expressions::RuntimeExpression::Number, s};

#[test]
pub fn test_environment() {
    let env = Environment::new();

    assert_eq!(env.provide("one", Number(1)), None);

    let env_one = env.prepare(s!("one"));
    env_one
        .provide("one", Number(1))
        .expect("Value was not prepared");

    let env_two = env_one.add(s!("two"), Number(2));

    assert_eq!(env_two.get("one"), Some(Number(1)));
    assert_eq!(env_two.get("two"), Some(Number(2)));
    assert_eq!(env_two.get("three"), None);
}
