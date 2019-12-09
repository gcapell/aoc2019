use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

pub struct MachineState {
    mem: Vec<i32>,
    instructions: HashMap<i32, Op>,
    pc: usize,
}

pub fn new(prog: &[i32]) -> MachineState {
    let mut i = HashMap::new();
    init_instructions(&mut i);

    MachineState {
        mem: prog.to_vec(),
        instructions: i,
        pc: 0,
    }
}

#[derive(Debug)]
enum ParamType {
    S,
    D,
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

enum NextPC {
    Relative,
    Absolute(i32),
    Exit,
}

use self::ParamType::*;

type OpFn = fn(m: &mut MachineState, p: Vec<i32>, i: &mut VecDeque<i32>, o: &mut VecDeque<i32>) -> NextPC;

struct Op {
    name: String,
    params: Vec<ParamType>,
    run: OpFn,
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Op {{ {} {:?} }}", self.name, self.params)
    }
}

impl MachineState {
    pub fn set_prog(&mut self, prog: &[i32]) {
        self.pc = 0;
        self.mem = prog.to_vec();
    }

    pub fn run(&mut self,  i:&mut VecDeque<i32>,  o: &mut VecDeque<i32>) {
        loop {
            let (func, params, size) = self.decode();
            match func(self, params, i, o) {
                NextPC::Relative => self.pc += size,
                NextPC::Absolute(addr) => self.pc = addr as usize,
                NextPC::Exit => return,
            }
        }
    }

    fn decode(&self) -> (OpFn, Vec<i32>, usize) {
        let (opcode, modes) = unpack(self.mem[self.pc]);
        let i = &self.instructions[&opcode];
        //println!("mem[{}] = {} => {:?}{:?}, modes:{:?}", pc, self.mem[pc], i.name, i.params, modes);
        let mut params = Vec::new();
        for (n, p) in i.params.iter().enumerate() {
            let v = self.mem[self.pc + 1 + n];
            //println!("mem[{}]={}", pc+1+n, v);
            params.push(match (&modes[n], p) {
                (Mode::Position, S) => self.mem[v as usize],
                (Mode::Position, D) => v,
                (Mode::Immediate, S) => v,
                (Mode::Immediate, D) => {
                    panic!("immediate dst {} @ {}, {:?}", self.mem[self.pc], self.pc, i)
                }
            });
        }
        //println!("pc:{}, {},{:?}", self.pc, i.name, params);
        return (i.run, params, i.params.len() + 1);
    }
}

fn unpack(mut i: i32) -> (i32, Vec<Mode>) {
    let opcode = ipop(&mut i, 100);
    let mut modes = Vec::new();
    for _ in 0..3 {
        let n = ipop(&mut i, 10);
        modes.push(match n {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("bad mode {}", n),
        });
    }
    (opcode, modes)
}
fn ipop(i: &mut i32, n: i32) -> i32 {
    let reply = (*i) % n;
    (*i) /= n;
    reply
}

fn bool_to_int(a: bool) -> i32 {
    if a {
        1
    } else {
        0
    }
}

fn init_instructions(i: &mut HashMap<i32, Op>) {
    i.insert(
        1,
        Op {
            name: "Add".to_string(),
            params: vec![S, S, D],
            run: |m, p, _i, _o| {
                if let [s1, s2, d] = p[..] {
                    m.mem[d as usize] = s1 + s2;
                    NextPC::Relative
                } else {
                    panic!("add params")
                }
            },
        },
    );
    i.insert(
        2,
        Op {
            name: "Mul".to_string(),
            params: vec![S, S, D],
            run: |m, p, _i, _o| {
                if let [s1, s2, d] = p[..] {
                    m.mem[d as usize] = s1 * s2;
                    NextPC::Relative
                } else {
                    panic!()
                }
            },
        },
    );
    i.insert(
        3,
        Op {
            name: "Input".to_string(),
            params: vec![D],
            run: |m, p, i, _o| {
                if let [d] = p[..] {
                    let val = i.pop_front().unwrap();
                    m.mem[d as usize] = val;
                    //println!("mem[{}] = {}", d, val);
                    NextPC::Relative
                } else {
                    panic!()
                }
            },
        },
    );
    i.insert(
        4,
        Op {
            name: "Output".to_string(),
            params: vec![S],
            run: |_m, p, _i, o| {
                if let [s] = p[..] {
                    o.push_back(s);
                    NextPC::Relative
                } else {
                    panic!()
                }
            },
        },
    );
    i.insert(
        5,
        Op {
            name: "JumpIfTrue".to_string(),
            params: vec![S, S],
            run: |_m, p, _i, _o| {
                if let [s, d] = p[..] {
                    if s != 0 {
                        NextPC::Absolute(d)
                    } else {
                        NextPC::Relative
                    }
                } else {
                    panic!()
                }
            },
        },
    );
    i.insert(
        6,
        Op {
            name: "JumpIfFalse".to_string(),
            params: vec![S, S],
            run: |_m, p, _i, _o| {
                if let [s, d] = p[..] {
                    if s == 0 {
                        NextPC::Absolute(d)
                    } else {
                        NextPC::Relative
                    }
                } else {
                    panic!()
                }
            },
        },
    );
    i.insert(
        7,
        Op {
            name: "LessThan".to_string(),
            params: vec![S, S, D],
            run: |m, p, _i, _o| {
                if let [a, b, d] = p[..] {
                    m.mem[d as usize] = bool_to_int(a < b);
                    NextPC::Relative
                } else {
                    panic!()
                }
            },
        },
    );
    i.insert(
        8,
        Op {
            name: "Equals".to_string(),
            params: vec![S, S, D],
            run: |m, p, _i, _o| {
                if let [a, b, d] = p[..] {
                    m.mem[d as usize] = bool_to_int(a == b);
                    NextPC::Relative
                } else {
                    panic!()
                }
            },
        },
    );
    i.insert(
        99,
        Op {
            name: "Exit".to_string(),
            params: Vec::new(),
            run: |_m, _p, _i, _o| NextPC::Exit,
        },
    );
}
