use std::io::{self, Write, Read};

struct Machine {
    pub pc: usize,
    pub ptr: usize,
    pub mem: Vec<u8>
}

impl Machine {
    fn new() -> Machine {
        Machine {
            pc: 0,
            ptr: 0,
            mem: Vec::new()
        }
    }

    fn run(&mut self, code: &[u8]) {
        self.pc = 0;
        self.ptr = 0;
        self.mem.clear();

        while self.pc < code.len() {
            if self.mem.len() <= self.ptr { self.mem.push(0); }

            match code[self.pc] {
                b'>' => { self.ptr += 1 },
                b'<' => { self.ptr -= 1 },
                b'+' => { self.mem[self.ptr] += 1 },
                b'-' => { self.mem[self.ptr] -= 1 },
                b'.' => { print!("{}", self.mem[self.ptr] as char) },
                b',' => { self.mem[self.ptr] = getchar() },
                b'[' | b']' => {},
                c => {
                    println!("Unexpected character {}", c as char);
                    break;
                }
            }

            self.pc = self.next_pc(code);
        }
    }

    fn next_pc(&self, code: &[u8]) -> usize{
        match code[self.pc] {
            b'[' => {
                if self.mem[self.ptr] == 0 { self.jump(code, b'[', b']',  1) } else { self.pc + 1 }
            },
            b']' => {
                if self.mem[self.ptr] != 0 { self.jump(code, b']', b'[', -1) } else { self.pc + 1 }
            },
            _ => {self.pc + 1}
        }
    }

    fn jump(&self, code: &[u8], open: u8, close: u8, d: i32) -> usize {
        let mut pc = self.pc;
        let mut level = 0;
        while pc < code.len() {
            if code[pc] == open {
                level += 1;
            } else if code[pc] == close {
                level -= 1;
            }

            if level == 0 { return pc + 1; }

            pc = (pc as i32 + d) as usize;
        }

        panic!("unlosed brackets.");
    }
}

fn getchar() -> u8 {
    let mut u8b: [u8; 1] = [0];
    io::stdin().read(&mut u8b).unwrap();
    u8b[0]
}

fn main() {
    let mut machine = Machine::new();

    loop {
        print!("bf> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    println!("\nBye.");
                    break;
                }
            },
            Err(e) => println!("{}", e)
        }

        machine.run(line.trim().as_bytes());
    }
}
