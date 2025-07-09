use core::panic;
use im::{vector, Vector};
use std::sync::Arc;
use Process::{Complete, Running, Spawn};

pub trait Stepable<I: Clone> {
    fn step(&self) -> Process<I>;
}

#[derive(Clone)]
pub enum Process<I: Clone> {
    Spawn(Arc<Process<I>>, Vector<Process<I>>),
    Running(Arc<dyn Stepable<I>>),
    Complete(I),
}

// Functions that return Processes count as Stepable by just calling themselves
impl<I: Clone + 'static, F: Fn() -> Process<I> + 'static> Stepable<I> for F {
    fn step(&self) -> Process<I> {
        self()
    }
}

// AndThen wraps an existing process. It will proxy calls to step(), wrapping
// any Running process returned by the wrapped step() until the wrapped process
// ends. The result will be passed to the function passed as a second argument
// to AndThen. That function should return a new process that will be returned
// directly by AndThen, ending the cycle of wrapping.
struct AndThen<A: Clone, B: Clone>(Process<A>, Arc<dyn Fn(A) -> Process<B>>);

impl<A: Clone + 'static, B: Clone + 'static> Stepable<B> for AndThen<A, B> {
    fn step(&self) -> Process<B> {
        let AndThen(process, and_then) = self;

        match process {
            Complete(result) => (and_then)(result.clone()),
            Running(stepable) => Running(Arc::new(AndThen(stepable.step(), and_then.clone()))),
            // TODO: AndThen spawn
            Spawn(..) => todo!(),
        }
    }
}

// The process data structure itself mostly just wraps a "running" thunk, or a
// complete value. It's just a way of marking whether we've reached the end or
// not. Most of the time Running will be holding a lambda, or an AndThen
// wrapping a lambda. step on the process just proxies down to the contained
// stepable (or panics).
impl<I: Clone + 'static> Process<I> {
    pub fn run_until_complete(self) -> Vector<I> {
        Process::round_robin(vector![self])
    }

    pub fn run_once_until_complete(self) -> I {
        Process::round_robin(vector![self]).head().unwrap().clone()
    }

    pub fn round_robin(processes: Vector<Process<I>>) -> Vector<I> {
        let mut active_processes = processes;
        let mut complete_processes: Vector<I> = vector![];

        while !active_processes.is_empty() {
            match active_processes.pop_front().unwrap() {
                Complete(result) => complete_processes.push_back(result),
                Running(stepable) => active_processes.push_back(stepable.step()),
                Spawn(continuation, spawned_processes) => {
                    active_processes.append(spawned_processes);
                    active_processes.push_back((*continuation).clone());
                }
            }
        }

        complete_processes
    }

    pub fn run_in_sequence(processes: Vector<Process<I>>) -> Process<Vector<I>> {
        Process::run_in_sequence_with_results(processes, vector![])
    }

    fn run_in_sequence_with_results(
        processes: Vector<Process<I>>,
        results: Vector<I>,
    ) -> Process<Vector<I>> {
        if processes.is_empty() {
            Complete(results)
        } else {
            let mut processes = processes;
            let mut results = results;

            let active_process = processes.pop_front().unwrap();

            match active_process {
                Complete(result) => results.push_back(result),
                Running(stepable) => processes.push_front(stepable.step()),
                // TODO: run_in_sequence_with_results spawn
                Spawn(..) => todo!(),
            }

            Running(Arc::new(move || {
                Process::run_in_sequence_with_results(processes.clone(), results.clone())
            }))
        }
    }

    pub fn run_in_sequence_tco(processes: Vector<Process<I>>) -> Process<I> {
        if processes.is_empty() {
            panic!("We must run at least one process");
        } else if processes.len() == 1 {
            let mut processes = processes;
            processes.pop_front().unwrap()
        } else {
            let mut processes = processes;

            match processes.pop_front().unwrap() {
                // A function's body is a list of expressions. We checked above
                // that we're not on the last one, so seeing a completed process
                // here means that we've finished some intermediate expression
                // who's result we're discarding.
                Complete(_) => Running(Arc::new(move || {
                    Process::run_in_sequence_tco(processes.clone())
                })),
                Running(stepable) => {
                    processes.push_front(stepable.step());
                    // TODO: These 3 lines are duplicated 3 times. Might be
                    // worth moving into a fn.
                    Running(Arc::new(move || {
                        Process::run_in_sequence_tco(processes.clone())
                    }))
                }
                // We unwrap the continuation and pop it back where the spawn
                // came from. Then lift the spawned process up a level, helping
                // it trickle outwards.
                Spawn(continuation, spawned_processes) => {
                    processes.push_front((*continuation).clone());

                    Spawn(
                        Arc::new(Running(Arc::new(move || {
                            Process::run_in_sequence_tco(processes.clone())
                        }))),
                        spawned_processes,
                    )
                }
            }
        }
    }

    pub fn and_then<B: Clone + 'static>(
        self,
        and_then: Arc<dyn Fn(I) -> Process<B>>,
    ) -> Process<B> {
        Running(Arc::new(AndThen(self, and_then)))
    }
}
