use std::sync::Arc;

pub use crate::environment::Environment;
use crate::{process::Process, s};
use im::Vector;
use RuntimeExpression::{
    BuiltinFunction, BuiltinMacro, Definition, Function, Hole, Keyword, List, Macro, MacroCall,
    Number, String as NString, Symbol, TaggedTuple,
};

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
    Number(u128),
    String(String),
    Symbol(String),
    Comment,
}

#[derive(PartialEq, Debug, Clone)]
pub enum RuntimeExpression {
    // Maybe builtin functions should have a name for more useful printing
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
    Number(u128),
    String(String),
    Symbol(String),
    Keyword(String),
    Definition(String, Arc<RuntimeExpression>),
}

pub fn print(expression: RuntimeExpression) -> String {
    match expression {
        BuiltinFunction(..) => s!("BuiltinFunction(..)"),
        Function(args, _env, _body) => format!("Function([{}] _)", print_strings(args, " ")),
        BuiltinMacro(args, _body) => format!("BuiltinMacro([{}] _)", print_strings(args, " ")),
        Definition(name, value) => format!("Definition({} {})", name, print((*value).clone())),
        Hole => s!("_"),
        Keyword(name) => format!(":{}", name),
        List(values) => format!("[{}]", print_many(values.clone(), " ")),
        Macro(args, _env, _body) => format!("Macro([{}] _)", print_strings(args, " ")),
        MacroCall(name, args) => format!("{}({})", name, print_many(args, " ")),
        Number(value) => format!("{}", value),
        NString(value) => format!("\"{}\"", value),
        Symbol(name) => name,
        TaggedTuple(tag, values) => {
            format!("{}({})", print((*tag).clone()), print_many(values, " "))
        }
    }
}

pub fn print_many(expressions: Vector<RuntimeExpression>, seperator: &str) -> String {
    expressions
        .into_iter()
        .map(|item| print(item))
        .collect::<Vec<_>>()
        .join(seperator)
}

fn print_strings(strings: Vector<String>, seperator: &str) -> String {
    strings.into_iter().collect::<Vec<_>>().join(seperator)
}
