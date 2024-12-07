use super::nana::*;

use im::{vector, Vector};
use LexicalExpression::*;

pub static FIZZBUZZ: &str = r#"
Package nana:examples@0.0.1

World fizzbuzz [
   Import wasi:cli/stdout
   Import wasi:streams/output-stream

   Export print-fizzbuzz [max u8] _
]

Func num-to-txt [num u8] string
  Match [mod(num 3) mod(num 5)] [
    [0 0] "Fizzbuzz"
    [0 _] "Fizz"
    [_ 0] "Buzz"
    [_ _] n
  ]

Func list-to-txt [list list<u8>] list<string>
  map(num-to-text list)

Func print-fizzbuzz [max u8] _
  Let[lines  list-to-text(range(1 100))
      stdout stdout/get-stdout()]
    For[line lines]
      stdout.write(line)
 
"#;

fn expected() -> Vector<LexicalExpression> {
    vector![
        MacroName("Package".to_string()),
        ValueName("nana:examples@0.0.1".to_string()),
        MacroName("World".to_string()),
        ValueName("fizzbuzz".to_string()),
        List(vector![
            MacroName("Import".to_string()),
            ValueName("wasi:cli/stdout".to_string()),
            MacroName("Import".to_string()),
            ValueName("wasi:streams/output-stream".to_string()),
            MacroName("Export".to_string()),
            ValueName("print-fizzbuzz".to_string()),
            List(vector![
                ValueName("max".to_string()),
                ValueName("u8".to_string()),
            ]),
            Hole,
        ]),
        MacroName("Func".to_string()),
        ValueName("num-to-txt".to_string()),
        List(vector![
            ValueName("num".to_string()),
            ValueName("u8".to_string()),
        ]),
        ValueName("string".to_string()),
        MacroName("Match".to_string()),
        List(vector![
            FunctionCall(
                "mod".to_string(),
                vector![ValueName("num".to_string()), Number(3)],
            ),
            FunctionCall(
                "mod".to_string(),
                vector![ValueName("num".to_string()), Number(5)],
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
            ValueName("n".to_string()),
        ]),
        MacroName("Func".to_string()),
        ValueName("list-to-txt".to_string()),
        List(vector![
            ValueName("list".to_string()),
            ValueName("list<u8>".to_string()),
        ]),
        ValueName("list<string>".to_string()),
        FunctionCall(
            "map".to_string(),
            vector![
                ValueName("num-to-text".to_string()),
                ValueName("list".to_string()),
            ],
        ),
        MacroName("Func".to_string()),
        ValueName("print-fizzbuzz".to_string()),
        List(vector![
            ValueName("max".to_string()),
            ValueName("u8".to_string()),
        ]),
        Hole,
        MacroName("Let".to_string()),
        List(vector![
            ValueName("lines".to_string()),
            FunctionCall(
                "list-to-text".to_string(),
                vector![FunctionCall(
                    "range".to_string(),
                    vector![Number(1), Number(100)],
                )],
            ),
            ValueName("stdout".to_string()),
            FunctionCall("stdout/get-stdout".to_string(), vector![]),
        ]),
        MacroName("For".to_string()),
        List(vector![
            ValueName("line".to_string()),
            ValueName("lines".to_string()),
        ]),
        FunctionCall(
            "stdout.write".to_string(),
            vector![ValueName("line".to_string())],
        ),
    ]
}

#[test]
fn parses_fizzbuzz() {
    let result = program(FIZZBUZZ);
    assert_eq!(Ok(("", expected())), result);
}
