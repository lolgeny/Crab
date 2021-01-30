#![feature(trait_alias, box_syntax, box_patterns)]
#![deny(rust_2018_idioms, rustdoc)]
#![warn(future_incompatible)]
#![allow(panic_fmt)]

mod lex;
mod eval;

use std::env::args;
use std::fs::read_to_string;
use std::io::{stdin, Read};

fn main() {
    if args().len() != 2 {
        panic!("Expected usage: 'crab <file.crab>'")
    }
    let program = read_to_string(args().nth(1).unwrap()).expect("Could not read file");
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Could not read input");
    let mut input = input.split(' ').map(str::parse).map(Result::unwrap);
    let tokens = lex::lex(&mut program.chars().peekable(), false, &mut input);
    let mut stack = vec![];
    eval::eval(tokens, &mut stack);
    println!("Stack trace:");
    println!("{:?}", stack);
}