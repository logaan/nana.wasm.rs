use std::ptr::eq;
use std::sync::RwLock;

use im::{hashmap, Vector};

use crate::expressions::Environment;
use crate::expressions::RuntimeExpression::{self, BuiltinFunction};
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

// TODO: Need to create a RuntimeExpression::Atom
// TODO: Add support for a validator function

pub fn atom_builtins() -> Environment {
    Environment::from(hashmap! {
      s!("atom") => BuiltinFunction(|_args| {
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
        todo!()
      }),

      s!("get") => BuiltinFunction(|_args| {
        // Args:
        //   - value: Atom
        //
        // Returns:
        //   - The current value of the atom
        todo!()
      }),

      s!("set!") => BuiltinFunction(|_args| {
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
        todo!()
      }),

      s!("transact!") => BuiltinFunction(|_args| {
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
        todo!()
      }),

      s!("subscribe!") => BuiltinFunction(|_args| {
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
        todo!()
      }),

      s!("unsubscribe!") => BuiltinFunction(|_args| {
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
        todo!()
      }),
    })
}
