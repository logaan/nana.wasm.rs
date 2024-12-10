use im::{hashmap, vector, Vector};

use crate::expressions::Environment;
use crate::expressions::RuntimeExpression::{BuiltinMacro, Function, List, ValueName};
use crate::process::Process::Complete;
use crate::s;

pub fn standard_library() -> Environment {
    // TODO: Write constructors for every `RuntimeExpression`.
    hashmap! {
        // TODO: Write tests for Fn. You'll need to manually construct and
        // `Environment` with the  `Fn` in it. To do that you'll need to `eval`
        // the `Fn` and `run_until_completion`. Once that env is done you can
        // `execute` a normal function call in a program string.
        s!("Fn") => BuiltinMacro(
            vector![
              s!("params"),
              s!("body")
            ],
            |mut args| {
                if args.len() == 2 {
                    let params = args.pop_front().unwrap();
                    let body = args.pop_front().unwrap();

                    match params {
                        List(params) => {
                            let param_strings = params.iter().map(|p| match p {
                                ValueName(s) => s,
                                _ => panic!("Func params must be ValueNames")
                            }).cloned().collect::<Vector<String>>();
                            Complete(Function(param_strings, hashmap!{}, vector![body]))
                        },
                        _ => panic!("Fn takes a list of params and a single body expression")
                    }
                } else {
                    panic!("Fn takes exactly 2 arguments")
                }
            }
        ),
    }
}
