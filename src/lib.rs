#![allow(dead_code)]

#[allow(warnings)]
mod bindings;
mod helpers;

mod environment;
#[cfg(test)]
mod environment_test;
mod errors;
mod eval;
#[cfg(test)]
mod expression_test;
mod expressions;
mod parsers;
mod process;
mod standard_library;

#[cfg(test)]
mod eval_test;
mod example_tests;
#[cfg(test)]
mod process_test;

use bindings::exports::component::nana::nana::Guest as Nana;
use bindings::exports::wasi::cli::run::Guest as Command;
use eval::execute;
use expressions::print;
use standard_library::core::standard_library;

struct Component;

impl Command for Component {
    fn run() -> Result<(), ()> {
        execute(PROGRAM_CODE.to_owned(), standard_library());
        Ok(())
    }
}

impl Nana for Component {
    fn evaluate(name: String) -> String {
        let results = execute(name, standard_library());
        results
            .into_iter()
            .map(|(result, _env)| {
                result
                    .into_iter()
                    .map(|item| print(item))
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .collect::<Vec<_>>()
            .join("\n#-------- Results from processes in order of completion ----------------\n")
    }
}

bindings::export!(Component with_types_in bindings);

pub static PROGRAM_CODE: &str = include_str!("../examples/main.nana");
