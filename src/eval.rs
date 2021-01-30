use crate::lex::Token;
use std::fmt::Debug;


pub (crate) trait NumberIter: Iterator<Item=f64> + Debug {
    fn box_clone(&self) -> Box<dyn NumberIter>;
}
impl<T: Iterator<Item=f64> + Debug + Clone + 'static> NumberIter for T {
    fn box_clone(&self) -> Box<dyn NumberIter> {
        box self.clone()
    }
}
impl Clone for Box<dyn NumberIter> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Debug)]
pub (crate) enum Value {
    Number(f64), Iter(Box<dyn NumberIter>)
}
impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Number(n) => Number(*n),
            Iter(it) => Iter(it.box_clone())
        }
    }
}
use Value::*;

type Stack = Vec<Value>;

pub (crate) fn eval(input: Vec<Token>, stack: &mut Stack) {
    let mut it = input.into_iter();
    while let Some(next) = it.next() {
        use Token::*;
        match next {
            Number(n) => number(n, stack),
            Plus => plus(stack),
            Minus => minus(stack),
            Times => times(stack),
            Modulus => modulus(stack),
            Divides => divides(stack),
            Floor => floor(stack),
            Ceil => ceil(stack),
            Greater => greater(stack),
            Less => less(stack),
            Eq => eq(stack),
            Neq => neq(stack),
            Divide => divide(stack),
            Natural => natural(stack),
            NaturalTo => natural_to(stack),
            Range => range(stack),
            Take => take(stack),
            Length => length(stack),
            Fold(f, first) => fold(f, first, stack),
            Filter(f) => filter(f, stack),
            Map(f) => map(f, stack),
            Print => print(stack),
            Pop => pop(stack),
            Duplicate => duplicate(stack),
        }
    }
}

fn pop_number(stack: &mut Stack) -> f64 {
    match stack.pop() {
        Some(Number(n)) => n,
        Some(_) => panic!("Expected number"),
        None => 0f64
    }
}
fn pop_iter(stack: &mut Stack) -> Box<dyn NumberIter> {
    match stack.pop() {
        Some(Iter(it)) => it,
        Some(_) => panic!("Expected iterator"),
        None => box (0..).map(|x|x as f64)
    }
}
fn pop_bool(stack: &mut Stack) -> bool {
    match stack.pop().unwrap_or(Number(0f64)) {
        Number(n) => n != 0f64,
        Iter(_) => true
    }
}

fn number(n: f64, stack: &mut Stack) {
    stack.push(Number(n));
}

fn plus(stack: &mut Stack) {
    let n = pop_number(stack) + pop_number(stack);
    stack.push(Number(n));
}
fn minus(stack: &mut Stack) {
    let n = pop_number(stack) - pop_number(stack);
    stack.push(Number(n));
}
fn times(stack: &mut Stack) {
    let n = pop_number(stack) * pop_number(stack);
    stack.push(Number(n));
}
fn divide(stack: &mut Stack) {
    let n = pop_number(stack) / pop_number(stack);
    stack.push(Number(n));
}
fn modulus(stack: &mut Stack) {
    let n = pop_number(stack) % pop_number(stack);
    stack.push(Number(n));
}
fn divides(stack: &mut Stack) {
    let n = pop_number(stack) % pop_number(stack);
    stack.push(Number((n == 0f64) as u8 as f64));
}
fn floor(stack: &mut Stack) {
    let n = pop_number(stack).floor();
    stack.push(Number(n));
}
fn ceil(stack: &mut Stack) {
    let n = pop_number(stack).ceil();
    stack.push(Number(n));
}
fn greater(stack: &mut Stack) {
    let cond = pop_number(stack) > pop_number(stack);
    stack.push(Number(cond as u8 as f64));
}
fn less(stack: &mut Stack) {
    let cond = pop_number(stack) < pop_number(stack);
    stack.push(Number(cond as u8 as f64));
}
fn eq(stack: &mut Stack) {
    let cond = pop_number(stack) == pop_number(stack);
    stack.push(Number(cond as u8 as f64));
}
fn neq(stack: &mut Stack) {
    let cond = pop_number(stack) != pop_number(stack);
    stack.push(Number(cond as u8 as f64));
}

fn natural(stack: &mut Stack) {
    stack.push(Iter(box (1..).map(|x|x as f64)));
}
fn natural_to(stack: &mut Stack) {
    let n = pop_number(stack) as i64;
    stack.push(Iter(box (1..=n).map(|x|x as f64)));
}
fn range(stack: &mut Stack) {
    let start = pop_number(stack) as i64;
    stack.push(Iter(box (start..).map(|x|x as f64)));
}
fn take(stack: &mut Stack) {
    let it;
    let n;
    if matches!(stack.last(), Some(Iter(_))) {
        it = pop_iter(stack);
        n = pop_number(stack);
    } else {
        n = pop_number(stack);
        it = pop_iter(stack);
    }
    stack.push(Iter(box it.take(n as usize)));
}
fn length(stack: &mut Stack) {
    match stack.pop().unwrap() {
        Number(n) => {
            stack.push(Iter(box (0..(n as i64)).map(|x|x as f64)));
        }
        Iter(it) => {
            stack.push(Number(it.count() as f64));
        }
    }
}
fn fold(f: Vec<Token>, first: bool, stack: &mut Stack) {
    let mut it = pop_iter(stack);
    if first {
        if let Some(next) = it.next() {
            stack.push(Number(next));
        }
    }
    for n in it {
        stack.push(Number(n));
        eval(f.clone(), stack);
    }
}
fn filter(f: Vec<Token>, stack: &mut Stack) {
    let it = pop_iter(stack);
    let filtered = it.filter(move |n| {
        let mut stack = vec![];
        stack.push(Number(*n));
        eval(f.clone(), &mut stack);
        pop_bool(&mut stack)
    });
    stack.push(Iter(box filtered));
}
fn map(f: Vec<Token>, stack: &mut Stack) {
    let it = pop_iter(stack);
    stack.push(Iter(box it.map(move |n| {
        let mut stack = vec![Number(n)];
        eval(f.clone(), &mut stack);
        pop_number(&mut stack)
    })));
}

fn pop(stack: &mut Stack) {stack.pop();}
fn duplicate(stack: &mut Stack) {
    stack.push(stack.last().expect("Nothing to duplicate").clone());
}

fn print(stack: &mut Stack) {
    match stack.pop().unwrap_or(Number(0f64)) {
        Number(n) => print!("{}\n", n),
        Iter(it) => {
            for n in it {
                print!("{} ", n)
            }
            println!()
        }
    };
}