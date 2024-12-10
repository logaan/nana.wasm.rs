use crate::process::Process;
use im::Vector;

#[derive(PartialEq, Debug, Clone)]
pub enum LexicalExpression {
    MacroName(String),
    ValueName(String),
    // TODO: FunctionCalls should take a LexicalExpression as their first
    // argument so that we can do things like get(someMap,
    // "someFuncName")(someArg).
    FunctionCall(String, Vector<LexicalExpression>),
    List(Vector<LexicalExpression>),
    Number(u8),
    String(String),
    Hole,
}

#[derive(PartialEq, Debug, Clone)]
pub enum RuntimeExpression {
    Macro(String, Vector<String>, Vector<RuntimeExpression>),
    BuiltinFunction(fn(Vector<RuntimeExpression>) -> Process<RuntimeExpression>),
    ValueName(String),
    FunctionCall(String, Vector<RuntimeExpression>),
    MacroCall(String, Vector<RuntimeExpression>),
    List(Vector<RuntimeExpression>),
    Number(u8),
    String(String),
    Hole,
}
