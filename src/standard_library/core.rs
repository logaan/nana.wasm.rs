use core::panic;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use im::{hashmap, vector, Vector};

use crate::eval::{apply, eval, execute, quote};
use crate::expressions::RuntimeExpression::{
    BuiltinFunction, BuiltinMacro, Definition, Function, Hole, Keyword, List, Macro, MacroCall,
    Number, String as NString, Symbol, TaggedTuple,
};
use crate::expressions::{print_many, Environment, RuntimeExpression};
use crate::process::Process::{Complete, Spawn};
use crate::s;

fn does_match(pattern: RuntimeExpression, value: RuntimeExpression) -> Option<Environment> {
    match pattern {
        Symbol(name) => Some(Environment::from(hashmap! {name => value})),
        Hole => Some(Environment::new()),
        List(patterns) if patterns.len() == 0 => match value {
            List(values) if values.len() == 0 => Some(Environment::new()),
            _ => None,
        },
        List(patterns) => match value {
            List(values) if values.len() == patterns.len() => patterns
                .iter()
                .cloned()
                .zip(values.iter().cloned())
                .map(|(pattern, value)| does_match(pattern, value))
                .fold(Some(Environment::new()), |acc, bindings| {
                    acc.and_then(|acc| {
                        bindings.and_then(|bindings| {
                            for (key, value) in bindings.iter() {
                                if let Some(existing_value) = acc.get(&key) {
                                    if existing_value != value {
                                        return None;
                                    }
                                }
                            }
                            Some(acc.union(bindings))
                        })
                    })
                }),
            _ => None,
        },
        NString(_) if pattern == value => Some(Environment::new()),
        NString(_) => None,
        Number(_) if pattern == value => Some(Environment::new()),
        Number(_) => None,
        Keyword(_) if pattern == value => Some(Environment::new()),
        Keyword(_) => None,
        TaggedTuple(..) => None,
        MacroCall(..) => None, // Macro calls should maybe evaluate and then compare
        BuiltinFunction(_) => None, // Builtins shouldn't be comparable
        Function(..) => None,  // Functions shouldn't be comparable
        BuiltinMacro(..) => None, // Builtins shouldn't be comparable
        Macro(..) => None,     // Macros shouldn't be comparable
        Definition(..) => None, // Definitions should just be at the top level.
    }
}

