use std::sync::Arc;

use super::nana::*;

use crate::{
    eval::read_code,
    expressions::LexicalExpression::{self, *},
};
use im::{vector, Vector};

fn expected() -> Vector<LexicalExpression> {
    vector![
        Comment,
        MacroName("Package".to_string()),
        Symbol("nana:examples@0.0.1".to_string()),
        MacroName("World".to_string()),
        Symbol("fizzbuzz".to_string()),
        List(vector![
            MacroName("Import".to_string()),
            Symbol("wasi:cli/stdout".to_string()),
            MacroName("Import".to_string()),
            Symbol("wasi:streams/output-stream".to_string()),
            MacroName("Export".to_string()),
            Symbol("print-fizzbuzz".to_string()),
            List(vector![Symbol("max".to_string()), Symbol("u8".to_string()),]),
            Hole,
        ]),
        MacroName("Func".to_string()),
        Symbol("num-to-txt".to_string()),
        List(vector![Symbol("num".to_string()), Symbol("u8".to_string()),]),
        Symbol("string".to_string()),
        MacroName("Match".to_string()),
        List(vector![
            TaggedTuple(
                Arc::new(Symbol("mod".to_string())),
                vector![Symbol("num".to_string()), Number(3)],
            ),
            TaggedTuple(
                Arc::new(Symbol("mod".to_string())),
                vector![Symbol("num".to_string()), Number(5)],
            ),
        ]),
        List(vector![
            List(vector![Number(0), Number(0)]),
            String("Fizzbuzz".to_string()),
            List(vector![Number(0), Hole]),
            String("Fizz".to_string()),
            List(vector![Hole, Number(0)]),
            String("Buzz".to_string()),
            List(vector![Hole, Hole]),
            Symbol("n".to_string()),
        ]),
        MacroName("Func".to_string()),
        Symbol("list-to-txt".to_string()),
        List(vector![
            Symbol("list".to_string()),
            Symbol("list<u8>".to_string()),
        ]),
        Symbol("list<string>".to_string()),
        TaggedTuple(
            Arc::new(Symbol("map".to_string())),
            vector![
                Symbol("num-to-text".to_string()),
                Symbol("list".to_string()),
            ],
        ),
        MacroName("Func".to_string()),
        Symbol("print-fizzbuzz".to_string()),
        List(vector![Symbol("max".to_string()), Symbol("u8".to_string()),]),
        Hole,
        MacroName("Let".to_string()),
        List(vector![
            Symbol("lines".to_string()),
            TaggedTuple(
                Arc::new(Symbol("list-to-text".to_string())),
                vector![TaggedTuple(
                    Arc::new(Symbol("range".to_string())),
                    vector![Number(1), Number(100)],
                )],
            ),
            Symbol("stdout".to_string()),
            TaggedTuple(Arc::new(Symbol("stdout/get-stdout".to_string())), vector![]),
        ]),
        MacroName("For".to_string()),
        List(vector![
            Symbol("line".to_string()),
            Symbol("lines".to_string()),
        ]),
        TaggedTuple(
            Arc::new(Symbol("stdout.write".to_string())),
            vector![Symbol("line".to_string())],
        ),
    ]
}

#[test]
fn parses_fizzbuzz() {
    let code = read_code("examples/fizzbuzz.nana");
    let result = program(&code);
    assert_eq!(Ok(("", expected())), result);
}
