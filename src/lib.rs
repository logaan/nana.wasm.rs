#![allow(dead_code)]

#[allow(warnings)]
mod bindings;
mod helpers;

mod eval;
mod expressions;
mod parsers;
mod process;
mod standard_library;

#[cfg(test)]
mod eval_test;
#[cfg(test)]
mod process_test;

use crate::eval::eval;

use crate::expressions::RuntimeExpression::{self, BuiltinFunction};
use bindings::exports::wasi::cli::run::Guest as Command;
use expressions::Environment;
use im::hashmap;
use parsers::macros::build_macros;
use parsers::nana::program;
use process::Process;

struct Component;

fn execute(code: String, env: Environment) -> RuntimeExpression {
    program(&code)
        .and_then(|(_, es)| Ok(build_macros(&es, &env)))
        .and_then(|(ast, _)| Ok(eval(ast, env)))
        .unwrap()
        .run_until_complete()
}

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
