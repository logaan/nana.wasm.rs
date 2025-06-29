use std::sync::Arc;

use crate::expressions::Environment;
use crate::expressions::LexicalExpression;
use crate::expressions::RuntimeExpression;
use im::vector;
use im::Vector;

fn build_macro_args(
    params: Vector<String>,
    rest: Vector<LexicalExpression>,
    environment: &Environment,
    name: &String,
) -> (Option<RuntimeExpression>, Vector<LexicalExpression>) {
    let (final_args, new_rest) =
        (0..params.len()).fold((Vector::new(), rest), |(args, curr_rest), _| {
            let (arg, remainder) = build_macros(&curr_rest, &environment);
            let new_args = arg
                .and_then(|arg| Some(args.clone() + Vector::unit(arg)))
                .unwrap_or(args);
            (new_args, remainder)
        });
    (
        Some(RuntimeExpression::MacroCall(name.to_string(), final_args)),
        new_rest,
    )
}

fn build_non_macro(expression: LexicalExpression, environment: &Environment) -> RuntimeExpression {
    match expression {
        LexicalExpression::MacroName(_) => {
            panic!("Macro should've been handled by build_macros")
        }
        LexicalExpression::Comment => {
            panic!("Comment should've been handled by build_macros")
        }
        LexicalExpression::List(expressions) => {
            RuntimeExpression::List(build_many_macros(&expressions, &environment))
        }

        LexicalExpression::TaggedTuple(tag, expressions) => RuntimeExpression::TaggedTuple(
            Arc::new(build_non_macro((*tag).clone(), environment)),
            build_many_macros(&expressions, &environment),
        ),
        LexicalExpression::Keyword(name) => RuntimeExpression::Keyword(name.to_string()),
        LexicalExpression::Symbol(name) => RuntimeExpression::Symbol(name.to_string()),
        LexicalExpression::Number(value) => RuntimeExpression::Number(value),
        LexicalExpression::String(value) => RuntimeExpression::String(value.to_string()),
        LexicalExpression::Hole => RuntimeExpression::Hole,
    }
}

pub fn build_macros(
    expressions: &Vector<LexicalExpression>,
    environment: &Environment,
) -> (Option<RuntimeExpression>, Vector<LexicalExpression>) {
    if expressions.len() == 0 {
        panic!("We can't build macros from an empty expression list")
    }

    let rest = expressions.skip(1);

    match expressions.head() {
        None => panic!("We can't build macros from an empty expression list"),
        Some(LexicalExpression::Comment) => (None, rest),
        Some(LexicalExpression::MacroName(name)) => match environment.get(name) {
            Some(RuntimeExpression::Macro(params, _, _)) => {
                build_macro_args(params, rest, environment, name)
            }
            Some(RuntimeExpression::BuiltinMacro(params, _)) => {
                build_macro_args(params, rest, environment, name)
            }
            Some(_) => panic!("A macro name should only ever point to a macro in the environment"),
            None => panic!("Macro {} was referenced but has not defined", name),
        },
        Some(expression) => (Some(build_non_macro(expression.clone(), environment)), rest),
    }
}

fn build_many_macros(
    incoming_exprs: &Vector<LexicalExpression>,
    environment: &Environment,
) -> Vector<RuntimeExpression> {
    let mut remaining_exprs = incoming_exprs.clone();
    let mut outgoing_exprs: Vector<RuntimeExpression> = vector![];

    while !remaining_exprs.is_empty() {
        let (built_expression, new_remaining_exprs) = build_macros(&remaining_exprs, &environment);
        built_expression.map(|expr| outgoing_exprs.push_back(expr));
        remaining_exprs = new_remaining_exprs;
    }

    outgoing_exprs
}
