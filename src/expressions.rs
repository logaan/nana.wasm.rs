use crate::process::Process;
use im::{HashMap, Vector};

#[derive(PartialEq, Debug, Clone)]
pub enum LexicalExpression {
    FunctionCall(String, Vector<LexicalExpression>),
    Hole,
    List(Vector<LexicalExpression>),
    MacroName(String),
    Number(u8),
    String(String),
    ValueName(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum RuntimeExpression {
    BuiltinFunction(fn(Vector<RuntimeExpression>) -> Process<RuntimeExpression>),
    Function(
        Vector<String>,
        HashMap<String, RuntimeExpression>,
        Vector<RuntimeExpression>,
    ),
    FunctionCall(String, Vector<RuntimeExpression>),
    Hole,
    List(Vector<RuntimeExpression>),
    BuiltinMacro(
        Vector<String>,
        fn(Vector<RuntimeExpression>) -> Process<RuntimeExpression>,
    ),
    Macro(Vector<String>, Vector<RuntimeExpression>),
    MacroCall(String, Vector<RuntimeExpression>),
    Number(u8),
    String(String),
    ValueName(String),
}
