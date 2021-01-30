#![feature(trait_alias, box_syntax, box_patterns)]
#![deny(rust_2018_idioms, rustdoc)]
#![warn(future_incompatible)]
#![allow(panic_fmt)]

mod lex;
mod eval;

use std::env::args;
use std::fs::read_to_string;
use std::io::{stdin, Read};
use crate::eval::Value;

fn main() {
    if args().len() != 2 {
        panic!("Expected usage: 'crab <file.crab>'")
    }
    let program = read_to_string(args().nth(1).unwrap()).expect("Could not read file");
    let tokens = lex::lex(&mut program.chars().peekable(), false);
    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Could not read input");
    let mut stack = input.split(' ').map(|s|Value::Number(s.parse().unwrap())).collect();
    eval::eval(tokens, &mut stack);
    println!("Stack trace:");
    println!("{:?}", stack);
}