use std::sync::Arc;

#[derive(Clone)]
pub enum Process<T: Clone> {
    // TODO: If the fn returned a list of processes then running processes could
    // kick off new processes and they could be added into the round robin pool.
    Running(Arc<dyn Fn() -> Process<T>>),
    Complete(T),
}

use im::{vector, Vector};
use Process::{Complete, Running};

impl<T: Clone> Process<T> {
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

    pub fn round_robin(processes: Vector<Process<T>>) -> Vector<T> {
        let mut active_processes = processes;
        let mut complete_processes: Vector<T> = vector![];

        while !active_processes.is_empty() {
            let new_process = active_processes.pop_front().unwrap().step();

            if new_process.is_complete() {
                complete_processes.push_back(new_process.result().unwrap());
            } else {
                active_processes.push_back(new_process);
            }
        }

        complete_processes
    }

    pub fn run_in_sequence(
        processes: Vector<Process<T>>,
        results: Vector<T>,
    ) -> Process<Vector<T>> {
        let mut processes = processes;
        let mut results = results;

        if processes.is_empty() {
            Complete(results)
        } else {
            let active_process = processes.pop_front().unwrap();
            if active_process.is_complete() {
                results.push_back(active_process.result().unwrap());
            } else {
                processes.push_front(active_process.step());
            }
            Self::run_in_sequence(processes, results)
        }
    }
}

#[cfg(test)]
mod tests {
    use im::vector;

    use crate::parsers::macros::RuntimeExpression::{self, List, Number};

    use super::Process::{Complete, Running};
    use super::*;

    fn make_process(a: u8, b: u8, c: u8) -> Process<RuntimeExpression> {
        Running(Arc::new(move || {
            Running(Arc::new(move || {
                Running(Arc::new(move || {
                    Complete(List(vector![Number(a), Number(b), Number(c)]))
                }))
            }))
        }))
    }

    #[test]
    fn test_process_by_steps() {
        let actual = make_process(1, 2, 3).step().step().step();

        assert!(actual.is_complete());
        assert!(!actual.is_running());

        let expected = List(vector![Number(1), Number(2), Number(3)]);

        assert_eq!(expected, actual.result().unwrap());
    }

    #[test]
    fn test_process_to_completion() {
        let actual = make_process(1, 2, 3).run_until_complete();

        let expected = List(vector![Number(1), Number(2), Number(3)]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_processes_to_completion() {
        let actual = Process::round_robin(vector![
            make_process(1, 2, 3),
            make_process(4, 5, 6),
            make_process(7, 8, 9),
        ]);

        let expected = vector![
            List(vector![Number(1), Number(2), Number(3)]),
            List(vector![Number(4), Number(5), Number(6)]),
            List(vector![Number(7), Number(8), Number(9)]),
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_complete_sequence() {
        let input = vector![
            Complete(Number(1)),
            Complete(Number(2)),
            Complete(Number(3))
        ];

        let process = Process::run_in_sequence(input, vector![]);

        let actual = process.run_until_complete();

        let expected = vector![Number(1), Number(2), Number(3)];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_running_sequence() {
        let input = vector![
            make_process(1, 2, 3),
            make_process(4, 5, 6),
            make_process(7, 8, 9),
        ];

        let process = Process::run_in_sequence(input, vector![]);

        let actual = process.run_until_complete();

        let expected = vector![
            List(vector![Number(1), Number(2), Number(3)]),
            List(vector![Number(4), Number(5), Number(6)]),
            List(vector![Number(7), Number(8), Number(9)]),
        ];

        assert_eq!(expected, actual);
    }
}
