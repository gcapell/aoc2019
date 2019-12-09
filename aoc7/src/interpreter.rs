use std::collections::VecDeque;

#[derive(Debug)]
pub struct MachineState {
    mem: Vec<i32>,
    pc: usize,
}

pub fn new(prog: &[i32]) -> MachineState {
      MachineState {
        mem: prog.to_vec(),
        pc: 0,
    }
}

pub enum Signal {
    NeedsInput,
    Exit,
}

impl MachineState {
    pub fn set_prog(&mut self, prog: &[i32]) {
        self.pc = 0;
        self.mem = prog.to_vec();
    }
	
    pub fn run(&mut self, i: &mut VecDeque<i32>, o: &mut VecDeque<i32>) ->Signal {
        loop {
			let (op, modes) = self.next_instruction();
			match op {
			    1 => {
			    	let (s1,s2,d) = self.params3(modes, S, S, D);
					self.mem[d as usize] = s1 + s2;
			    }
				2 => {
			    	let (s1,s2,d) = self.params3(modes, S, S, D);
					self.mem[d as usize] = s1 * s2;
				}
				3 => {
			    	let d = self.params1(modes, D);
                    match i.pop_front() {
                        Some(val) => {
                            self.mem[d as usize] = val;
                            //println!("mem[{}] = {}", d, val);
                        }
                        None => return Signal::NeedsInput,
                    }
				}
				4 => {
			    	let s = self.params1(modes, S);
					o.push_back(s);
				}
				5 => {
			    	let (s,d) = self.params2(modes, S, S);
					if s != 0 {
						self.pc = d as usize
					}
				}
				6 => {
			    	let (s,d) = self.params2(modes, S, S);
					if s == 0 {
						self.pc = d as usize
					}
				}				
				7 => {
			    	let (a,b,d) = self.params3(modes, S, S, D);
					self.mem[d as usize] = bool_to_int(a < b);
				}
				8 => {
			    	let (a,b,d) = self.params3(modes, S, S, D);
					self.mem[d as usize] = bool_to_int(a == b);
				}
				99 => {
			    	return Signal::Exit;
				}
				_ => panic!("unknown opcode {}", op),		
			}
        }
    }

	fn next_instruction(&mut self)->(i32, ModeTuple){
		let (opcode, modes) = unpack(self.mem[self.pc]);
		self.pc += 1;
		(opcode, modes)
	}
	
	fn params3(&mut self, m:ModeTuple, a:ParamType, b:ParamType, c:ParamType)->(i32,i32,i32){
		(self.param(m.0,a), self.param(m.1,b), self.param(m.2,c))
	}
	fn params2(&mut self, m:ModeTuple, a:ParamType, b:ParamType)->(i32,i32){
		(self.param(m.0,a), self.param(m.1,b))
	}
	fn params1(&mut self, m:ModeTuple, a:ParamType)->i32 {
		self.param(m.0,a)
	}
	fn param(&mut self, mode:i32, pt:ParamType)->i32{
		let v = self.mem[self.pc];
		self.pc += 1;
		match (pt,mode) {
			(S,0) => self.mem[v as usize],
			(S,1) => v,
			(D,0) => v,
			(D,_)=> panic!("mode {} for Destination", mode),
			(S,_)=>panic!("unknown mode {} for Src", mode),
		}
	}
}

type ModeTuple = (i32,i32,i32);

#[derive(Debug)]
enum ParamType {
    S,
    D,
}
use self::ParamType::*;

fn unpack(mut i: i32) -> (i32, ModeTuple) {
    let opcode = ipop(&mut i, 100);
	let modes = (ipop(&mut i, 10),ipop(&mut i, 10),ipop(&mut i, 10));
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

