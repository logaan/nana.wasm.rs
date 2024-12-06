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
                let (args, new_rest) = (0..params.len()).fold(
                    (Vec::new(), rest.to_vec()),
                    |(mut args, curr_rest), _| {
                        let (arg, remainder) = build_macros(curr_rest, environment.clone());
                        // TODO: Switch to persistent data structures to avoid mutating.
                        // https://github.com/orium/rpds seems best maintained
                        args.push(arg);
                        (args, remainder)
                    },
                );
                (MacroCall(name.clone(), args), new_rest)
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
