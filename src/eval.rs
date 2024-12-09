use im::HashMap;

use crate::parsers::macros::RuntimeExpression::{
    self, FunctionCall, Hole, List, Macro, MacroCall, Number, ValueName,
};

use crate::process::FnProcess::{self, Complete};

fn eval(
    expression: RuntimeExpression,
    environment: &HashMap<String, RuntimeExpression>,
) -> FnProcess<RuntimeExpression> {
    match expression {
        FunctionCall(_name, _args) => todo!(),
        MacroCall(_name, _args) => todo!(),
        // TODO: We can use run_in_sequence to evaluate everything in the list.
        // But that'll just complete with Process<Vector<RuntimeExpression>>.
        // What we need is a Process<RuntimeExpression>, we need to be able to
        // grab the final results and wrap them in a List.
        //
        // I think there's two ways of doing that. Either we have a wrapper
        // process that runs an inner process until completion and then does the
        // transform. Or we add a `then` field to Running and update `step` so
        // that it calls then rather than Complete.
        List(_expressions) => todo!(),

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
