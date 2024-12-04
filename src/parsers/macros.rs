use super::nana::Expression;
use std::collections::HashMap;

pub fn build_macros(_: Vec<Expression>, _: HashMap<String, Expression>) -> Vec<Expression> {
    // TODO: Do a depth first search of the syntax tree and every time you find a
    // MacroName use it to create a Macro, pulling in the next few expressions
    // as arguments
    vec![Expression::Hole]
}
