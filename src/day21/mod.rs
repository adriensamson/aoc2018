use day19::{Program, State};
use std::collections::HashSet;

pub fn step1(input : String) {
    let mut program = Program::parse_prog(&input);
    let mut state = State::init();

    program.optimize();

    println!("{:?}", program);

    println!("{}", run_loop(0));
}
/*
$4 = 0

loop {
    $3 = $4 | 0x10000           # 0x10000
    $4 = 707129                 # 707129

    loop {
        $4 = ($4 + ($3 % 0x100)) % 0x1000000
                                # 707129    # 8765139   # 7902514
        $4 = ($4 * 65899) % 0x1000000
                                # 8765139   # 7902513   # 2985446
        if 256 > $3 {
            break
        }

        $3 /= 256               # 0x100    # 1
    }

    if $4 == $0 {
        halt
    }
}
*/

fn run_loop(previous : usize) -> usize {
    let mut c = previous | 0x10000;
    let mut d = 707129;
    loop {
        d = (d + (c % 0x100)) % 0x1000000;
        d = (d * 65899) % 0x1000000;
        if 256 > c {
            break;
        }
        c /= 256;
    }
    d
}

pub fn step2(_input : String) {
    let mut set = HashSet::new();
    let mut d = 0;
    let max = loop {
        let next = run_loop(d);
        if set.contains(&next) {
            break d;
        }
        set.insert(next);
        d = next;
    };

    println!("{}", max);
}
