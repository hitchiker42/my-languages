extern crate derive_more;
extern crate strum_macros;
use std::{env, io, error};
use std::io::prelude::*;

mod interpreter;
mod parser;
mod types;
fn main() -> Result<(), Box<dyn error::Error>> {
    let args : Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage {} [script]", args[0]);
    } else if args.len() == 2 {
        interpreter::eval_file(&args[1])?;
    } else {
        interpreter::repl()?;
    }
    Ok(())
}
