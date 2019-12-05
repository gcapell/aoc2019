use std::collections::VecDeque;
use std::collections::HashMap;
use std::fmt;

struct MachineState{
    mem: Vec<i32>,
    input: VecDeque<i32>,
    output: Vec<i32>,
	instructions: HashMap<i32,Op>,
}
enum ParamType {
	S,
	D,
}
enum NextPC {
    Relative,
    Absolute(i32),
    Exit,
}

use self::ParamType::*;

type OpFn = fn( m: &mut MachineState, p:&[i32])->NextPC;

struct Op {
	name: String,
	params :Vec<ParamType>,
	run : OpFn,
}

impl MachineState {
fn interpret(&mut self)  {
    let mut pc = 0;
    loop {
        let (func, params, size) = self.decode(pc);
        match func(self, params) {
            NextPC::Relative => pc += size,
            NextPC::Absolute(addr) => pc = addr as usize,
            NextPC::Exit => return,
        }
    }
}

fn decode(&self, pc:usize)-> (OpFn, Vec<i32>, usize){
	let op = self.mem[pc];
	let (opcode, modes) = unpack(self.mem[pc]);
	let i = self.instruction[opcode].unpack();
	let params = Vec::new();
	for (n,p) in i.params.enumerate() {
		let v= self.mem[pc+1+n];
		match modes[n] {
			Mode::Position => {
				params.push(match p {
					S=>self.mem[v],
					D=>v}
				);
			}
			Mode::Immediate =>  {
				params.push(match p {
					S=>v,
					D=>panic!()}
				);
			}
		}
	}
	return (i.run, params, i.params.len() + 1);
}
}

