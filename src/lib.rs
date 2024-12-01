#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest as Command;

struct Component;

impl Command for Component {
    /// Say hello!
    fn run() -> Result<(), ()> {
        println!("Hello world");
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
