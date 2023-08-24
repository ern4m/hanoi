#[derive(Debug)]
struct Block {
    size: u8,
} 

impl Block {
    fn smaller(&self, other: Block) -> bool {
        if other.size > self.size {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Stack {
    items: Vec<Block>
}

impl Stack {
    fn new() -> Stack {
        Stack {
            items: Vec::new()
        }
    }

    fn from(vec: Vec<Block>) -> Stack {
        Stack {
            items: Vec::from(vec)
        }
    }
}

#[derive(Debug)]
struct Hanoi {
    first_pile: Stack,
    second_pile: Stack,
    third_pile: Stack
}

impl Hanoi {
}


fn main() {
    let range = 1..9;

    let size = range.len();

    let mh = Hanoi {
        first_pile: Stack::from(range.rev().into_iter().map(|i| Block { size: i }).collect()),
        second_pile: Stack::new(),
        third_pile: Stack::new()
    };

    // implement iteration over the Hanoi Stacks
    // implement better Structs using Option to encase the values
}
