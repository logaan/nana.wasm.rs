#![allow(dead_code)]

#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest as Command;
use parsers::nana::program;

mod parsers;

struct Component;

static FIZZBUZZ: &str = r#"
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

impl Command for Component {
    fn run() -> Result<(), ()> {
        let result = program(FIZZBUZZ);
        println!("Parser result: {:?}", result);
        println!("Hello world");
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
