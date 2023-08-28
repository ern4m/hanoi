use std::{error::Error, any::type_name};

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[derive(Debug, Clone)]
struct Block (u8);

impl Block {
    fn smaller(&self, other: Option<&Block>) -> bool {
        match other {
            Some(block) => if block.0 > self.0 {true} else {false},
            None => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Stack (Vec<Block>, u8);

impl Stack {
    fn new(size: u8) -> Stack {
        Stack((1..=size).rev().into_iter().map(|x| Block(x)).collect::<Vec<_>>(), size)
    }

    fn add(&mut self, block: Block) {
        self.0.push(block);
    }

    fn move_block(&mut self, other: &mut Stack) {
        other.0.push(self.0.pop().unwrap());
        self.1 -= 1;
        other.1 += 1;
    }

    fn new_empty(size: u8) -> Stack{
        Stack(Vec::with_capacity(size as usize), 0)
    }

}

#[derive(Debug, Clone)]
struct Hanoi<'a> {
    a: &<'a> mut Stack,
    b: Stack,
    c: Stack
}

impl Hanoi {
    fn new(size: u8) -> Self {
        Hanoi {
            a: Stack::new(size),
            b: Stack::new_empty(size),
            c: Stack::new_empty(size),
        }
    }
}

impl Iterator for Stack {
    type Item = Block;

    fn next(&mut self) -> Option<Block> {
       match self.0.pop() {
           Some(block) => Some(block),
           None => None,
       } 
    }
}

fn solve(hanoi: Hanoi) {
    let mut h = hanoi.a.1; 
    for block in hanoi.a.clone() {
        if hanoi.b.0.is_empty() {
            let _ = hanoi.a.move_block(&mut hanoi.b);
        } else if block.smaller(hanoi.b.0.last()) {
            let _ = hanoi.a.move_block(&mut hanoi.c);
        }
    }
}

fn main() {
    let size = 9;

    let mut mh = Hanoi::new(size);

    // println!("{:?}", mh.a.0.last().unwrap().smaller(mh.c.0.last().unwrap().clone()));
    println!("{:?}", mh.c.last());
    // println!("{mh:?}");
    // implement iteration over the Hanoi Stacks
    // implement better Structs using Option to encase the values
}
