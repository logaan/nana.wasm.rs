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
                let (final_args, new_rest) = (0..params.len()).fold(
                    (Vec::new(), rest.to_vec()),
                    |(mut args, curr_rest), _| {
                        let (arg, remainder) = build_macros(curr_rest, environment.clone());
                        // TODO: Switch to persistent data structures to avoid mutating.
                        // https://github.com/orium/rpds seems best maintained
                        args.push(arg);
                        (args, remainder)
                    },
                );
                (MacroCall(name.clone(), final_args), new_rest)
            }
            Some(_) => panic!("A macro name should only ever point to a macro in the environment"),
            None => panic!("Macro was referenced but has not defined"),
        },
        [Expression::List(expressions), rest @ ..] => (
            Expression::List(build_many_macros(
                expressions.clone(),
                vec![],
                environment.clone(),
            )),
            rest.to_vec(),
        ),
        [Expression::FunctionCall(name, expressions), rest @ ..] => (
            Expression::FunctionCall(
                name.to_string(),
                build_many_macros(expressions.clone(), vec![], environment.clone()),
            ),
            rest.to_vec(),
        ),
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

// TODO: This isn't gaurenteed to receive TCO. Should switch to an iterative
// implementation.
fn build_many_macros(
    incoming_exprs: Vec<Expression>,
    outgoing_exprs: Vec<Expression>,
    environment: HashMap<String, Expression>,
) -> Vec<Expression> {
    match incoming_exprs.as_slice() {
        [] => outgoing_exprs,
        _ => {
            let (new_outgoing_expr, new_incoming_exprs) =
                build_macros(incoming_exprs, environment.clone());
            let mut new_outgoing_exprs = outgoing_exprs.clone();
            new_outgoing_exprs.push(new_outgoing_expr);
            build_many_macros(new_incoming_exprs, new_outgoing_exprs, environment)
        }
    }
}
