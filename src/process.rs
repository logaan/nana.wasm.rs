use std::sync::Arc;

pub trait Stepable<T: Clone> {
    fn step(&self) -> Process<T>;
}

// Functions that return Processes count as Stepable by just calling themselves
impl<T: Clone + 'static, F: Fn() -> Process<T> + 'static> Stepable<T> for F {
    fn step(&self) -> Process<T> {
        self()
    }
}

struct AndThen<A: Clone, B: Clone>(Process<A>, Arc<dyn Fn(A) -> Process<B>>);

impl<A: Clone + 'static, B: Clone + 'static> Stepable<B> for AndThen<A, B> {
    fn step(&self) -> Process<B> {
        let AndThen(process, and_then) = self;
        let new_process = process.step();

        if new_process.is_complete() {
            (and_then)(new_process.result().unwrap())
        } else {
            Running(Arc::new(AndThen(new_process, and_then.clone())))
        }
    }
}

#[derive(Clone)]
pub enum Process<T: Clone> {
    // TODO: If the fn returned a list of processes then running processes could
    // kick off new processes and they could be added into the round robin pool.
    Running(Arc<dyn Stepable<T>>),
    Complete(T),
}

use im::{vector, Vector};
use Process::{Complete, Running};

impl<T: Clone + 'static> Process<T> {
    pub fn step(&self) -> Process<T> {
        match self {
            Complete(_) => panic!("Process is already complete"),
            Running(f) => f.step(),
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

    pub fn run_in_sequence(processes: Vector<Process<T>>) -> Process<Vector<T>> {
        Process::run_in_sequence_with_results(processes, vector![])
    }

    fn run_in_sequence_with_results(
        processes: Vector<Process<T>>,
        results: Vector<T>,
    ) -> Process<Vector<T>> {
        if processes.is_empty() {
            Complete(results)
        } else {
            let mut processes = processes;
            let mut results = results;

            let active_process = processes.pop_front().unwrap();

            if active_process.is_complete() {
                results.push_back(active_process.result().unwrap());
            } else {
                processes.push_front(active_process.step());
            }

            Running(Arc::new(move || {
                Process::run_in_sequence_with_results(processes.clone(), results.clone())
            }))
        }
    }

    pub fn and_then<B: Clone + 'static>(
        self,
        and_then: Arc<dyn Fn(T) -> Process<B>>,
    ) -> Process<B> {
        Running(Arc::new(AndThen(self, and_then)))
    }
}
