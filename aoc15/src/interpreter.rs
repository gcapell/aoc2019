use std::collections::HashMap;
use std::collections::VecDeque;

pub type Word = i64;

#[derive(Debug)]
pub struct MachineState {
    mem: Vec<Word>,
    pc: usize,
    rbase: Word,
    pub halted: bool,
    extended_mem: HashMap<Word, Word>,
}

pub struct Channel {
    m: MachineState,
    i: VecDeque<Word>,
    o: VecDeque<Word>,
}

pub fn new_channel(prog: &[Word]) -> Channel {
    Channel {
        m: new(prog),
        i: VecDeque::new(),
        o: VecDeque::new(),
    }
}

impl Channel {
    pub fn send(&mut self, n: Word) -> Word {
        assert!(self.i.is_empty());
        assert!(self.o.is_empty());
        self.i.push_back(n);
        self.m.run(&mut self.i, &mut self.o);
        assert_eq!(self.o.len(), 1);
        self.o.pop_front().unwrap()
    }
}

pub fn new(prog: &[Word]) -> MachineState {
    MachineState {
        mem: prog.to_vec(),
        pc: 0,
        rbase: 0,
        halted: false,
        extended_mem: HashMap::new(),
    }
}

pub enum Signal {
    NeedsInput,
    Exit,
}

impl MachineState {
    pub fn run(&mut self, i: &mut VecDeque<Word>, o: &mut VecDeque<Word>) -> Signal {
        assert!(!self.halted);
        assert_eq!(o.len(), 0);

        loop {
            let (op, modes) = self.next_instruction();
            match op {
                1 => {
                    let (s1, s2, d) = self.params3(modes, S, S, D);
                    self.mem_set(d, s1 + s2);
                }
                2 => {
                    let (s1, s2, d) = self.params3(modes, S, S, D);
                    self.mem_set(d, s1 * s2);
                }
                3 => {
                    let d = self.params1(modes, D);
                    match i.pop_front() {
                        Some(val) => {
                            println!("<- {}", val);
                            self.mem_set(d, val)
                        }
                        None => {
                            self.pc -= 2;
                            return Signal::NeedsInput;
                        }
                    }
                }
                4 => {
                    let s = self.params1(modes, S);
                    println!("-> {}", s);
                    o.push_back(s);
                }
                5 => {
                    let (s, d) = self.params2(modes, S, S);
                    if s != 0 {
                        self.pc = d as usize
                    }
                }
                6 => {
                    let (s, d) = self.params2(modes, S, S);
                    if s == 0 {
                        self.pc = d as usize
                    }
                }
                7 => {
                    let (a, b, d) = self.params3(modes, S, S, D);
                    self.mem_set(d, bool_to_int(a < b));
                }
                8 => {
                    let (a, b, d) = self.params3(modes, S, S, D);
                    self.mem_set(d, bool_to_int(a == b));
                }
                9 => self.rbase += self.params1(modes, S),
                99 => {
                    self.halted = true;
                    return Signal::Exit;
                }
                _ => panic!("unknown opcode {}", op),
            }
        }
    }

    fn next_instruction(&mut self) -> (Word, ModeTuple) {
        let (opcode, modes) = unpack(self.mem[self.pc]);
        self.pc += 1;
        (opcode, modes)
    }

    fn params3(
        &mut self,
        m: ModeTuple,
        a: ParamType,
        b: ParamType,
        c: ParamType,
    ) -> (Word, Word, Word) {
        (self.param(m.0, a), self.param(m.1, b), self.param(m.2, c))
    }
    fn params2(&mut self, m: ModeTuple, a: ParamType, b: ParamType) -> (Word, Word) {
        (self.param(m.0, a), self.param(m.1, b))
    }
    fn params1(&mut self, m: ModeTuple, a: ParamType) -> Word {
        self.param(m.0, a)
    }

    fn mem_get(&mut self, p: Word) -> Word {
        if p < self.mem.len() as Word {
            return self.mem[p as usize];
        }
        *self.extended_mem.get(&p).unwrap_or(&0)
    }
    fn mem_set(&mut self, k: Word, v: Word) {
        if k < self.mem.len() as Word {
            self.mem[k as usize] = v;
            return;
        }
        self.extended_mem.insert(k, v);
    }

    fn param(&mut self, mode: Mode, pt: ParamType) -> Word {
        let v = self.mem[self.pc];
        self.pc += 1;
        match (pt, mode) {
            (S, 0) => self.mem_get(v),
            (S, 1) => v,
            (S, 2) => self.mem_get(v + self.rbase),
            (D, 0) => v,
            (D, 2) => v + self.rbase,
            (D, _) => panic!("mode {} for Destination", mode),
            (S, _) => panic!("unknown mode {} for Src", mode),
        }
    }
}

type Mode = i8;
type ModeTuple = (Mode, Mode, Mode);

#[derive(Debug)]
enum ParamType {
    S,
    D,
}
use self::ParamType::*;

fn unpack(mut i: Word) -> (Word, ModeTuple) {
    let g = &mut i;
    (ipop(g, 100), (imode(g), imode(g), imode(g)))
}

fn imode(i: &mut Word) -> Mode {
    ipop(i, 10) as Mode
}

fn ipop(i: &mut Word, n: Word) -> Word {
    let reply = (*i) % n;
    (*i) /= n;
    reply
}

fn bool_to_int(a: bool) -> Word {
    if a {
        1
    } else {
        0
    }
}
