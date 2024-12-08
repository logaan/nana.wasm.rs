use std::sync::Arc;

pub enum Process<T> {
    Running(Arc<dyn Fn() -> Process<T>>),
    Complete(T),
}

use Process::{Complete, Running};

impl<T> Process<T> {
    pub fn step(&self) -> Process<T> {
        match self {
            Complete(_) => panic!("Process is already complete"),
            Running(f) => f(),
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(self, Complete(_))
    }

    pub fn is_running(&self) -> bool {
        !self.is_complete()
    }
}

#[cfg(test)]
mod tests {
    use im::vector;

    use crate::parsers::macros::RuntimeExpression::{List, Number};

    use super::Process::{Complete, Running};
    use super::*;

    #[test]
    fn test_step_running_process() {
        let process = Running(Arc::new(|| {
            let a = 1;
            Running(Arc::new(move || {
                let b = 2;
                Running(Arc::new(move || {
                    let c = 3;
                    Complete(List(vector![Number(a), Number(b), Number(c)]))
                }))
            }))
        }));

        let actual = process.step().step().step();

        assert!(actual.is_complete());
        assert!(!actual.is_running());

        let expected = List(vector![Number(1), Number(2), Number(3)]);

        match actual {
            Complete(result) => assert_eq!(expected, result),
            _ => assert!(false, "The process was not complete"),
        }
    }
}
