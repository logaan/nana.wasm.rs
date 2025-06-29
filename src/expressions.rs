use std::sync::Arc;

pub use crate::environment::Environment;
use crate::process::Process;
use im::Vector;

pub fn is_comment(expression: &LexicalExpression) -> bool {
    match expression {
        LexicalExpression::Comment => true,
        _ => false,
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum LexicalExpression {
    TaggedTuple(Arc<LexicalExpression>, Vector<LexicalExpression>),
    Keyword(String),
    Hole,
    List(Vector<LexicalExpression>),
    MacroName(String),
    Number(i64),
    String(String),
    Symbol(String),
    Comment,
}

#[derive(PartialEq, Debug, Clone)]
pub enum RuntimeExpression {
    BuiltinFunction(fn(Vector<RuntimeExpression>) -> Process<RuntimeExpression>),
    Function(Vector<String>, Environment, Vector<RuntimeExpression>),
    TaggedTuple(Arc<RuntimeExpression>, Vector<RuntimeExpression>),
    Hole,
    List(Vector<RuntimeExpression>),
    BuiltinMacro(
        Vector<String>,
        fn(Vector<RuntimeExpression>, Environment) -> Process<RuntimeExpression>,
    ),
    Macro(Vector<String>, Environment, Vector<RuntimeExpression>),
    MacroCall(String, Vector<RuntimeExpression>),
    Number(i64),
    String(String),
    Symbol(String),
    Keyword(String),
    Definition(String, Arc<RuntimeExpression>),
}
