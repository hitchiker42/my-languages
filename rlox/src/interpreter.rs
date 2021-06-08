use std::io::{self, prelude::*};
use std::fs;
use crate::{
    parser,
    types::*
};
pub fn run(source : &str) -> Result<()> {
    let tokens : Vec<Token> = parser::scan(source)?;
    //let ast : AST = parser::parse(tokens)
    for token in tokens {
        println!("{}", token);
    }
    Ok(())
}
pub fn eval_file(filename : &str) -> Result<()> {
    let src = fs::read_to_string(filename)?;
    run(&src)
}
pub fn repl() -> Result<()> {
    let mut input = String::new();
    loop {
        io::stdout().write(b"> ")?;
        io::stdin().read_line(&mut input)?;
        if input.len() == 0  { break; }
        let result = run(&input).unwrap_or_else(|error|
             match error {
                 Error::Parse(e) => println!("{}", e),
                 Error::Io(e) => panic!("{}", e), //Abort On IO error I guess
             }
        );
//        println!("{}", result);
    }
    Ok(())
}
