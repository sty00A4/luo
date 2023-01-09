#![allow(unused)]
mod scanning;
mod compiler;
use std::env;
use scanning::{tokens, lexer, nodes, parser};
use compiler::{types, check};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args = args.iter();
    args.next();
    let Some(input_path) = args.next() else {
        println!("USAGE: luo [input path] ([output path])");
        return
    };
    let output_path = args.next();
    // lexing
    // parsing
    // type checking
    // compilation
}
