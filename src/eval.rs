use std::fs::File;
use std::io::Read;
use std::path::Path;

use std::sync::Arc;

use im::{vector, HashMap, Vector};

use crate::expressions::RuntimeExpression::{
    self, BuiltinFunction, BuiltinMacro, Definition, Function, Hole, List, Macro, MacroCall,
    Number, Symbol, TaggedTuple,
};
use crate::expressions::{is_comment, Environment, LexicalExpression};

use crate::parsers::macros::build_macros;
use crate::parsers::nana::program;
use crate::process::Process::{self, Complete, Running};

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
        _ => panic!("Not a function"),
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
        _ => panic!("Not a macro"),
    }
}

pub fn eval(expression: RuntimeExpression, environment: Environment) -> Process<RuntimeExpression> {
    match expression {
        TaggedTuple(name, args) => {
            let maybe_function = environment.get(&name);
            match maybe_function {
                Some(function) => {
                    let environment = environment.clone();
                    let eval_processes = args
                        .iter()
                        .cloned()
                        .map(move |e| {
                            let environment = environment.clone();
                            Process::Running(Arc::new(move || eval(e.clone(), environment.clone())))
                        })
                        .collect::<im::Vector<_>>();

                    let function = function.clone();

                    Process::run_in_sequence(eval_processes).and_then(Arc::new(
                        move |evaluated_expressions| apply(function.clone(), evaluated_expressions),
                    ))
                }
                _ => panic!("No function of that name found"),
            }
        }

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
                        _ => panic!("No macro of that name found"),
                    }
                }
                _ => panic!("No macro of that name found"),
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
            None => panic!("{} not found", &name),
        },

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

fn execute_with_definitions_and_process(
    work: Vector<LexicalExpression>,
    env: Environment,
    mut results: Vector<RuntimeExpression>,
    process: Process<RuntimeExpression>,
) -> Process<(Vector<RuntimeExpression>, Environment)> {
    if process.is_complete() {
        let (new_seed, new_env) = match process.result().unwrap() {
            Definition(name, value) => ((*value).clone(), env.add(name, (*value).clone())),
            value => (value, env),
        };

        results.push_back(new_seed.clone());
        Running(Arc::new(move || {
            execute_with_definitions(work.clone(), new_env.clone(), results.clone())
        }))
    } else {
        Running(Arc::new(move || {
            execute_with_definitions_and_process(
                work.clone(),
                env.clone(),
                results.clone(),
                process.step(),
            )
        }))
    }
}

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

pub fn execute(code: String, env: Environment) -> Vector<RuntimeExpression> {
    let (_err, expressions) = program(&code).unwrap();
    let comments_stripped = expressions.into_iter().filter(|e| !is_comment(e)).collect();
    let process = execute_with_definitions(comments_stripped, env, vector![]);
    let (results, _env) = process.run_until_complete();
    results
}
