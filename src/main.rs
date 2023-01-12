#![allow(unused)]
mod error;
mod scanning;
mod compiler;
use std::{env, fs};
use error::Error;
use scanning::{tokens, lexer, nodes, parser};
use compiler::{types, check};

fn run() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut args = args.iter();
    args.next();
    let Some(input_path) = args.next() else {
        println!("USAGE: luo [input path] ([output path])");
        return Ok(())
    };
    let Ok(text) = fs::read_to_string(input_path) else {
        return Err(Error::InputFile(input_path.clone()))
    };
    let output_path = args.next();
    // lexing
    let tokens = lexer::lex(input_path, text)?;
    println!("{tokens:?}");
    // parsing
    let ast = parser::parse(input_path, tokens)?;
    println!("{}", ast.format(0, false));
    // type checking
    // compilation
    Ok(())
}

fn main() {
    let res = run();
    if let Some(err) = res.err() { eprint!("{err}") }
}
