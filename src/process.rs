use std::sync::Arc;

pub enum Process<T> {
    Running(Arc<dyn Fn() -> Process<T>>),
    Complete(T),
}

use im::Vector;
use Process::{Complete, Running};

impl<T> Process<T> {
    pub fn step(&self) -> Process<T> {
        match self {
            Complete(_) => panic!("Process is already complete"),
            Running(f) => f(),
        }
    }

    pub fn result(self) -> Result<T, String> {
        match self {
            Complete(result) => Ok(result),
            Running(_) => Err("Process still running".to_string()),
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(self, Complete(_))
    }

    pub fn is_running(&self) -> bool {
        !self.is_complete()
    }

    pub fn run_until_complete(self) -> T {
        let mut active_process = self;
        while active_process.is_running() {
            active_process = active_process.step();
        }

        active_process.result().unwrap()
    }

    pub fn round_robin(_processes: Vector<Process<T>>) -> Vector<T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use im::vector;

    use crate::parsers::macros::RuntimeExpression::{self, List, Number};

    use super::Process::{Complete, Running};
    use super::*;

    fn make_process() -> Process<RuntimeExpression> {
        Running(Arc::new(|| {
            let a = 1;
            Running(Arc::new(move || {
                let b = 2;
                Running(Arc::new(move || {
                    let c = 3;
                    Complete(List(vector![Number(a), Number(b), Number(c)]))
                }))
            }))
        }))
    }

    #[test]
    fn test_process_by_steps() {
        let actual = make_process().step().step().step();

        assert!(actual.is_complete());
        assert!(!actual.is_running());

        let expected = List(vector![Number(1), Number(2), Number(3)]);

        assert_eq!(expected, actual.result().unwrap());
    }

    #[test]
    fn test_process_to_completion() {
        let actual = make_process().run_until_complete();

        let expected = List(vector![Number(1), Number(2), Number(3)]);
        assert_eq!(expected, actual);
    }
}