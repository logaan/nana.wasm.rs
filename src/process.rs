use std::sync::Arc;

#[derive(Clone)]
pub enum FnProcess<T: Clone> {
    // TODO: If the fn returned a list of processes then running processes could
    // kick off new processes and they could be added into the round robin pool.
    Running(Arc<dyn Fn() -> FnProcess<T>>),
    Complete(T),
}

use im::{vector, Vector};
use FnProcess::{Complete, Running};

impl<T: Clone + 'static> FnProcess<T> {
    pub fn step(&self) -> FnProcess<T> {
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

    pub fn round_robin(processes: Vector<FnProcess<T>>) -> Vector<T> {
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
        processes: Vector<FnProcess<T>>,
        results: Vector<T>,
    ) -> FnProcess<Vector<T>> {
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
                FnProcess::run_in_sequence(processes.clone(), results.clone())
            }))
        }
    }
}
