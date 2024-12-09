use std::sync::Arc;

use im::{HashMap, Vector};

use crate::parsers::macros::RuntimeExpression::{
    self, Function, FunctionCall, Hole, List, Macro, MacroCall, Number, ValueName,
};

use crate::process::Process::{self, Complete};

pub fn apply(
    _function: RuntimeExpression,
    _args: Vector<RuntimeExpression>,
) -> Process<RuntimeExpression> {
    todo!()
}

pub fn eval(
    expression: RuntimeExpression,
    environment: &HashMap<String, RuntimeExpression>,
) -> Process<RuntimeExpression> {
    match expression {
        FunctionCall(name, args) => {
            let maybe_function = environment.get(&name);
            match maybe_function {
                Some(function) => {
                    let eval_processes = args
                        .iter()
                        .cloned()
                        // TODO: This should be dropped into a running. Imagine they had
                        // side effects. You'd want to space them out.
                        .map(|e| eval(e, environment))
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
                .map(|e| eval(e, environment))
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
        Function(_, _, _) => todo!("When would you actually eval a function?"),
        Macro(_, _, _) => Complete(expression),

        Hole => panic!("I can't imagine what holes evaluate to"),
    }
}
