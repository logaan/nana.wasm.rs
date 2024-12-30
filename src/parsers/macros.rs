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
        Some(LexicalExpression::List(expressions)) => (
            Some(RuntimeExpression::List(build_many_macros(
                expressions,
                &environment,
            ))),
            rest,
        ),
        Some(LexicalExpression::TaggedTuple(name, expressions)) => (
            Some(RuntimeExpression::TaggedTuple(
                name.to_string(),
                build_many_macros(expressions.into(), &environment),
            )),
            rest,
        ),
        Some(LexicalExpression::Symbol(name)) => {
            (Some(RuntimeExpression::Symbol(name.to_string())), rest)
        }
        Some(LexicalExpression::Number(value)) => (Some(RuntimeExpression::Number(*value)), rest),
        Some(LexicalExpression::String(value)) => {
            (Some(RuntimeExpression::String(value.to_string())), rest)
        }
        Some(LexicalExpression::Hole) => (Some(RuntimeExpression::Hole), rest),
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
