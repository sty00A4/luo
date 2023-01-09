#![allow(unused)]
mod scanning;
mod compiler;
use scanning::{tokens, lexer, nodes, parser};
use compiler::{types, check};

fn main() {
    println!("Hello, world!");
}
