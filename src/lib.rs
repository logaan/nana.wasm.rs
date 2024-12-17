#![allow(dead_code)]

#[allow(warnings)]
mod bindings;
mod helpers;

mod environment;
#[cfg(test)]
mod environment_test;
mod eval;
mod expressions;
mod parsers;
mod process;
mod standard_library;

#[cfg(test)]
mod eval_test;
mod example_tests;
#[cfg(test)]
mod process_test;
#[cfg(test)]
mod standard_library_test;

use crate::expressions::RuntimeExpression::{self, BuiltinFunction};
use bindings::exports::wasi::cli::run::Guest as Command;
use eval::execute;
use im::hashmap;
use process::Process;

struct Component;

impl Command for Component {
    fn run() -> Result<(), ()> {
        let environment = hashmap! {
            String::from("greet") => BuiltinFunction(|_args| {
                Process::Complete(RuntimeExpression::String(String::from("Hello, World.")))
            }),
        };
        let result = execute(PROGRAM_CODE.to_owned(), environment);
        println!("{:?}", result);
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);

pub static PROGRAM_CODE: &str = r#"
greet()
"#;
