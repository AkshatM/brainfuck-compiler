use std::{
    fs,
    io::{stdin, Read},
    path::Path,
};

#[derive(Debug)]
enum Instruction {
    MoveRightOne,
    MoveLeftOne,
    Increment,
    Decrement,
    Output,
    Input,
    JumpForward(usize),
    JumpBack(usize),
    Noop,
}

impl From<&u8> for Instruction {
    fn from(byte: &u8) -> Self {
        match *byte as char {
            '>' => Self::MoveRightOne,
            '<' => Self::MoveLeftOne,
            '+' => Self::Increment,
            '-' => Self::Decrement,
            '.' => Self::Output,
            ',' => Self::Input,
            '[' => Self::JumpForward(0),
            ']' => Self::JumpBack(0),
            _ => Self::Noop,
        }
    }
}

fn parse(stream: Vec<u8>) -> Vec<Instruction> {
    let mut tree = stream.iter().map(Instruction::from).collect::<Vec<_>>();
    let mut stack: Vec<usize> = vec![];
    let mut brace_pairs: Vec<(usize, usize)> = vec![];

    for (index, item) in tree.iter().enumerate() {
        match item {
            Instruction::JumpForward(_) => stack.push(index),
            Instruction::JumpBack(_) => {
                let previous_index = stack
                    .pop()
                    .unwrap_or_else(|| panic!("Encountered ] before [ at {index}"));
                brace_pairs.push((previous_index, index))
            }
            _ => {}
        }
    }

    for (left, right) in brace_pairs.into_iter() {
        if let Some(l) = tree.get_mut(left) {
            *l = Instruction::JumpForward(right);
        }

        if let Some(r) = tree.get_mut(right) {
            *r = Instruction::JumpBack(left);
        }
    }

    tree
}

fn execute<const N: usize>(ast: Vec<Instruction>, mut stack: [u8; N]) {
    let (mut rip, mut rsp) = (0_usize, 0_usize); // a little x86 assembly reference

    while rip < ast.len() {
        match ast[rip] {
            Instruction::MoveRightOne => {
                rsp = (rsp + 1) % N;
            }
            Instruction::MoveLeftOne => {
                rsp = match rsp {
                    0 => N - 1,
                    _ => rsp - 1,
                };
            }
            Instruction::Increment => stack[rsp] += 1,
            Instruction::Decrement => stack[rsp] -= 1,
            Instruction::Input => {
                let mut buf = [0; 1];
                stdin()
                    .read_exact(&mut buf)
                    .expect("Nothing read from stdin");
                stack[rsp] = buf[0];
            }
            Instruction::Output => print!("{}", stack[rsp] as char),
            Instruction::JumpForward(next) => {
                if stack[rsp] == 0_u8 {
                    rip = next;
                }
            }
            Instruction::JumpBack(previous) => {
                if stack[rsp] != 0_u8 {
                    rip = previous
                }
            }
            _ => {}
        }

        rip += 1;
    }
}

fn main() {
    let memory: [u8; 32768] = [0; 32768];
    let stream: Vec<u8> = fs::read(Path::new("./src/examples/hello-world.bf")).unwrap();
    let ast = parse(stream);
    execute(ast, memory)
}
