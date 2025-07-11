use std::sync::Arc;

use im::vector;

use crate::{
    expressions::RuntimeExpression::{self, Keyword, String as NString, TaggedTuple},
    process::Process::{self, Complete},
    s,
};

pub fn error(variety: &str) -> Process<RuntimeExpression> {
    Complete(TaggedTuple(
        Arc::new(Keyword(s!("error"))),
        vector![Keyword(s!(variety))],
    ))
}

pub fn error_with_message(variety: &str, message: &str) -> Process<RuntimeExpression> {
    Complete(TaggedTuple(
        Arc::new(Keyword(s!("error"))),
        vector![Keyword(s!(variety)), NString(s!(message))],
    ))
}

pub fn argument_error(message: &str) -> Process<RuntimeExpression> {
    error_with_message("argument", message)
}

pub fn not_found_error(message: &str) -> Process<RuntimeExpression> {
    error_with_message("not-found", message)
}
