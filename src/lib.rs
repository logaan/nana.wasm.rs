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

use bindings::exports::component::nana::nana::Guest as Nana;
use bindings::exports::wasi::cli::run::Guest as Command;
use eval::execute;
use standard_library::standard_library;

struct Component;

impl Command for Component {
    fn run() -> Result<(), ()> {
        execute(PROGRAM_CODE.to_owned(), standard_library());
        Ok(())
    }
}

impl Nana for Component {
    fn evaluate(name: String) -> String {
        let result = execute(name, standard_library());
        result
            .into_iter()
            .map(|item| format!("> {:?}", item))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

bindings::export!(Component with_types_in bindings);

pub static PROGRAM_CODE: &str = include_str!("../examples/main.nana");
