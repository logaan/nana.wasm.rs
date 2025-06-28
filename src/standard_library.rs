use core::panic;
use std::sync::Arc;

use im::{hashmap, vector, Vector};

use crate::eval::{eval, execute_with_env, quote};
use crate::expressions::RuntimeExpression::{
    BuiltinFunction, BuiltinMacro, Definition, Function, Hole, Keyword, List, Macro, MacroCall,
    Number, String as NString, Symbol, TaggedTuple,
};
use crate::expressions::{Environment, RuntimeExpression};
use crate::process::Process::Complete;
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
        // TODO: If someone sees a function call on the left they're going to
        // think it's being evaluated. Here's maybe the issue with my current
        // tagged tuples, because they're tagged with strings there's no
        // distinction between one that'll evaluate to a function vs one that we
        // might be using to store a tag. Like (foo 1 2) vs ('foo 1 2) or (:foo
        // 1 2)
        //
        // If we ignore the syntax we can maybe get to the heart of it:
        //
        //   Match data
        //     [tagged-tuple(function, [arg1, arg2]) [function, arg1, arg2]]
        //
        //   Match data
        //     [function(arg1, arg2) [function, arg1, arg2]]
        //
        // These two examples should do the same thing: pull the tag and the
        // args out, because in both cases `function`, `arg1`, and `arg2` are
        // each symbols. The complication comes from the similarity between the two.
        // If we want to be able to match on `function` in the second exampel
        // then `tagged-tuple` in the first example also becomes a match, rather
        // than acting as a tag.
        //
        // We also have the issue of being unable to specify a concrete type
        // that we want to match on:
        //
        //   Match data
        //     [tagged-tuple(point, [x, y]) [point, x, y]]
        //
        //   Match data
        //     [point(x, y) [point, x, y]]
        //
        // In these cases "point" isn't acting as a way of saying "match against
        // a point" instead it's just binding whatever the function happens to
        // be to a variable called "point".
        //
        // Keywords help us here. There's no ambiguity to:
        //
        //   Match data
        //     [tagged-tuple(:point, [x, y]) [x, y]]
        //
        //   Match data
        //     [:point(x, y) [x, y]]
        //
        // But it forces quite a concrete match style. You can't use
        // abstractions to match. But that's probably fine. Abstractions can
        // provide methods for matching with If:
        //
        // If matches(SomeHashMap, OtherHashMap) do-a() do-b()
        //
        // I guess it'll mean people can't swap out the foundational data
        // structures, as I'd often like to for more functional ones. But maybe
        // macros save us there. Libraries for those data structures can write
        // their own Match fn.
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
            println!("{:?}", args.clone());
            Complete(args.head().unwrap().clone())
        }),

        s!("panic") => BuiltinFunction(|args| {
            panic!("Panic called with {:?}", args);
        }),

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
    })
}

pub fn standard_library() -> Environment {
    let (_result, new_env) = execute_with_env(PROGRAM_CODE.to_owned(), builtins());
    new_env
}

pub static PROGRAM_CODE: &str = include_str!("../examples/standard_library.nana");
