use super::nana::LexicalExpression;
use im::{vector, Vector};
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum RuntimeExpression {
    Macro(String, Vector<String>, Vector<RuntimeExpression>),
    ValueName(String),
    FunctionCall(String, Vector<RuntimeExpression>),
    MacroCall(String, Vector<RuntimeExpression>),
    List(Vector<RuntimeExpression>),
    Number(u8),
    String(String),
    Hole,
}

pub fn build_macros(
    expressions: Vector<LexicalExpression>,
    environment: HashMap<String, RuntimeExpression>,
) -> (RuntimeExpression, Vector<LexicalExpression>) {
    let rest = expressions.skip(1);

    match expressions.head() {
        None => panic!("Empty expression list"),
        Some(LexicalExpression::MacroName(name)) => match environment.get(name) {
            Some(RuntimeExpression::Macro(name, params, _)) => {
                let (final_args, new_rest) =
                    (0..params.len()).fold((Vector::new(), rest), |(mut args, curr_rest), _| {
                        let (arg, remainder) = build_macros(curr_rest, environment.clone());
                        // TODO: Switch to persistent data structures to avoid mutating.
                        // https://github.com/orium/rpds seems best maintained
                        args.push_back(arg);
                        (args, remainder)
                    });
                (
                    RuntimeExpression::MacroCall(name.clone(), final_args),
                    new_rest,
                )
            }
            Some(_) => panic!("A macro name should only ever point to a macro in the environment"),
            None => panic!("Macro was referenced but has not defined"),
        },
        Some(LexicalExpression::List(expressions)) => (
            RuntimeExpression::List(build_many_macros(
                expressions.into(),
                vector![],
                environment.clone(),
            )),
            rest,
        ),
        Some(LexicalExpression::FunctionCall(name, expressions)) => (
            RuntimeExpression::FunctionCall(
                name.to_string(),
                build_many_macros(expressions.into(), vector![], environment.clone()),
            ),
            rest,
        ),
        Some(LexicalExpression::ValueName(name)) => {
            (RuntimeExpression::ValueName(name.to_string()), rest)
        }
        Some(LexicalExpression::Number(value)) => (RuntimeExpression::Number(*value), rest),
        Some(LexicalExpression::String(value)) => {
            (RuntimeExpression::String(value.to_string()), rest)
        }
        Some(LexicalExpression::Hole) => (RuntimeExpression::Hole, rest),
    }
}

// TODO: This isn't gaurenteed to receive TCO. Should switch to an iterative
// implementation.
fn build_many_macros(
    incoming_exprs: Vector<LexicalExpression>,
    outgoing_exprs: Vector<RuntimeExpression>,
    environment: HashMap<String, RuntimeExpression>,
) -> Vector<RuntimeExpression> {
    if incoming_exprs.is_empty() {
        outgoing_exprs
    } else {
        let (new_outgoing_expr, new_incoming_exprs) =
            build_macros(incoming_exprs, environment.clone());
        let mut new_outgoing_exprs = outgoing_exprs.clone();
        new_outgoing_exprs.push_back(new_outgoing_expr);
        build_many_macros(new_incoming_exprs, new_outgoing_exprs, environment)
    }
}
