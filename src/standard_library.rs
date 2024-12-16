use std::sync::Arc;

use im::{hashmap, vector, Vector};

use crate::eval::eval;
use crate::expressions::RuntimeExpression::{
    BuiltinFunction, BuiltinMacro, Function, Hole, List, Symbol,
};
use crate::expressions::{Environment, RuntimeExpression};
use crate::process::Process::Complete;
use crate::s;

fn does_match(pattern: RuntimeExpression, value: RuntimeExpression) -> Option<Environment> {
    match pattern {
        Symbol(name) => Some(hashmap! {name => value}),
        Hole => Some(hashmap! {}),
        _ if pattern == value => Some(hashmap! {}),
        _ => None,
    }
}

pub fn standard_library() -> Environment {
    // TODO: Write constructors for every `RuntimeExpression`.
    hashmap! {
        s!("dec") => BuiltinFunction(|mut args| {
            if args.len() == 1 {
                match args.pop_front().unwrap() {
                    RuntimeExpression::Number(n) => Complete(RuntimeExpression::Number(n - 1)),
                    _ => panic!("dec takes a number")
                }
            } else {
                panic!("dec takes exactly 1 argument")
            }
        }),
        s!("Match") => BuiltinMacro(
            vector![
              s!("value"),
              s!("cases")
            ],
            |mut args, env| {
                if args.len() == 2 {
                    let value = args.pop_front().unwrap();
                    let cases = args.pop_front().unwrap();

                    eval(value, env.clone()).and_then(Arc::new(move |value|
                        match cases.clone() {
                            List(cases) => {
                                if cases.len() % 2 != 0 {
                                    panic!("Cases must be a list with an even number of elements")
                                }

                                let mut iter = cases.iter();
                                while let (Some(pattern), Some(body)) = (iter.next(), iter.next()) {
                                    match does_match(pattern.clone(), value.clone()) {
                                        Some(bindings) => return eval(body.clone(), env.clone().union(bindings)),
                                        None => {}
                                    }
                                }
                                panic!("No match found")
                            },
                            _ => panic!("Match takes a value and a list of cases")
                        }
                    ))
                } else {
                    panic!("Match takes exactly 2 arguments")
                }
            }
        ),
        s!("Fn") => BuiltinMacro(
            vector![
              s!("params"),
              s!("body")
            ],
            |mut args, env| {
                if args.len() == 2 {
                    let params = args.pop_front().unwrap();
                    let body = args.pop_front().unwrap();

                    match params {
                        List(params) => {
                            let param_strings = params.iter().map(|p| match p {
                                Symbol(s) => s,
                                _ => panic!("Func params must be ValueNames")
                            }).cloned().collect::<Vector<String>>();
                            Complete(Function(param_strings, env, vector![body]))
                        },
                        _ => panic!("Fn takes a list of params and a single body expression")
                    }
                } else {
                    panic!("Fn takes exactly 2 arguments")
                }
            }
        ),
    }
}
