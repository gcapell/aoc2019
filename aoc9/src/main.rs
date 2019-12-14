mod interpreter;
use permutohedron::LexicalPermutation;
use std::collections::VecDeque;

fn main() {
    let mut i = interpreter::new(TEST1);
    println!("{}", max_amplify(&mut i, TEST1));
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

