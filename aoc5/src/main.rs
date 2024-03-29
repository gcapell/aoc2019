use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;


struct MachineState {
    mem: Vec<i32>,
    input: VecDeque<i32>,
    output: Vec<i32>,
    instructions: HashMap<i32, Op>,
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

type OpFn = fn(m: &mut MachineState, p: Vec<i32>) -> NextPC;


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
    fn run(&mut self) {
        let mut pc = 0;
        loop {
            let (func, params, size) = self.decode(pc);
            match func(self, params) {
                NextPC::Relative => pc += size,
                NextPC::Absolute(addr) => pc = addr as usize,
                NextPC::Exit => {println!("exiting"); return},
            }
        }
    }

    fn decode(&self, pc: usize) -> (OpFn, Vec<i32>, usize) {
        let (opcode, modes) = unpack(self.mem[pc]);
        let i = &self.instructions[&opcode];
		//println!("mem[{}] = {} => {:?}{:?}, modes:{:?}", pc, self.mem[pc], i.name, i.params, modes);
        let mut params = Vec::new();
        for (n, p) in i.params.iter().enumerate() {
            let v = self.mem[pc + 1 + n];
			//println!("mem[{}]={}", pc+1+n, v);
			params.push(
				match (&modes[n],p) {
					(Mode::Position,S)=>self.mem[v as usize],
					(Mode::Position,D)=>v,
					(Mode::Immediate,S)=>v,
					(Mode::Immediate,D)=> panic!("immediate dst {} @ {}, {:?}", self.mem[pc], pc, i),
				}
			);
        }
        //println!("{},{:?}", i.name, params);
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


fn main() {
    let mut state = MachineState {
        mem: PROG5.to_vec(),
        input: VecDeque::new(),
        output: Vec::new(),
        instructions: HashMap::new(),
    };

    init_instructions(&mut state.instructions);
    state.input.push_back(5);
    state.run();
    println!("{:?}", state.output);
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
            run: |m, p| {
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
            run: |m, p| {
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
            run: |m, p| {
                if let [d] = p[..] {
                    m.mem[d as usize] = m.input.pop_front().unwrap();
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
            run: |m, p| {
                if let [s] = p[..] {
                    m.output.push(s);
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
            run: |_m, p| {
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
            run: |_m, p| {
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
            run: |m, p| {
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
            run: |m, p| {
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
            run: |_m, _p| NextPC::Exit,
        },
    );
}

static PROG5: &[i32] = &[
    3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 72, 36, 225, 1101, 87, 26, 225, 2, 144,
    13, 224, 101, -1872, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223, 224, 223,
    1102, 66, 61, 225, 1102, 25, 49, 224, 101, -1225, 224, 224, 4, 224, 1002, 223, 8, 223, 1001,
    224, 5, 224, 1, 223, 224, 223, 1101, 35, 77, 224, 101, -112, 224, 224, 4, 224, 102, 8, 223,
    223, 1001, 224, 2, 224, 1, 223, 224, 223, 1002, 195, 30, 224, 1001, 224, -2550, 224, 4, 224,
    1002, 223, 8, 223, 1001, 224, 1, 224, 1, 224, 223, 223, 1102, 30, 44, 225, 1102, 24, 21, 225,
    1, 170, 117, 224, 101, -46, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 224, 223,
    223, 1102, 63, 26, 225, 102, 74, 114, 224, 1001, 224, -3256, 224, 4, 224, 102, 8, 223, 223,
    1001, 224, 3, 224, 1, 224, 223, 223, 1101, 58, 22, 225, 101, 13, 17, 224, 101, -100, 224, 224,
    4, 224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 224, 223, 223, 1101, 85, 18, 225, 1001, 44, 7,
    224, 101, -68, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 5, 224, 1, 223, 224, 223, 4, 223,
    99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1,
    99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1,
    99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225,
    225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225,
    225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 7, 677, 226, 224, 102, 2, 223, 223, 1005,
    224, 329, 101, 1, 223, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 344, 1001, 223, 1,
    223, 1107, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 359, 1001, 223, 1, 223, 1107, 226, 677,
    224, 102, 2, 223, 223, 1005, 224, 374, 101, 1, 223, 223, 7, 226, 677, 224, 102, 2, 223, 223,
    1005, 224, 389, 101, 1, 223, 223, 8, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 404, 101, 1,
    223, 223, 1008, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 419, 1001, 223, 1, 223, 107, 677,
    677, 224, 102, 2, 223, 223, 1005, 224, 434, 101, 1, 223, 223, 1108, 677, 226, 224, 1002, 223,
    2, 223, 1006, 224, 449, 101, 1, 223, 223, 1108, 677, 677, 224, 102, 2, 223, 223, 1006, 224,
    464, 101, 1, 223, 223, 1007, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 479, 101, 1, 223, 223,
    1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 494, 101, 1, 223, 223, 108, 226, 226, 224,
    1002, 223, 2, 223, 1006, 224, 509, 101, 1, 223, 223, 107, 226, 226, 224, 102, 2, 223, 223,
    1006, 224, 524, 101, 1, 223, 223, 1107, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 539, 1001,
    223, 1, 223, 108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 554, 101, 1, 223, 223, 1007,
    226, 226, 224, 102, 2, 223, 223, 1005, 224, 569, 101, 1, 223, 223, 8, 226, 226, 224, 102, 2,
    223, 223, 1006, 224, 584, 101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1005, 224,
    599, 1001, 223, 1, 223, 107, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 614, 1001, 223, 1,
    223, 1108, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 629, 101, 1, 223, 223, 7, 677, 677, 224,
    1002, 223, 2, 223, 1005, 224, 644, 1001, 223, 1, 223, 108, 677, 677, 224, 102, 2, 223, 223,
    1005, 224, 659, 101, 1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 674, 101,
    1, 223, 223, 4, 223, 99, 226,
];
