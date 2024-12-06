use super::nana::Expression::{self, Macro, MacroCall, MacroName};
use std::collections::HashMap;

pub fn build_macros(
    expressions: Vec<Expression>,
    environment: HashMap<String, Expression>,
) -> (Expression, Vec<Expression>) {
    match expressions.as_slice() {
        [] => panic!("Empty expression list"),
        [MacroName(name), rest @ ..] => match environment.get(name) {
            Some(Macro(name, params, _)) => {
                // TODO: Rewrite this in a functional way with reduce
                let mut args = Vec::new();
                let mut new_rest = rest.to_vec();

                for _ in 0..params.len() {
                    let (arg, remainder) = build_macros(new_rest, environment.clone());
                    args.push(arg);
                    new_rest = remainder
                }
                (MacroCall(name.clone(), args.to_vec()), new_rest.to_vec())
            }
            Some(_) => panic!("A macro name should only ever point to a macro in the environment"),
            None => panic!("Macro was referenced but has not defined"),
        },
        // Any lexical expression that includes expressions needs to be handled
        // here. That's function calls, and lists for now. The expressions in a
        // macro object aren't lexical so won't have macro names for us to
        // build.
        _ => (
            expressions.first().unwrap().clone(),
            expressions[1..].to_vec(),
        ),
    }
}
