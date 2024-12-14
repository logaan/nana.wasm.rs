use im::{hashmap, vector, Vector};

use crate::expressions::Environment;
use crate::expressions::RuntimeExpression::{BuiltinMacro, Function, List, Symbol};
use crate::process::Process::Complete;
use crate::s;

pub fn standard_library() -> Environment {
    // TODO: Write constructors for every `RuntimeExpression`.
    hashmap! {
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
                                Symbol(s) => s,
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
        s!("Match") => BuiltinMacro(
            vector![
              s!("value"),
              s!("cases")
            ],
            |mut args| {
                if args.len() == 2 {
                    let value = args.pop_front().unwrap();
                    let cases = args.pop_front().unwrap();

                    match cases {
                        List(cases) => {
                            if cases.len() % 2 != 0 {
                                panic!("Cases must be a list with an even number of elements")
                            }

                            let mut iter = cases.iter();
                            while let (Some(pattern), Some(body)) = (iter.next(), iter.next()) {
                                if value == *pattern {
                                    return Complete(body.clone())
                                }
                            }
                            panic!("No match found")
                        },
                        _ => panic!("Match takes a value and a list of cases")
                    }
                } else {
                    panic!("Match takes exactly 2 arguments")
                }
            }
        ),
    }
}
