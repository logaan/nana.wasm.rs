use im::HashMap;

use crate::parsers::macros::RuntimeExpression::{
    self, FunctionCall, Hole, List, Macro, MacroCall, Number, ValueName,
};

use crate::process::Process::{self, Complete};

fn eval(
    expression: RuntimeExpression,
    environment: &HashMap<String, RuntimeExpression>,
) -> Process<RuntimeExpression> {
    match expression {
        FunctionCall(_name, _args) => todo!(),
        MacroCall(_name, _args) => todo!(),

        List(expressions) => {
            let eval_processes = expressions
                .iter()
                .cloned()
                .map(|e| eval(e, environment))
                .collect::<im::Vector<_>>();

            Process::run_in_sequence(eval_processes)
                .and_then(|evaluated_expressions| Complete(List(evaluated_expressions)))
        }

        ValueName(name) => match environment.get(&name) {
            // TODO: Give this clone some thought
            Some(value) => Complete(value.clone()),
            None => panic!("{} not found", &name),
        },

        Number(_) => Complete(expression),
        RuntimeExpression::String(_) => Complete(expression),
        Macro(_, _, _) => Complete(expression),

        Hole => panic!("I can't imagine what holes evaluate to"),
    }
}
