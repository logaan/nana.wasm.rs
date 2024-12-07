use super::nana::LexicalExpression;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum RuntimeExpression {
    Macro(String, Vec<String>, Vec<RuntimeExpression>),
    ValueName(String),
    FunctionCall(String, Vec<RuntimeExpression>),
    MacroCall(String, Vec<RuntimeExpression>),
    List(Vec<RuntimeExpression>),
    Number(u8),
    String(String),
    Hole,
}

pub fn build_macros(
    expressions: Vec<LexicalExpression>,
    environment: HashMap<String, RuntimeExpression>,
) -> (RuntimeExpression, Vec<LexicalExpression>) {
    match expressions.as_slice() {
        [] => panic!("Empty expression list"),
        [LexicalExpression::MacroName(name), rest @ ..] => match environment.get(name) {
            Some(RuntimeExpression::Macro(name, params, _)) => {
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
                (
                    RuntimeExpression::MacroCall(name.clone(), final_args),
                    new_rest,
                )
            }
            Some(_) => panic!("A macro name should only ever point to a macro in the environment"),
            None => panic!("Macro was referenced but has not defined"),
        },
        [LexicalExpression::List(expressions), rest @ ..] => (
            RuntimeExpression::List(build_many_macros(
                expressions.clone(),
                vec![],
                environment.clone(),
            )),
            rest.to_vec(),
        ),
        [LexicalExpression::FunctionCall(name, expressions), rest @ ..] => (
            RuntimeExpression::FunctionCall(
                name.to_string(),
                build_many_macros(expressions.clone(), vec![], environment.clone()),
            ),
            rest.to_vec(),
        ),
        [LexicalExpression::ValueName(name), rest @ ..] => (
            RuntimeExpression::ValueName(name.to_string()),
            rest.to_vec(),
        ),
        [LexicalExpression::Number(value), rest @ ..] => {
            (RuntimeExpression::Number(*value), rest.to_vec())
        }
        [LexicalExpression::String(value), rest @ ..] => {
            (RuntimeExpression::String(value.to_string()), rest.to_vec())
        }
        [LexicalExpression::Hole, rest @ ..] => (RuntimeExpression::Hole, rest.to_vec()),
    }
}

// TODO: This isn't gaurenteed to receive TCO. Should switch to an iterative
// implementation.
fn build_many_macros(
    incoming_exprs: Vec<LexicalExpression>,
    outgoing_exprs: Vec<RuntimeExpression>,
    environment: HashMap<String, RuntimeExpression>,
) -> Vec<RuntimeExpression> {
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
