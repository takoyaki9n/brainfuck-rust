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
    fn new() -> Machine {
        Machine {
            pc: 0,
            ptr: 0,
            mem: Vec::new(),
            code: Vec::new(),
            map: HashMap::new()
        }
    }

    fn run(&mut self, code: &[u8]) -> Option<String> {
        self.pc = 0;
        self.ptr = 0;
        self.mem.clear();
        self.code = Vec::from(code);
        self.map = match self.init_map() {
            Some(map) => { map },
            None => { return Some(String::from("Unclosed brackets.")); }
        };

        while self.pc < self.code.len() {
            if self.mem.len() <= self.ptr { self.mem.push(0); }

            match self.code[self.pc] {
                b'>' => { self.ptr += 1; },
                b'<' => { self.ptr -= 1; },
                b'+' => { self.mem[self.ptr] += 1; },
                b'-' => { self.mem[self.ptr] -= 1; },
                b'.' => { print!("{}", self.mem[self.ptr] as char); },
                b',' => { self.mem[self.ptr] = getchar(); },
                b'[' | b']' => {},
                c => { return Some(format!("Unexpected charanter {}.", c as char)); }
            }

            self.pc = match self.next_pc() {
                Some(pc) => { pc },
                None => { return Some(String::from("Invalid PC.")); }
            }
        }

        None
    }

    fn init_map(&self) -> Option<HashMap<usize, usize>> {
        let mut map = HashMap::new();
        let mut stk = Vec::new();
        for i in 0..self.code.len() {
            match self.code[i] {
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

    fn next_pc(&self) -> Option<usize> {
        match self.code[self.pc] {
            b'[' => {
                if self.mem[self.ptr] == 0 { self.map.get(&self.pc).cloned() } else { Some(self.pc + 1) }
            },
            b']' => {
                if self.mem[self.ptr] != 0 { self.map.get(&self.pc).cloned() } else { Some(self.pc + 1) }
            },
            _ => { Some(self.pc + 1) }
        }
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

        match machine.run(line.trim().as_bytes()) {
            Some(s) => { println!("{}", s); },
            None => {}
        }
    }
}