pub fn builtins() -> Environment {
    Environment::from(hashmap! {
        // TODO: Make expressions print themselves in a readable form
        s!("log") => BuiltinFunction(|args| {
            println!("{}", print_many(args.clone(), " "));
            Complete(args.head().unwrap().clone())
        }),

        s!("panic") => BuiltinFunction(|args| {
            panic!("Panic called with {:?}", args);
        }),

        // TODO: equality

        s!("add") => BuiltinFunction(|mut args| {
            if args.len() == 2 {
                match [args.pop_front().unwrap(), args.pop_front().unwrap()] {
                    [Number(l), Number(r)] => Complete(Number(l + r)),
                    _ => panic!("add takes exactly 2 numbers")
                }
            } else {
                panic!("add takes exactly 2 numbers")
            }
        }),

        s!("subtract") => BuiltinFunction(|mut args| {
            if args.len() == 2 {
                match [args.pop_front().unwrap(), args.pop_front().unwrap()] {
                    [Number(l), Number(r)] => Complete(Number(l - r)),
                    _ => panic!("subtract takes exactly 2 numbers")
                }
            } else {
                panic!("subtract takes exactly 2 numbers")
            }
        }),

        s!("multiply") => BuiltinFunction(|mut args| {
            if args.len() == 2 {
                match [args.pop_front().unwrap(), args.pop_front().unwrap()] {
                    [Number(l), Number(r)] => Complete(Number(l * r)),
                    _ => panic!("multiply takes exactly 2 numbers")
                }
            } else {
                panic!("multiply takes exactly 2 numbers")
            }
        }),

        s!("divide") => BuiltinFunction(|mut args| {
            if args.len() == 2 {
                match [args.pop_front().unwrap(), args.pop_front().unwrap()] {
                    [Number(l), Number(r)] => Complete(Number(l / r)),
                    _ => panic!("divide takes exactly 2 numbers")
                }
            } else {
                panic!("divide takes exactly 2 numbers")
            }
        }),

        s!("remainder") => BuiltinFunction(|mut args| {
            if args.len() == 2 {
                match [args.pop_front().unwrap(), args.pop_front().unwrap()] {
                    [Number(l), Number(r)] => Complete(Number(l % r)),
                    _ => panic!("remainder takes exactly 2 numbers")
                }
            } else {
                panic!("remainder takes exactly 2 numbers")
            }
        }),

        s!("Def") => BuiltinMacro(
            vector![
              s!("name"),
              s!("value")
            ],
            |mut args, env| {
                if args.len() == 2 {
                    let name = args.pop_front().unwrap();
                    let value = args.pop_front().unwrap();

                    match name {
                        Symbol(name) | NString(name) => {
                            let new_env = env.prepare(name.clone());
                            eval(value, new_env.clone()).and_then(Arc::new(move |result| {
                                new_env.provide(&name, result.clone()).expect("Providing a prepared value should not fail");
                                Complete(Definition(name.clone(), Arc::new(result.clone())))
                            }))
                        },
                        _ => panic!("Def takes a symbol and a value")
                    }
                } else {
                    panic!("Def takes exactly 2 arguments")
                }
            }
        ),

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
                                        // TODO: I think this is probably why
                                        // shadowing isn't working correctly. WE
                                        // shouldn't be evalling here. Instead
                                        // we should be returning a blob of code
                                        // that uses calls to llambdas to define
                                        // local variables and then maybe makes
                                        // use of a Builtin If statement or
                                        // something. More should be happening
                                        // at runtime rather than macro expand time.
                                        // TODO: Split macro-expand out as a
                                        // function so macros can be debugged
                                        // better. Implement the existing macro
                                        // running stuff in terms of
                                        // macro-expand followed by eval.
                                        Some(bindings) => return eval(body.clone(), env.clone().union(bindings)),
                                        None => {}
                                    }
                                }
                                Complete(TaggedTuple(Arc::new(Keyword(s!("error"))), vector![Keyword(s!("no-match-found")), value.clone()]))
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

        s!("Quote") => BuiltinMacro(
            vector![
              s!("value")
            ],
            |mut args, env| {
                if args.len() == 1 {
                    quote(args.pop_front().unwrap(), env)
                } else {
                    panic!("Quote takes exactly 1 argument")
                }
            }
        ),

        s!("Macro") => BuiltinMacro(
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
                                _ => panic!("Macro params must be symbols")
                            }).cloned().collect::<Vector<String>>();
                            Complete(Macro(param_strings, env, vector![body]))
                        },
                        _ => panic!("Macro takes a list of params and a single body expression")
                    }
                } else {
                    panic!("Macro takes exactly 2 arguments")
                }
            }
        ),
        s!("time") => BuiltinFunction(|args| {
            if args.len() == 0 {
                let start = SystemTime::now();
                let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

                Complete(Number(since_epoch.as_millis()))
            } else {
                panic!("time takes no arguments")
            }
        }),

        // TODO: spawn needs to:
        // 1. Create a promise
        // 2. Wrap the readable side of the promise in a Complete process
        // 3. Create a Running process that will call the Fn and
        // when it completes use its return value to resolve the
        // process.
        // 4. Return the (Complete(..), Some(Running(..)))
        s!("spawn") => BuiltinFunction(|mut args| {
            if args.len() == 1 {
                let first_arg = args.pop_front().unwrap();
                match first_arg {
                    Function(..) => Spawn(Arc::new(Complete(Keyword(s!("process-spawned")))),
                                          vector![apply(first_arg, vector![])]),
                    // TODO: Should probably support BuiltinFunction too
                    _ => panic!("spawn takes 1 function (with no arguments) as an argument")
                }
            } else {
                panic!("spawn takes 1 function (with no arguments) as an argument")
            }
        })
    })
}

pub fn standard_library() -> Environment {
    let (_result, new_env) = execute(PROGRAM_CODE.to_owned(), builtins())
        .head()
        .unwrap()
        .clone();
    new_env
}

pub static PROGRAM_CODE: &str = include_str!("../../examples/standard_library.nana");
