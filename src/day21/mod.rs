use day19::{Program, State};

pub fn step1(input : String) {
    let mut program = Program::parse_prog(&input);
    let mut state = State::init();

    program.optimize();

    println!("{:?}", program);
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



pub fn step2(input : String) {

}
