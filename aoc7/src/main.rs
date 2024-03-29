mod interpreter;
use interpreter::Signal::*;
use permutohedron::LexicalPermutation;
use std::collections::VecDeque;

fn main() {
    let mut i = interpreter::new(PROG7);
    println!("{}", max_amplify(&mut i, PROG7));
}

fn max_amplify(i: &mut interpreter::MachineState, prog: &[i32]) -> i32 {
    let mut phases = vec![0, 1, 2, 3, 4];
    let mut ok = true;
    let mut max = 0;
    while ok {
        let val = amplify(i, prog, &phases);
        if val > max {
            max = val;
        }
        ok = phases.next_permutation();
    }
    max
}

fn amplify(interpreter: &mut interpreter::MachineState, prog: &[i32], phases: &[i32]) -> i32 {
    let mut val = 0;
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();
    for s in phases {
        interpreter.set_prog(prog);
        input.push_back(*s);
        input.push_back(val);
        interpreter.run(&mut input, &mut output);
        val = output.pop_front().unwrap()
    }
    val
}

fn feedback_amplify(prog: &[i32], seq: &[i32]) -> i32 {
    let mut i = VecDeque::new();
    let mut o = VecDeque::new();
    let mut machines: Vec<interpreter::MachineState> = seq
        .iter()
        .map(|s| {
            let mut m = interpreter::new(prog);
            i.push_back(*s);
            match m.run(&mut i, &mut o) {
                NeedsInput => (),
                Exit => panic!("unexpected exit init feedback_amplify"),
            }
            m
        })
        .collect();

    i.push_back(0);
    assert_eq!(o.len(), 0, "banana");
    let last = machines.len() - 1;
    loop {
        for (n, m) in machines.iter_mut().enumerate() {
            assert_eq!(i.len(), 1);
            assert_eq!(o.len(), 0);
            let sig = m.run(&mut i, &mut o);
            assert_eq!(o.len(), 1);
            let val = o.pop_front().unwrap();
            println!("machine {}, output {}", n, val);
            i.push_back(val);
            if m.halted && n == last {
                return val;
            }
        }
    }
}

#[test]
fn test_feedback_amplify() {
    let prog: &[i32] = &[
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    let seq: &[i32] = &[9, 8, 7, 6, 5];
    assert_eq!(feedback_amplify(prog, seq), 139629729);
}

#[test]
fn test_max_amplify() {
    let test_data = [(TEST1, 43210), (TEST2, 54321), (TEST3, 65210)];
    let mut i = interpreter::new(test_data[0].0);

    for (prog, expected) in test_data.iter() {
        assert_eq!(max_amplify(&mut i, prog), *expected);
    }
}

static TEST1: &[i32] = &[
    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
];

static TEST2: &[i32] = &[
    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0,
    0,
];
static TEST3: &[i32] = &[
    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31,
    31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
];

static PROG7: &[i32] = &[
    3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 42, 55, 64, 77, 94, 175, 256, 337, 418, 99999, 3, 9,
    102, 4, 9, 9, 1001, 9, 5, 9, 102, 2, 9, 9, 101, 3, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 101, 5,
    9, 9, 4, 9, 99, 3, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9, 102, 4, 9, 9, 101, 5, 9, 9, 4, 9, 99, 3,
    9, 102, 5, 9, 9, 1001, 9, 3, 9, 1002, 9, 5, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101,
    1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9,
    3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9,
    9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9,
    3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9,
    2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3,
    9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2,
    9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
    9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2,
    9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3,
    9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9,
    4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9,
    1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9,
    4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
    1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99,
];
