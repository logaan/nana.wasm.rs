use std::sync::OnceLock;

use im::HashMap;

use crate::expressions::RuntimeExpression;

type Environment = HashMap<String, OnceLock<RuntimeExpression>>;

pub fn create_environment() -> Environment {
    HashMap::new()
}

pub fn prepare(env: Environment, key: String) -> Environment {
    env.update(key, OnceLock::new())
}

pub fn provide(env: &Environment, key: String, value: RuntimeExpression) -> Option<()> {
    env.get(&key).and_then(|lock| lock.set(value).ok())
}

pub fn add(env: Environment, key: String, value: RuntimeExpression) -> Environment {
    env.update(key, OnceLock::from(value))
}

pub fn get(env: &Environment, key: String) -> Option<RuntimeExpression> {
    env.get(&key).map(|lock| lock.get()).flatten().cloned()
}
