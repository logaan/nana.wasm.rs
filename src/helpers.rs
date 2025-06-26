use im::Vector;

use crate::expressions::RuntimeExpression;
use crate::expressions::RuntimeExpression::{Function, Macro};

#[macro_export]
macro_rules! s {
    ($s:expr) => {
        String::from($s)
    };
}

// Asserting against functions will cause a stack overflow because functions
// have a reference to themselves via their closed over environment.
pub fn strip_functions(expressions: Vector<RuntimeExpression>) -> Vector<RuntimeExpression> {
    expressions
        .into_iter()
        .filter(|e| match e {
            Function(..) => false,
            Macro(..) => false,
            _ => true,
        })
        .collect()
}
