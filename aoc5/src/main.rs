use std::collections::VecDeque;
use std::fmt;

fn main() {
    let mut mem: Vec<i32> = vec![
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
        223, 1006, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226,
    ];

    let mut input: VecDeque<i32> = VecDeque::new();
    input.push_back(5);

    let output = interpret(&mut mem, &mut input);
    println!("{:?}", output);
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

struct Addr {
    a: i32,
    m: Mode,
}

impl Addr {
    fn fetch(&self, mem: &[i32]) -> i32 {
        match self.m {
            Mode::Position => mem[self.a as usize],
            Mode::Immediate => self.a,
        }
    }
    fn store(&self, mem: &mut [i32], val: i32) {
        match self.m {
            Mode::Position => mem[self.a as usize] = val,
            Mode::Immediate => panic!("store immediate"),
        }
    }
}

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.m {
                Mode::Position => "@",
                Mode::Immediate => "i",
            },
            self.a
        )
    }
}

#[derive(Debug)]
enum Instruction {
    Add(Addr, Addr, Addr),
    Mul(Addr, Addr, Addr),
    Input(Addr),
    Output(Addr),
    JumpIfTrue(Addr, Addr),
    JumpIfFalse(Addr, Addr),
    LessThan(Addr, Addr, Addr),
    Equals(Addr, Addr, Addr),

    Exit,
}

fn interpret(mem: &mut [i32], input: &mut VecDeque<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut pc = 0;
    loop {
        let i = decode(&mem[pc..]);
        match i.interpret(mem, input, &mut output) {
            NextPC::Relative(delta) => pc += delta as usize,
            NextPC::Absolute(addr) => pc = addr as usize,
            NextPC::Exit => return output,
        }
    }
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

fn decode(mem: &[i32]) -> Instruction {
    let (opcode, mode0, mode1, mode2) = unpack(mem[0]);
    match opcode {
        1 => Instruction::Add(
            Addr {
                a: mem[1],
                m: mode0,
            },
            Addr {
                a: mem[2],
                m: mode1,
            },
            Addr {
                a: mem[3],
                m: mode2,
            },
        ),
        2 => Instruction::Mul(
            Addr {
                a: mem[1],
                m: mode0,
            },
            Addr {
                a: mem[2],
                m: mode1,
            },
            Addr {
                a: mem[3],
                m: mode2,
            },
        ),
        3 => Instruction::Input(Addr {
            a: mem[1],
            m: mode0,
        }),
        4 => Instruction::Output(Addr {
            a: mem[1],
            m: mode0,
        }),
        5 => Instruction::JumpIfTrue(
            Addr {
                a: mem[1],
                m: mode0,
            },
            Addr {
                a: mem[2],
                m: mode1,
            },
        ),
        6 => Instruction::JumpIfFalse(
            Addr {
                a: mem[1],
                m: mode0,
            },
            Addr {
                a: mem[2],
                m: mode1,
            },
        ),
        7 => Instruction::LessThan(
            Addr {
                a: mem[1],
                m: mode0,
            },
            Addr {
                a: mem[2],
                m: mode1,
            },
            Addr {
                a: mem[3],
                m: mode2,
            },
        ),

        8 => Instruction::Equals(
            Addr {
                a: mem[1],
                m: mode0,
            },
            Addr {
                a: mem[2],
                m: mode1,
            },
            Addr {
                a: mem[3],
                m: mode2,
            },
        ),

        99 => Instruction::Exit,
        _ => panic!("{}", opcode),
    }
}

enum NextPC {
    Relative(i32),
    Absolute(i32),
    Exit,
}

fn bool_to_int(a: bool) -> i32 {
    if a {
        1
    } else {
        0
    }
}

impl Instruction {
    fn interpret(
        &self,
        mem: &mut [i32],
        input: &mut VecDeque<i32>,
        output: &mut Vec<i32>,
    ) -> NextPC {
        println!("{:?}", self);
        match self {
            Instruction::Add(s1, s2, dst) => {
                let v = s1.fetch(mem) + s2.fetch(mem);
                dst.store(mem, v);
                NextPC::Relative(4)
            }
            Instruction::Mul(s1, s2, dst) => {
                let v = s1.fetch(mem) * s2.fetch(mem);
                dst.store(mem, v);
                NextPC::Relative(4)
            }
            Instruction::Input(dst) => {
                dst.store(mem, input.pop_front().unwrap());
                NextPC::Relative(2)
            }
            Instruction::Output(src) => {
                output.push(src.fetch(mem));
                NextPC::Relative(2)
            }
            Instruction::JumpIfTrue(src, dst) => {
                if src.fetch(mem) != 0 {
                    NextPC::Absolute(dst.fetch(mem))
                } else {
                    NextPC::Relative(3)
                }
            }
            Instruction::JumpIfFalse(src, dst) => {
                if src.fetch(mem) == 0 {
                    NextPC::Absolute(dst.fetch(mem))
                } else {
                    NextPC::Relative(3)
                }
            }
            Instruction::LessThan(a, b, dst) => {
                dst.store(mem, bool_to_int(a.fetch(mem) < b.fetch(mem)));
                NextPC::Relative(4)
            }
            Instruction::Equals(a, b, dst) => {
                dst.store(mem, bool_to_int(a.fetch(mem) == b.fetch(mem)));
                NextPC::Relative(4)
            }
            Instruction::Exit => NextPC::Exit,
        }
    }
}
