use std::str::FromStr;
use regex::Regex;

pub fn step1(input : String) {
    let program = Program::parse_prog(&input);
    let mut state = State::init();
    while program.is_running(&state) {
        program.run_instruction(&mut state);
    }
    println!("{}", state.get(0));
}

pub fn step2(input : String) {
    let program = Program::parse_prog(&input);
    let mut state = State::init();

    state.set(0, 1);
    while program.is_running(&state) {
        program.run_instruction(&mut state);
    }
    println!("{}", state.get(0));
}

struct Program {
    ip_reg : usize,
    instructions : Vec<Instruction>,
}

impl Program {
    pub fn parse_prog(input : &str) -> Program {
        let mut ip = None;
        let mut insts = Vec::new();
        let ip_re = Regex::new(r"#ip (\d)").unwrap();
        let inst_re = Regex::new(r"(\w+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();

        for line in input.lines() {
            if let Some(caps) = ip_re.captures(line) {
                ip = Some(usize::from_str(&caps[1]).unwrap());
            }
            if let Some(caps) = inst_re.captures(line) {
                insts.push(Instruction::new(&caps[1], (usize::from_str(&caps[2]).unwrap(), usize::from_str(&caps[3]).unwrap(), usize::from_str(&caps[4]).unwrap())));
            }
        }
        Program {
            ip_reg: ip.unwrap(),
            instructions: insts,
        }
    }

    fn is_running(&self, state : &State) -> bool {
        state.get(self.ip_reg) < self.instructions.len()
    }

    fn run_instruction(&self, state : &mut State) {
        state.set(self.ip_reg, state.ip);
        let inst = &self.instructions[state.ip];
        inst.run(state);
        state.ip = state.get(self.ip_reg);
        state.ip += 1;
    }
}

struct Instruction {
    op : Op,
    params: (usize, usize, usize),
}

impl Instruction {
    pub fn new(name : &str, params : (usize, usize, usize)) -> Instruction {
        Instruction {
            op: Op::from_name(name),
            params,
        }
    }

    fn run(&self, state : &mut State) {
        match self.op {
            Op::Addr => state.set(self.params.2, state.get(self.params.0) + state.get(self.params.1)),
            Op::Addi => state.set(self.params.2, state.get(self.params.0) + self.params.1),
            Op::Mulr => state.set(self.params.2, state.get(self.params.0) * state.get(self.params.1)),
            Op::Muli => state.set(self.params.2, state.get(self.params.0) * self.params.1),
            Op::Banr => state.set(self.params.2, state.get(self.params.0) & state.get(self.params.1)),
            Op::Bani => state.set(self.params.2, state.get(self.params.0) & self.params.1),
            Op::Borr => state.set(self.params.2, state.get(self.params.0) | state.get(self.params.1)),
            Op::Bori => state.set(self.params.2, state.get(self.params.0) | self.params.1),
            Op::Setr => state.set(self.params.2, state.get(self.params.0)),
            Op::Seti => state.set(self.params.2, self.params.0),
            Op::Gtrr => state.set(self.params.2, if state.get(self.params.0) > state.get(self.params.1) { 1 } else { 0 }),
            Op::Gtri => state.set(self.params.2, if state.get(self.params.0) > self.params.1 { 1 } else { 0 }),
            Op::Gtir => state.set(self.params.2, if self.params.0 > state.get(self.params.1) { 1 } else { 0 }),
            Op::Eqrr => state.set(self.params.2, if state.get(self.params.0) == state.get(self.params.1) { 1 } else { 0 }),
            Op::Eqri => state.set(self.params.2, if state.get(self.params.0) == self.params.1 { 1 } else { 0 }),
            Op::Eqir => state.set(self.params.2, if self.params.0 == state.get(self.params.1) { 1 } else { 0 }),
        }
    }
}

struct State {
    ip : usize,
    regs : Vec<usize>,
}

impl State {
    pub fn init() -> State {
        State {
            ip: 0,
            regs: vec![0; 6],
        }
    }

    fn get(&self, r : usize) -> usize {
        self.regs[r]
    }

    fn set(&mut self, r : usize, val : usize) {
        self.regs[r] = val;
    }
}

enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtrr,
    Gtri,
    Gtir,
    Eqrr,
    Eqri,
    Eqir,
}

impl Op {
    fn from_name(name : &str) -> Op {
        match name {
            "addr" => Op::Addr,
            "addi" => Op::Addi,
            "mulr" => Op::Mulr,
            "muli" => Op::Muli,
            "banr" => Op::Banr,
            "bani" => Op::Bani,
            "borr" => Op::Borr,
            "bori" => Op::Bori,
            "setr" => Op::Setr,
            "seti" => Op::Seti,
            "gtrr" => Op::Gtrr,
            "gtri" => Op::Gtri,
            "gtir" => Op::Gtir,
            "eqrr" => Op::Eqrr,
            "eqri" => Op::Eqri,
            "eqir" => Op::Eqir,
            _ => panic!(),
        }
    }
}
