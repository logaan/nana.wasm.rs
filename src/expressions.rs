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
    // TODO: Tagging things arbitrarily at the syntax level isn't that crazy.
    // It's basically just expressing the idea of function application. A tuple
    // tagged with a tuple tagged with a symbol is just calling a function
    // returned by another function:
    //
    //   getCallback("onClick")(clickEvent)
    //
    // Using it on maps is already pretty clear:
    //
    //   {:a 1 :b 2}(:a) # => 1
    //
    // And lists should probably just do nth:
    //
    //   [:a :b :c](1) # => :b
    //
    // Strings could probably do nth too:
    //
    //   "Hello"(1) # => "e"
    //
    // I'm not sure about numbers though...
    //
    //   100()
    //
    // What does (100 :a :b) do in Clojure? Or 100[:a] do in Ruby?
    // - iFn isn't defined for numbers in Clojure
    // - In Ruby it is Bit Reference. It Returns the nth bit in the binary
    // representation of int, where int[0] is the least significant bit.
    TaggedTuple(Arc<LexicalExpression>, Vector<LexicalExpression>),
    Keyword(String),
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
    TaggedTuple(Arc<RuntimeExpression>, Vector<RuntimeExpression>),
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
    Keyword(String),
    Definition(String, Arc<RuntimeExpression>),
}
