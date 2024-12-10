use std::sync::Arc;

use im::{HashMap, Vector};

use crate::expressions::RuntimeExpression::{
    self, BuiltinFunction, FunctionCall, Hole, List, Macro, MacroCall, Number, ValueName,
};

use crate::process::Process::{self, Complete};

pub fn apply(
    function: RuntimeExpression,
    args: Vector<RuntimeExpression>,
) -> Process<RuntimeExpression> {
    match function {
        BuiltinFunction(body) => (body)(args),
        _ => panic!("Not a function"),
    }
}

pub fn eval(
    expression: RuntimeExpression,
    environment: HashMap<String, RuntimeExpression>,
) -> Process<RuntimeExpression> {
    match expression {
        FunctionCall(name, args) => {
            let maybe_function = environment.get(&name);
            match maybe_function {
                Some(function) => {
                    // TODO: I clone this environment three times. There has got
                    // to be a better way.
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

        MacroCall(_name, _args) => todo!(),

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

        ValueName(name) => match environment.get(&name) {
            // TODO: Give this clone some thought
            Some(value) => Complete(value.clone()),
            None => panic!("{} not found", &name),
        },

        Number(_) => Complete(expression),
        RuntimeExpression::String(_) => Complete(expression),

        BuiltinFunction(..) => todo!("When would you actually eval a function?"),
        Macro(..) => todo!("Do we eval macros?"),
        Hole => todo!("I can't imagine what holes evaluate to"),
    }
}
