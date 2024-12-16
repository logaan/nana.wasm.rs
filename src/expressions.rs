use crate::process::Process;
use im::{HashMap, Vector};

pub type Environment = HashMap<String, RuntimeExpression>;

#[derive(PartialEq, Debug, Clone)]
pub enum LexicalExpression {
    TaggedTuple(String, Vector<LexicalExpression>),
    Hole,
    List(Vector<LexicalExpression>),
    MacroName(String),
    Number(u8),
    String(String),
    Symbol(String),
    Comment,
}

#[derive(PartialEq, Debug, Clone)]
pub enum RuntimeExpression {
    BuiltinFunction(fn(Vector<RuntimeExpression>) -> Process<RuntimeExpression>),
    Function(Vector<String>, Environment, Vector<RuntimeExpression>),
    TaggedTuple(String, Vector<RuntimeExpression>),
    Hole,
    List(Vector<RuntimeExpression>),
    BuiltinMacro(
        Vector<String>,
        fn(Vector<RuntimeExpression>, Environment) -> Process<RuntimeExpression>,
    ),
    Macro(Vector<String>, Environment, Vector<RuntimeExpression>),
    MacroCall(String, Vector<RuntimeExpression>),
    Number(u8),
    String(String),
    Symbol(String),
}
