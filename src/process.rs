use core::panic;
use std::sync::Arc;

// TODO: This is the right place for processes to split. We could go with:
//
//   (Process<T>, Option<Process<T>>)
//
// But it feels inelegant to me. Like a null. It's not nonsense though; it
// doesn't introduce any invalid representations. A new type would be clean and
// expressive:
//
//   pub enum MaybeFork<T: Clone> {
//       Continue(Process<T>),
//       Split(Process<T>, Process<T>),
//   }
//
// But I can't think of a meaningful name for it. We could flatten the idea down.
// Consider Process to be a kind of 2, 1, or 0 outcome. Where 0 is a completed
// process, 1 is a continuing one, and 2 is a splitting one. But it's possible
// that in the split we'll actually have 2 completed processes. And what would
// you call this new abstraction? Certainly not singular process.
//
//   #[derive(Clone)]
//   pub enum Process<T: Clone> {
//       Splitting(Process<T>, Process<T>),
//       Running(Arc<dyn Stepable<T>>),
//       Complete(T),
//   }
//
// It's also bad because we don't want our list of running processes to include
// splitting ones. Either seems appropriate but it isn't part of Rust's core
// libraries so I'm slightly reluctant to pull it in.
//
// fn step(&self) -> Either<Process<T>, (Process<T>, Process<T>)>;
//
// Perhaps we treat stepping a bit like Mapcat / Flatmap. Step always returns a
// collection of processes and they just get appended onto the list of running
// ones.
//
//   fn step(&self) -> Vec<Process<T>>;
//
// Then spawning becomes quite natural. The existing:
//
//   active_processes.push_back(new_process);
//
// just becomes:
//
//   active_processes.append(new_processes);
//
// However it opens up the case of an empty array of processes. That shouldn't
// happen unless they've completed. So it's elegant in one way but it introduces
// an invalid state.
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
        if process.is_complete() {
            (and_then)(process.clone().result().unwrap())
        } else {
            let new_process = process.step();

            if new_process.is_complete() {
                (and_then)(new_process.result().unwrap())
            } else {
                Running(Arc::new(AndThen(new_process, and_then.clone())))
            }
        }
    }
}

#[derive(Clone)]
pub enum Process<T: Clone> {
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
            let next = active_processes.pop_front().unwrap();

            if next.is_complete() {
                complete_processes.push_back(next.result().unwrap());
            } else {
                let new_process = next.step();

                if new_process.is_complete() {
                    complete_processes.push_back(new_process.result().unwrap());
                } else {
                    active_processes.push_back(new_process);
                }
            }
        }

        complete_processes
    }

    pub fn run_in_sequence(processes: Vector<Process<T>>) -> Process<Vector<T>> {
        Process::run_in_sequence_with_results(processes, vector![])
    }

    pub fn run_in_sequence_tco(processes: Vector<Process<T>>) -> Process<T> {
        if processes.is_empty() {
            panic!("We must run at least one process");
        } else if processes.len() == 1 {
            let mut processes = processes;
            processes.pop_front().unwrap()
        } else {
            let mut processes = processes;

            let active_process = processes.pop_front().unwrap();

            if !active_process.is_complete() {
                processes.push_front(active_process.step());
            }

            Running(Arc::new(move || {
                Process::run_in_sequence_tco(processes.clone())
            }))
        }
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
