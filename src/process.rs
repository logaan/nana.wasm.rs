use std::sync::Arc;

pub enum Process<T> {
    Running(Arc<dyn Fn() -> Process<T>>),
    Complete(T),
}

impl<T> Process<T> {
    pub fn step(self) -> Process<T> {
        match self {
            Process::Complete(_) => panic!("Process is already complete"),
            Process::Running(f) => f(),
        }
    }

    pub fn is_complete(self) -> bool {
        match self {
            Process::Complete(_) => true,
            Process::Running(_) => false,
        }
    }

    pub fn is_running(self) -> bool {
        !self.is_complete()
    }
}

#[cfg(test)]
mod tests {
    use im::vector;

    use crate::parsers::macros::RuntimeExpression;

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
                    Complete(RuntimeExpression::List(vector![
                        RuntimeExpression::Number(a),
                        RuntimeExpression::Number(b),
                        RuntimeExpression::Number(c)
                    ]))
                }))
            }))
        }));

        let expected = RuntimeExpression::List(vector![
            RuntimeExpression::Number(1),
            RuntimeExpression::Number(2),
            RuntimeExpression::Number(3)
        ]);

        let actual = process.step().step().step();

        match actual {
            Complete(result) => assert_eq!(expected, result),
            _ => assert!(false, "The process was not complete"),
        }
    }
}
