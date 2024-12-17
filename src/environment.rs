use std::sync::OnceLock;

use im::HashMap;

use crate::expressions::RuntimeExpression;

#[derive(PartialEq, Debug, Clone)]
pub struct Environment {
    map: HashMap<String, OnceLock<RuntimeExpression>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: HashMap::new(),
        }
    }

    pub fn from(from: HashMap<String, RuntimeExpression>) -> Environment {
        Environment {
            map: from.into_iter().fold(HashMap::new(), |acc, (key, value)| {
                acc.update(key, OnceLock::from(value))
            }),
        }
    }

    pub fn prepare(self, key: String) -> Environment {
        Environment {
            map: self.map.update(key, OnceLock::new()),
        }
    }

    pub fn provide(&self, key: &str, value: RuntimeExpression) -> Option<()> {
        self.map.get(key).and_then(|lock| lock.set(value).ok())
    }

    pub fn add(self, key: String, value: RuntimeExpression) -> Environment {
        Environment {
            map: self.map.update(key, OnceLock::from(value)),
        }
    }

    pub fn get(&self, key: &str) -> Option<RuntimeExpression> {
        self.map.get(key).map(|lock| lock.get()).flatten().cloned()
    }

    pub fn union(self, other: Environment) -> Environment {
        Environment {
            map: self.map.union(other.map),
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (String, RuntimeExpression)> + 'a {
        self.map
            .iter()
            .filter_map(|(key, lock)| lock.get().map(|value| (key.clone(), value.clone())))
    }
}
