use std::sync::Arc;

use im::{HashMap, Vector};

use crate::expressions::Environment;
use crate::expressions::RuntimeExpression::{
    self, BuiltinFunction, BuiltinMacro, Definition, Function, Hole, List, Macro, MacroCall,
    Number, Symbol, TaggedTuple,
};

use crate::parsers::macros::build_many_macros;
use crate::parsers::nana::program;
use crate::process::Process::{self, Complete};

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

            Process::run_in_sequence(eval_body).and_then(Arc::new(
                |results: Vector<RuntimeExpression>| Complete(results.last().unwrap().clone()),
            ))
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
                Some(a_macro) => macro_expand(a_macro.clone(), args, environment.clone())
                    .and_then(Arc::new(move |re| eval(re, environment.clone()))),
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
        Function(..) => Complete(expression),
        BuiltinMacro(..) => todo!("Do we eval macros?"),
        Macro(..) => todo!("Do we eval macros?"),
        Hole => todo!("I can't imagine what holes evaluate to"),
        Definition(..) => Complete(expression),
    }
}

pub fn execute_with_all_results(code: String, env: Environment) -> Vector<RuntimeExpression> {
    let (_, expressions) = program(&code).unwrap();
    let ast = build_many_macros(&expressions, &env);

    Process::run_in_sequence(
        ast.iter()
            .map(|expr| eval(expr.clone(), env.clone()))
            .collect(),
    )
    .run_until_complete()
}

pub fn execute(code: String, env: Environment) -> RuntimeExpression {
    let results = execute_with_all_results(code, env);
    results.last().unwrap().clone()
}
