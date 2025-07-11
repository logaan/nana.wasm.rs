use std::fs::File;
use std::io::Read;
use std::path::Path;

use std::sync::Arc;

use im::{vector, HashMap, Vector};

use crate::errors::{error_with_message, not_found_error};
use crate::expressions::RuntimeExpression::{
    self, BuiltinFunction, BuiltinMacro, Definition, Function, Hole, Keyword, List, Macro,
    MacroCall, Number, String as NString, Symbol, TaggedTuple,
};
use crate::expressions::{is_comment, Environment, LexicalExpression};

use crate::parsers::macros::build_macros;
use crate::parsers::nana::program;
use crate::process::Process::{self, Complete, Running, Spawn};
use crate::s;

pub fn read_code(path: &str) -> String {
    let mut file = File::open(Path::new(path)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn apply(
    function: RuntimeExpression,
    args: Vector<RuntimeExpression>,
) -> Process<RuntimeExpression> {
    match function {
        BuiltinFunction(body) => (body)(args),
        Function(params, environment, body) => {
            let new_env = environment.union(Environment::from(
                params
                    .iter()
                    .cloned()
                    .zip(args.iter().cloned())
                    .collect::<HashMap<_, _>>(),
            ));

            let eval_body = body
                .iter()
                .cloned()
                .map(move |e| {
                    let new_env = new_env.clone();
                    Process::Running(Arc::new(move || eval(e.clone(), new_env.clone())))
                })
                .collect::<im::Vector<_>>();

            Process::run_in_sequence_tco(eval_body)
        }
        _ => error_with_message(
            "invalid-application",
            "You've called an invalid value as a function",
        ),
    }
}

pub fn macro_expand(
    macro_expression: RuntimeExpression,
    args: Vector<RuntimeExpression>,
    environment: Environment,
) -> Process<RuntimeExpression> {
    match macro_expression {
        BuiltinMacro(_params, body) => (body)(args, environment),
        Macro(params, environment, body) => {
            let new_env = environment.union(Environment::from(
                params
                    .iter()
                    .cloned()
                    .zip(args.iter().cloned())
                    .collect::<HashMap<_, _>>(),
            ));

            let eval_body = body
                .iter()
                .cloned()
                .map(move |e| {
                    let new_env = new_env.clone();
                    Process::Running(Arc::new(move || eval(e.clone(), new_env.clone())))
                })
                .collect::<im::Vector<_>>();

            Process::run_in_sequence(eval_body).and_then(Arc::new(
                |results: Vector<RuntimeExpression>| Complete(results.last().unwrap().clone()),
            ))
        }
        _ => error_with_message("invalid-macro-expand", "Not a macro"),
    }
}

pub fn eval(expression: RuntimeExpression, environment: Environment) -> Process<RuntimeExpression> {
    match expression {
        TaggedTuple(tag, args) => match (*tag).clone() {
            // TODO: This should just eval the tag and then apply it to the tuple
            Symbol(name) => {
                let maybe_function = environment.get(&name);
                match maybe_function {
                    Some(function) => {
                        let function = function.clone();

                        eval_expressions(&args, &environment).and_then(Arc::new(
                            move |evaluated_expressions| {
                                apply(function.clone(), evaluated_expressions)
                            },
                        ))
                    }
                    _ => not_found_error(&format!("No function '{}' found", name)),
                }
            }
            Keyword(_) => eval_expressions(&args, &environment).and_then(Arc::new(
                move |evaluated_expressions| {
                    Complete(TaggedTuple(tag.clone(), evaluated_expressions))
                },
            )),
            // TODO: Eval tagged tuples other than Symbols and Keywords
            _ => todo!(),
        },

        MacroCall(name, args) => {
            let maybe_macro = environment.get(&name);
            match maybe_macro {
                Some(a_macro) => {
                    let expanded = macro_expand(a_macro.clone(), args, environment.clone());
                    match a_macro {
                        Macro(..) => {
                            expanded.and_then(Arc::new(move |re| eval(re, environment.clone())))
                        }
                        BuiltinMacro(..) => expanded,
                        _ => not_found_error("No macro of that name found"),
                    }
                }
                _ => not_found_error("No macro of that name found"),
            }
        }

        List(expressions) => {
            let eval_processes = expressions
                .iter()
                .cloned()
                .map(move |e| eval(e, environment.clone()))
                .collect::<im::Vector<_>>();

            Process::run_in_sequence(eval_processes).and_then(Arc::new(|evaluated_expressions| {
                Complete(List(evaluated_expressions))
            }))
        }

        Symbol(name) => match environment.get(&name) {
            Some(value) => Complete(value.clone()),
            None => not_found_error(&format!("{} not found", &name)),
        },

        Keyword(_) => Complete(expression),
        Number(_) => Complete(expression),
        RuntimeExpression::String(_) => Complete(expression),

        BuiltinFunction(..) => todo!("When would you actually eval a function?"),
        Function(..) => todo!("Evalling a function"),
        BuiltinMacro(..) => todo!("Do we eval macros?"),
        Macro(..) => todo!("Do we eval macros?"),
        Hole => todo!("I can't imagine what holes evaluate to"),
        Definition(..) => todo!("Evalling a definition"),
    }
}

// Quote needs to return a process. Because when we hit unquote we're going to
// have to eval.
pub fn quote(value: RuntimeExpression, env: Environment) -> Process<RuntimeExpression> {
    match value {
        BuiltinFunction(_) => Complete(value),
        Function(params, env, body) => {
            let new_env = env.clone();
            let processes = body
                .iter()
                .cloned()
                .map(move |re| quote(re, env.clone()))
                .collect();
            Process::run_in_sequence(processes).and_then(Arc::new(move |new_body| {
                Complete(Function(params.clone(), new_env.clone(), new_body))
            }))
        }
        TaggedTuple(tag, values) => {
            let processes = values
                .iter()
                .cloned()
                .map(move |re| quote(re, env.clone()))
                .collect();
            Process::run_in_sequence(processes).and_then(Arc::new(move |new_values| {
                Complete(TaggedTuple(tag.clone(), new_values))
            }))
        }
        Hole => Complete(value),
        List(values) => {
            let processes = values
                .iter()
                .cloned()
                .map(move |re| quote(re, env.clone()))
                .collect();
            Process::run_in_sequence(processes)
                .and_then(Arc::new(|new_values| Complete(List(new_values))))
        }
        BuiltinMacro(_, _) => Complete(value),
        Macro(params, env, body) => {
            let new_env = env.clone();
            let processes = body
                .iter()
                .cloned()
                .map(move |re| quote(re, env.clone()))
                .collect();
            Process::run_in_sequence(processes).and_then(Arc::new(move |new_body| {
                Complete(Macro(params.clone(), new_env.clone(), new_body))
            }))
        }
        MacroCall(name, args) => {
            if name == s!("Unquote") {
                let mut args = args;
                let value = args.pop_front().unwrap();
                eval(value, env)
            } else {
                let processes = args
                    .iter()
                    .cloned()
                    .map(move |re| quote(re, env.clone()))
                    .collect();
                Process::run_in_sequence(processes).and_then(Arc::new(move |new_args| {
                    Complete(MacroCall(name.clone(), new_args))
                }))
            }
        }
        Number(_) => Complete(value),
        NString(_) => Complete(value),
        Keyword(_) => Complete(value),
        Symbol(_) => Complete(value),
        Definition(name, value) => {
            let process = quote((*value).clone(), env);
            process.and_then(Arc::new(move |new_value| {
                Complete(Definition(name.clone(), Arc::new(new_value)))
            }))
        }
    }
}

fn execute_with_definitions_and_process(
    work: Vector<LexicalExpression>,
    env: Environment,
    mut results: Vector<RuntimeExpression>,
    process: Process<RuntimeExpression>,
) -> Process<(Vector<RuntimeExpression>, Environment)> {
    match process {
        Complete(result) => {
            let (new_seed, new_env) = match result {
                Definition(name, value) => ((*value).clone(), env.add(name, (*value).clone())),
                value => (value, env),
            };

            results.push_back(new_seed.clone());
            Running(Arc::new(move || {
                execute_with_definitions(work.clone(), new_env.clone(), results.clone())
            }))
        }
        Running(stepable) => Running(Arc::new(move || {
            execute_with_definitions_and_process(
                work.clone(),
                env.clone(),
                results.clone(),
                stepable.step(),
            )
        })),
        Spawn(continuation, spawned_processes) => Spawn(
            Arc::new(execute_with_definitions_and_process(
                work.clone(),
                env.clone(),
                results.clone(),
                (*continuation).clone(),
            )),
            spawned_processes
                .iter()
                .map(move |process| {
                    execute_with_definitions_and_process(
                        vector![],
                        env.clone(),
                        vector![],
                        (*process).clone(),
                    )
                })
                .collect::<im::Vector<_>>(),
        ),
    }
}

// Runs lexical expressions, basically code prior to macro parsing, and returns
// a process that will eventually produce the result of each top level
// expression, as well as an environment containing all new definitions
pub fn execute_with_definitions(
    work: Vector<LexicalExpression>,
    env: Environment,
    results: Vector<RuntimeExpression>,
) -> Process<(Vector<RuntimeExpression>, Environment)> {
    if work.is_empty() {
        Complete((results, env))
    } else {
        let (head, remaining_work) = build_macros(&work, &env);
        match head {
            Some(first_expression) => {
                let first_process = eval(first_expression, env.clone());
                execute_with_definitions_and_process(remaining_work, env, results, first_process)
            }
            None => Complete((vector![], env)),
        }
    }
}

// Runs a string of code and returns the result of each top level expression, as
// well as an environment containing all new definitions
pub fn execute(code: String, env: Environment) -> Vector<(Vector<RuntimeExpression>, Environment)> {
    let (_err, expressions) = program(&code).unwrap();
    let comments_stripped = expressions.into_iter().filter(|e| !is_comment(e)).collect();
    let process = execute_with_definitions(comments_stripped, env, vector![]);
    process.run_until_complete()
}

pub fn execute_once(code: String, env: Environment) -> Vector<RuntimeExpression> {
    let (result, _new_env) = execute(code, env).head().unwrap().clone();
    result
}

fn eval_expressions(
    args: &Vector<RuntimeExpression>,
    environment: &Environment,
) -> Process<Vector<RuntimeExpression>> {
    Process::run_in_sequence(
        args.iter()
            .cloned()
            .map(|e| {
                let environment = environment.clone();
                Process::Running(Arc::new(move || eval(e.clone(), environment.clone())))
            })
            .collect::<im::Vector<_>>(),
    )
}
