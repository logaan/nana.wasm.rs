use super::nana::LexicalExpression;
use im::{vector, HashMap, Vector};

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
    expressions: &Vector<LexicalExpression>,
    environment: &HashMap<String, RuntimeExpression>,
) -> (RuntimeExpression, Vector<LexicalExpression>) {
    let rest = expressions.skip(1);

    match expressions.head() {
        None => panic!("Empty expression list"),
        Some(LexicalExpression::MacroName(name)) => match environment.get(name) {
            Some(RuntimeExpression::Macro(name, params, _)) => {
                let (final_args, new_rest) =
                    (0..params.len()).fold((Vector::new(), rest), |(args, curr_rest), _| {
                        let (arg, remainder) = build_macros(&curr_rest, &environment);
                        let new_args = args + Vector::unit(arg);
                        (new_args, remainder)
                    });
                (
                    RuntimeExpression::MacroCall(name.to_string(), final_args),
                    new_rest,
                )
            }
            Some(_) => panic!("A macro name should only ever point to a macro in the environment"),
            None => panic!("Macro was referenced but has not defined"),
        },
        Some(LexicalExpression::List(expressions)) => (
            RuntimeExpression::List(build_many_macros(expressions, &environment)),
            rest,
        ),
        Some(LexicalExpression::FunctionCall(name, expressions)) => (
            RuntimeExpression::FunctionCall(
                name.to_string(),
                build_many_macros(expressions.into(), &environment),
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

fn build_many_macros(
    incoming_exprs: &Vector<LexicalExpression>,
    environment: &HashMap<String, RuntimeExpression>,
) -> Vector<RuntimeExpression> {
    let mut remaining_exprs = incoming_exprs.clone();
    let mut outgoing_exprs: Vector<RuntimeExpression> = vector![];

    while !remaining_exprs.is_empty() {
        let (built_expression, new_remaining_exprs) = build_macros(&remaining_exprs, &environment);
        outgoing_exprs.push_back(built_expression);
        remaining_exprs = new_remaining_exprs;
    }

    outgoing_exprs
}
