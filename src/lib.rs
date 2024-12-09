#![allow(dead_code)]

#[allow(warnings)]
mod bindings;
mod eval;
#[cfg(test)]
mod eval_test;
mod process;
#[cfg(test)]
mod process_test;

use bindings::exports::wasi::cli::run::Guest as Command;

mod parsers;

struct Component;

impl Command for Component {
    fn run() -> Result<(), ()> {
        println!("Hello world");
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
