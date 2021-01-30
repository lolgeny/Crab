#[derive(Clone, Debug)]
pub enum Token {
    Number(f64),
    Plus, Minus, Times, Divide, Greater, Less, Eq, Neq,
    Natural, NaturalTo, Range, Take, Length, Fold(Vec<Token>, bool), Filter(Vec<Token>), Map(Vec<Token>),
    Pop, Duplicate,
    Print
}
use Token::*;
use std::iter::Peekable;

const HEX: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

pub fn lex(it: &mut Peekable<impl Iterator<Item=char>>, block: bool, stdin: &mut impl Iterator<Item=f64>) -> Vec<Token> {
    let mut tokens = vec![];
    while let Some(next) = it.next() {
        tokens.push(match next {
            '+' => Plus,
            '-' => Minus,
            '*' => Times,
            '/' => Divide,
            '>' => Greater,
            '<' => Less,
            '=' => Eq,
            '!' => Neq,
            'N' => Natural,
            'n' => NaturalTo,
            'R' => Range,
            't' => Take,
            ',' => Length,
            's' => Fold(vec![Plus], true),
            'f' => Fold(lex(&mut it.next().into_iter().peekable(), false, stdin), true),
            'F' => Fold(lex(&mut it.next().into_iter().peekable(), false, stdin), false),
            '\\' => Fold(lex(it, true, stdin), false),
            '|' => Fold(lex(it, true, stdin), true),
            '#' => Filter(lex(it, true, stdin)),
            'm' => Map(lex(&mut it.next().into_iter().peekable(), false, stdin)),
            '%' => Map(lex(it, true, stdin)),
            ';' => Pop,
            '$' => Duplicate,
            'p' => Print,
            'r' => Number(stdin.next().unwrap()),
            '}' => if block {
                return tokens
            } else {
                panic!("Didn't expect '}' - not in a function block")
            },
            x if HEX.contains(&x) => {
                let mut num = String::from(x);
                while let Some(next) = it.peek() {
                    if !HEX.contains(next) {break}
                    num.push(it.next().unwrap());
                }
                Token::Number(i64::from_str_radix(&num, 16).unwrap() as f64)
            }
            x if x.is_whitespace() => continue,
            x => panic!(format!("Unknown operator {}", x))
        })
    }
    tokens
}