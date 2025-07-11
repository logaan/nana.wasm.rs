use std::ptr::eq;
use std::sync::{Arc, RwLock};

use im::{hashmap, Vector};

use crate::errors::argument_error;
use crate::expressions::Environment;
use crate::expressions::RuntimeExpression::{self, BuiltinFunction, List};
use crate::process::Process::Complete;
use crate::s;

// TODO: Split out the core Atom implementation from the nana methods that work
// with it. Atom's contained type can be parametric and not coupled to
// RuntimeExpression.
#[derive(Debug)]
pub struct Atom {
    value: RwLock<RuntimeExpression>,
    watchers: RwLock<Vector<RuntimeExpression>>,
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        eq(self, other)
    }
}

impl Clone for Atom {
    fn clone(&self) -> Self {
        Atom {
            value: RwLock::new(self.value.read().unwrap().clone()),
            watchers: RwLock::new(self.watchers.read().unwrap().clone()),
        }
    }
}

impl Atom {
    pub fn get(self) -> RuntimeExpression {
        self.value.read().unwrap().clone()
    }
}

// TODO: Add support for a validator function
// TODO: Merge these builtins into the standard library

pub fn atom_builtins() -> Environment {
    Environment::from(hashmap! {
        // Args:
        //   - value: Any.
        //   - watchers: List<Function | BuiltinFunction>
        //
        // Constructs an instance of Atom with those values and returns it.
        //
        // Returns:
        //   - :error(:argument "Takes two arguments")
        //   - :error(:argument "Second argument must be a list of functions")
        //   - the newly created atom
        s!("atom") => BuiltinFunction(|mut args| {
            if args.len() == 2 {
                match [args.pop_front().unwrap(), args.pop_front().unwrap()] {
                    // TODO: Should maybe check that all watchers are
                    // RuntimeExpression::Functions or
                    // RuntimeExpression::BuiltinFunctions
                    [value, List(watchers)] => Complete(RuntimeExpression::Atom(Arc::new(Atom {
                        value: RwLock::new(value),
                        watchers: RwLock::new(watchers),
                    }))),
                    _ => argument_error("atom's second argument must be a list of watcher functions"),
                }
            } else {
                argument_error("atom takes exactly 2 arguments")
            }
        }),

        // Args:
        //   - value: Atom
        //
        // Returns:
        //   - The current value of the atom
        s!("get") => BuiltinFunction(|mut args| {
            if args.len() == 1 {
                match args.pop_front().unwrap() {
                    RuntimeExpression::Atom(atom) => Complete((*atom).clone().get()),
                    _ => argument_error("get takes exactly one atom as an argument")
                }
            } else {
                argument_error("get takes exactly one atom as an argument")
            }
        }),

        // Args:
        //   - atom: Atom
        //   - new-value: Any
        //
        // 1. Updates the value stored inside the atom with the new value
        // 2. Calls all watchers with the new and old values
        //
        // Returns:
        //   - :error(:argument "Takes two arguments")
        //   - :error(:argument "First argument must be an atom")
        //   - :ok(old-value)
        s!("set!") => BuiltinFunction(|_args| {
            todo!()
        }),

        // Args:
        //   - transaction: (Function | BuiltinFunction)<new: Any, old: Any> -> Any
        //
        // 1. Takes out a write lock
        // 2. Calls `transaction` with the current value.
        // 3. Calls `reset` with the result of `transaction`
        // 4. Releases the lock
        //
        // Returns:
        //   - :error(:argument "Takes one argument")
        //   - :error(:argument "First argument must be function")
        //   - :ok(old-value new-value)
        s!("transact!") => BuiltinFunction(|_args| {
            todo!()
        }),

        // Args:
        //   - name: Keyword
        //   - watcher: Function<Any, Any> -> :ok
        //
        // 1. Adds the new watcher to the list of watchers
        //
        // Returns:
        //   - :error(:argument "Takes two arguments")
        //   - :error(:argument "First argument must be a keyword")
        //   - :error(:argument "Second argument must be function")
        //   - :error(:key "Key already present")
        //   - :ok
        s!("subscribe!") => BuiltinFunction(|_args| {
            todo!()
        }),

        // Args:
        //   - name: Keyword
        //
        // 1. Removes the named watcher from the list of watchers
        //
        // Returns:
        //   - :error(:argument "Takes one arguments")
        //   - :error(:argument "First argument must be a keyword")
        //   - :error(:key "Key not found")
        //   - :ok
        s!("unsubscribe!") => BuiltinFunction(|_args| {
            todo!()
        }),
    })
}
