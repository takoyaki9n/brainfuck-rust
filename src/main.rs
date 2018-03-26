use std::io::{self, Write, Read};
use std::collections::HashMap;

struct Machine {
    pub pc: usize,
    pub ptr: usize,
    pub mem: Vec<u8>,
    pub code: Vec<u8>,
    pub map: HashMap<usize, usize>
}

impl Machine {
    fn run(code: &[u8]) -> Option<String> {
        let map = match Machine::init_map(code) {
            Some(map) => { map },
            None => { return Some(String::from("Unclosed brackets.")); }
        };
        let mut machine = Machine {
            pc: 0,
            ptr: 0,
            mem: vec![0],
            code: Vec::from(code),
            map: map
        };

        while machine.pc < machine.code.len() {
            match machine.step() {
                Some(s) => { return Some(s); }
                None => {}
            }

            machine.pc = machine.next();
        }

        None
    }

    fn init_map(code: &[u8]) -> Option<HashMap<usize, usize>> {
        let mut map = HashMap::new();
        let mut stk = Vec::new();
        for i in 0..code.len() {
            match code[i] {
                b'[' => { stk.push(i); },
                b']' => {
                    match stk.pop() {
                        Some(j) => {
                            map.insert(i, j + 1);
                            map.insert(j, i + 1);
                        },
                        None => { return None; }
                    }
                },
                _ => {}
            }
        }

        if stk.is_empty() { Some(map) } else { None }
    }

    fn step(&mut self) -> Option<String> {
        match self.code[self.pc] {
            b'>' => {
                self.ptr += 1;
                if self.mem.len() <= self.ptr { self.mem.push(0); }
            },
            b'<' => { self.ptr -= 1; },
            b'+' => { self.mem[self.ptr] += 1; },
            b'-' => { self.mem[self.ptr] -= 1; },
            b'.' => { print!("{}", self.mem[self.ptr] as char); },
            b',' => { self.mem[self.ptr] = getchar(); },
            b'[' | b']' => {},
            c => { return Some(format!("Unexpected charanter '{}'.", c as char)); }
        }
        None
    }

    fn next(&self) -> usize {
        match self.code[self.pc] {
            b'[' => { if self.mem[self.ptr] == 0 { self.map[&self.pc] } else { self.pc + 1 } },
            b']' => { if self.mem[self.ptr] != 0 { self.map[&self.pc] } else { self.pc + 1 } },
            _ => { self.pc + 1 }
        }
    }
}

fn getchar() -> u8 {
    let mut u8b: [u8; 1] = [0];
    io::stdin().read(&mut u8b).unwrap();
    u8b[0]
}

fn main() {
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

        match Machine::run(line.trim().as_bytes()) {
            Some(s) => { println!("Error: {}", s); },
            None => {}
        }
    }
}
