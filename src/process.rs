use core::panic;
use im::{vector, Vector};
use std::sync::Arc;
use Process::{Complete, Running, Spawn};

use crate::expressions::RuntimeExpression;

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
//
// Step should be returning "1 or more" new processes. We can represent that with
//
//   (Process<T>, Vector<Process<T>>)
//
// Then we don't risk returning 0 processes, because the left one must alwasy be
// present. And we don't need to worry about at some point returning more than
// one spawned process.
//
// ----------------------------------------------------------------------
//
// After a few failed attempts at implementing this I think I've learned a
// couple of things:
//
//  1: Some of the eval code really expects there to be a concept of a main
//  thread. Something that's updating definitions and maybe accumulating return
//  values of each top level expression.
//
//  2: The type for process isn't always the same. Sometimes it's
//  (Vector<RuntimeExpression>, Environment), and sometimes it's
//  Vector<Process<(Vector<RuntimeExpression>, Environment)>>. Which really
//  complicates things because the spawned threads don't nessisarily match.
//
// My best idea at the moment is that Running processes maybe add a set of child
// processes as a second field. Though it'll be hard to track the state of which
// children have been run as we're doing the round robin (which will now be a
// tree traversal).
//
// It might also be worth thinking about how to represent blocked processes.
//
// ----------------------------------------------------------------------
//
// Here's the plan:
// 1. [x] Remove step() from process. Replace any breakages with match.
// 2. [x] Remove is_complete() and is_running(). Replace and breakages with match.
// 3. [x] Remove result(). Replace break match.
// 4. [ ] Add a third Process type: Spawn(Arc<Process>), update all matches to
//        handle it. Spawning will be a side effect, for now it evaluates to :ok
// 5. [ ] Add a spawn() function that returns a Spawn process.
pub trait Stepable<T: Clone> {
    fn step(&self) -> Process<T>;
}

#[derive(Clone)]
pub enum Process<T: Clone> {
    // 1. I'm not sure that Vector is strictly needed here. But I need some kind
    // of indirection, Option isn't sufficient. And Option inside Arc feels a
    // little inelegant.
    // 2. I think RuntimeExpression could be a type parameter. T becomes
    // IntermediateResultType and RuntimeExpression becomes TopResultType. But
    // that would involve updating all references to Process. I'd prefer to make
    // some progress and then come back to clean up.
    Spawn(Arc<Process<T>>, Vector<Process<RuntimeExpression>>),
    Running(Arc<dyn Stepable<T>>),
    Complete(T),
}

// Functions that return Processes count as Stepable by just calling themselves
impl<T: Clone + 'static, F: Fn() -> Process<T> + 'static> Stepable<T> for F {
    fn step(&self) -> Process<T> {
        self()
    }
}

// AndThen wraps an existing process. It will proxy calls to step(), wrapping
// any Running process returned by the wrapped step() until the wrapped process
// ends. The result will be passed to the function passed as a second argument
// to AndThen. That function should return a new process that will be returned
// directly by AndThen, ending the cycle of wrapping.
//
// At any stage a wrapped step() spawning a new process should have that new
// process exposed without wrapping. We only call step() at most once per cycle
// so there's no need for Vec rather than Option for spawned processes.
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
impl<T: Clone + 'static> Process<T> {
    // This is used in tests, and for eval. round_robin hasn't been adopted yet.
    // But this should probably be deprecated because it can't support spawning
    // new processes.
    pub fn run_until_complete(self) -> T {
        let mut active_process = self;
        loop {
            match active_process {
                Complete(result) => return result,
                Running(stepable) => active_process = stepable.step(),
                // TODO: run_until_complete spawn
                Spawn(..) => todo!(),
            }
        }
    }

    // Round robin only works with top level processes.
    pub fn round_robin(processes: Vector<Process<RuntimeExpression>>) -> Vector<RuntimeExpression> {
        let mut active_processes = processes;
        let mut complete_processes: Vector<RuntimeExpression> = vector![];

        while !active_processes.is_empty() {
            match active_processes.pop_front().unwrap() {
                Complete(result) => complete_processes.push_back(result),
                Running(stepable) => active_processes.push_back(stepable.step()),
                Spawn(continuation, spawned_processes) => {
                    // The continuation might be a Vector<Process<T>> or
                    // something with an Environment. Where the spawned
                    // processes will always be a top level process... maybe it
                    // is worth looking into process trees... traversing one
                    // probably wouldn't be so different to a queue.
                    active_processes.append(spawned_processes);
                    active_processes.push_back((*continuation).clone());
                }
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

    pub fn run_in_sequence_tco(processes: Vector<Process<T>>) -> Process<T> {
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
        and_then: Arc<dyn Fn(T) -> Process<B>>,
    ) -> Process<B> {
        Running(Arc::new(AndThen(self, and_then)))
    }
}