fn main() {
	let mut state = MachineState{
	    mem: Prog5.to_vec(),
		input:VecDeque::new(),
		output: Vec::new(),
		instructions: HashMap::new(),
	};

	init_instructions(&mut state.instructions);
    state.input.push_back(5);
	state.run();
    println!("{:?}", state.output);
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

fn unpack(mut i: i32) -> (i32, Mode, Mode, Mode) {
    let opcode = ipop(&mut i, 100);
    let mode0 = pop_mode(&mut i);
    let mode1 = pop_mode(&mut i);
    let mode2 = pop_mode(&mut i);
    (opcode, mode0, mode1, mode2)
}

fn pop_mode(i: &mut i32) -> Mode {
    let n = ipop(i, 10);
    match n {
        0 => Mode::Position,
        1 => Mode::Immediate,
        _ => panic!("bad mode {}", n),
    }
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

fn init_instructions(i:&mut HashMap<i32,Op>)  {
	i.insert(1, Op {
		name: "Add".to_string(),
		params:vec!(S,S,D),
		run: |m,p| {
			if let [s1,s2,d] = p[..] {
				m.mem[d as usize]=s1+s2;
	            NextPC::Exit // FIME	
			} else {
				 panic!("add params")
			 }
		}
	});
	i.insert(2, Op {
		name: "Mul".to_string(),
		params:vec!(S,S,D),
		run: |m,p| {
			if let [s1,s2,d] = p[..] {
				m.mem[d as usize]=s1*s2;
	            NextPC::Relative	
				} else {panic!()}
		}
	});
	i.insert(3, Op {
		name: "Input".to_string(),
		params:vec!(D),
		run: |m,p| {
			if let [d] = p[..] {
				m.mem[d as usize]=m.input.pop_front().unwrap();
	            NextPC::Relative
				} else {panic!()}
		}
	});	
	i.insert(4, Op {
		name: "Output".to_string(),
		params:vec!(S),
		run: |m,p| {
			if let [s] = p[..] {
				m.ouput.push(s);
	            NextPC::Relative
				} else {panic!()}
		}
	});	 
	i.insert(5, Op {
		name: "JumpIfTrue".to_string(),
		params:vec!(S,D),
		run: |m,p| {
			if let [s,d] = p[..] {
				if s != 0 {
						NextPC:PAbsolute(d)
					} else {
						NextPC::Relative
					}
				
			} else {panic!()}
		}
	});	 
	i.insert(6, Op {
		name: "JumpIfFalse".to_string(),
		params:vec!(S,D),
		run: |m,p| {
			if let [s,d] = p[..] {
				if s == 0 {
					NextPC:PAbsolute(d)
					} else {
						NextPC::Relative
					}
				
			} else {panic!()}
		}
	});	 
	i.insert(7, Op {
		name: "LessThan".to_string(),
		params:vec!(S,S,D),
		run: |m,p| {
			if let [a,b,d] = p[..] {
				m.mem[d] = bool_to_int(a<b);
				NextPC::Relative
			} else {panic!()}
		}
	});	 
	i.insert(8, Op {
		name: "Equals".to_string(),
		params:vec!(S,S,D),
		run: |m,p| {
			if let [a,b,d] = p[..] {
				m.mem[d] = bool_to_int(a==b);
				NextPC::Relative
			} else {panic!()}
		}
	});	  
	i.insert(9, Op {
		name: "Exit".to_string(),
		params:Vec::new(),
		run: |m,p| { NextPC::Exit },
	});	  
}

static Prog5:&[i32] =&[
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 72, 36, 225, 1101, 87, 26, 225, 2,
        144, 13, 224, 101, -1872, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223,
        224, 223, 1102, 66, 61, 225, 1102, 25, 49, 224, 101, -1225, 224, 224, 4, 224, 1002, 223, 8,
        223, 1001, 224, 5, 224, 1, 223, 224, 223, 1101, 35, 77, 224, 101, -112, 224, 224, 4, 224,
        102, 8, 223, 223, 1001, 224, 2, 224, 1, 223, 224, 223, 1002, 195, 30, 224, 1001, 224,
        -2550, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 1, 224, 1, 224, 223, 223, 1102, 30, 44,
        225, 1102, 24, 21, 225, 1, 170, 117, 224, 101, -46, 224, 224, 4, 224, 1002, 223, 8, 223,
        101, 5, 224, 224, 1, 224, 223, 223, 1102, 63, 26, 225, 102, 74, 114, 224, 1001, 224, -3256,
        224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 1101, 58, 22, 225, 101,
        13, 17, 224, 101, -100, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 224, 223,
        223, 1101, 85, 18, 225, 1001, 44, 7, 224, 101, -68, 224, 224, 4, 224, 102, 8, 223, 223,
        1001, 224, 5, 224, 1, 223, 224, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105,
        1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 7, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 329, 101, 1, 223, 223, 8,
        677, 226, 224, 1002, 223, 2, 223, 1005, 224, 344, 1001, 223, 1, 223, 1107, 677, 677, 224,
        102, 2, 223, 223, 1005, 224, 359, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2, 223, 223,
        1005, 224, 374, 101, 1, 223, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 389, 101,
        1, 223, 223, 8, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 404, 101, 1, 223, 223, 1008,
        226, 677, 224, 1002, 223, 2, 223, 1005, 224, 419, 1001, 223, 1, 223, 107, 677, 677, 224,
        102, 2, 223, 223, 1005, 224, 434, 101, 1, 223, 223, 1108, 677, 226, 224, 1002, 223, 2, 223,
        1006, 224, 449, 101, 1, 223, 223, 1108, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 464,
        101, 1, 223, 223, 1007, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 479, 101, 1, 223, 223,
        1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 494, 101, 1, 223, 223, 108, 226, 226,
        224, 1002, 223, 2, 223, 1006, 224, 509, 101, 1, 223, 223, 107, 226, 226, 224, 102, 2, 223,
        223, 1006, 224, 524, 101, 1, 223, 223, 1107, 677, 226, 224, 102, 2, 223, 223, 1005, 224,
        539, 1001, 223, 1, 223, 108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 554, 101, 1, 223,
        223, 1007, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 569, 101, 1, 223, 223, 8, 226, 226,
        224, 102, 2, 223, 223, 1006, 224, 584, 101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223, 2,
        223, 1005, 224, 599, 1001, 223, 1, 223, 107, 226, 677, 224, 1002, 223, 2, 223, 1005, 224,
        614, 1001, 223, 1, 223, 1108, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 629, 101, 1, 223,
        223, 7, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 644, 1001, 223, 1, 223, 108, 677, 677,
        224, 102, 2, 223, 223, 1005, 224, 659, 101, 1, 223, 223, 1007, 677, 677, 224, 102, 2, 223,
        223, 1006, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226];
