use std::fmt;
use std::str::FromStr;
use regex::Regex;
use std::fmt::Formatter;
use std::fmt::Error;

pub fn step1(input : String) {
    let program = Program::parse_prog(&input);
    let mut state = State::init();
    while program.is_running(&state) {
        program.run_instruction(&mut state);
    }
    println!("{}", state.get(0));
}

pub fn step2(input : String) {
    let mut program = Program::parse_prog(&input);
    let mut state = State::init();

    println!("{:?}", program);

    program.optimize();

    println!("{:?}", program);

    let mut sum = 0;
    for i in 1..=10_551_339 {
        let q = 10_551_339 / i;
        if i * q == 10_551_339 {
            sum += i;
        }
    }
    println!("{}", sum);
}

struct Program {
    ip_reg : usize,
    instructions : Vec<Instruction2>,
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
                insts.push(Instruction2::OpInstruction(Instruction::new(&caps[1], (usize::from_str(&caps[2]).unwrap(), usize::from_str(&caps[3]).unwrap(), usize::from_str(&caps[4]).unwrap()))));
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

    fn optimize(&mut self) {
        self.inline_ip();
        self.inline_if();
    }

    fn inline_ip(&mut self) {
        let mut new_insts = self.instructions.clone();
        for (i, inst) in self.instructions.iter().enumerate() {
            if let Instruction2::OpInstruction(op_inst) = inst {
                let new_inst = match op_inst.op {
                    //Op::Addr if op_inst.params.2 == self.ip_reg && op_inst.params.0 == self.ip_reg && i*2 > self.instructions.len() => Some(Instruction2::Goto(999)),
                    Op::Addr if op_inst.params.1 == self.ip_reg => Some(Instruction2::OpInstruction(Instruction {op: Op::Addi, params: (op_inst.params.0, i, op_inst.params.2)})),
                    Op::Addr if op_inst.params.0 == self.ip_reg => Some(Instruction2::OpInstruction(Instruction {op: Op::Addi, params: (op_inst.params.1, i, op_inst.params.2)})),
                    Op::Mulr if op_inst.params.0 == self.ip_reg && op_inst.params.1 == self.ip_reg && op_inst.params.2 == self.ip_reg => Some(Instruction2::Goto(i * i)),
                    Op::Mulr if op_inst.params.1 == self.ip_reg => Some(Instruction2::OpInstruction(Instruction {op: Op::Muli, params: (op_inst.params.0, i, op_inst.params.2)})),
                    Op::Mulr if op_inst.params.0 == self.ip_reg => Some(Instruction2::OpInstruction(Instruction {op: Op::Muli, params: (op_inst.params.1, i, op_inst.params.2)})),
                    Op::Setr if op_inst.params.0 == self.ip_reg => Some(Instruction2::OpInstruction(Instruction {op: Op::Seti, params: (i, 0, op_inst.params.2)})),
                    Op::Addi if op_inst.params.2 == self.ip_reg && op_inst.params.0 == self.ip_reg => Some(Instruction2::Goto(i + op_inst.params.1)),
                    Op::Seti if op_inst.params.2 == self.ip_reg => Some(Instruction2::Goto(op_inst.params.0)),
                    _ => None,
                };
                if let Some(inst2) = new_inst {
                    new_insts[i] = inst2;
                }
            }
        }
        self.instructions = new_insts;
    }

    fn inline_if(&mut self) {
        let mut new_insts = self.instructions.clone();
        for i in 1..self.instructions.len() {
            if let (Instruction2::OpInstruction(op_inst1), Instruction2::OpInstruction(op_inst2)) = (&self.instructions[i-1], &self.instructions[i]) {
                let new_inst = match (op_inst1.op, op_inst2.op) {
                    (Op::Eqrr, Op::Addi) if op_inst2.params.2 == self.ip_reg && op_inst1.params.2 == op_inst2.params.0 => Some(Instruction2::If(op_inst1.clone(), op_inst2.params.1)),
                    (Op::Gtrr, Op::Addi) if op_inst2.params.2 == self.ip_reg && op_inst1.params.2 == op_inst2.params.0 => Some(Instruction2::If(op_inst1.clone(), op_inst2.params.1)),
                    _ => None,
                };
                if let Some(inst2) = new_inst {
                    new_insts[i-1] = inst2;
                }
            }
        }
        self.instructions = new_insts;
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("ip <-> ${}\n", self.ip_reg));
        for (i, inst) in self.instructions.iter().enumerate() {
            f.write_fmt(format_args!("{}: {:?}", i, inst));
        }
        f.write_str("")
    }
}

#[derive(Clone)]
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

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.op {
            Op::Addr => f.write_fmt(format_args!("${} = ${} + ${}\n", self.params.2, self.params.0, self.params.1)),
            Op::Addi => f.write_fmt(format_args!("${} = ${} + {}\n", self.params.2, self.params.0, self.params.1)),
            Op::Mulr => f.write_fmt(format_args!("${} = ${} * ${}\n", self.params.2, self.params.0, self.params.1)),
            Op::Muli => f.write_fmt(format_args!("${} = ${} * {}\n", self.params.2, self.params.0, self.params.1)),
            Op::Banr => f.write_fmt(format_args!("${} = ${} && ${}\n", self.params.2, self.params.0, self.params.1)),
            Op::Bani => f.write_fmt(format_args!("${} = ${} && {}\n", self.params.2, self.params.0, self.params.1)),
            Op::Borr => f.write_fmt(format_args!("${} = ${} || ${}\n", self.params.2, self.params.0, self.params.1)),
            Op::Bori => f.write_fmt(format_args!("${} = ${} || {}\n", self.params.2, self.params.0, self.params.1)),
            Op::Setr => f.write_fmt(format_args!("${} = ${}\n", self.params.2, self.params.0)),
            Op::Seti => f.write_fmt(format_args!("${} = {}\n", self.params.2, self.params.0)),
            Op::Gtrr => f.write_fmt(format_args!("${} = ${} > ${}\n", self.params.2, self.params.0, self.params.1)),
            Op::Gtri => f.write_fmt(format_args!("${} = ${} > {}\n", self.params.2, self.params.0, self.params.1)),
            Op::Gtir => f.write_fmt(format_args!("${} = {} > ${}\n", self.params.2, self.params.0, self.params.1)),
            Op::Eqrr => f.write_fmt(format_args!("${} = ${} == ${}\n", self.params.2, self.params.0, self.params.1)),
            Op::Eqri => f.write_fmt(format_args!("${} = ${} == {}\n", self.params.2, self.params.0, self.params.1)),
            Op::Eqir => f.write_fmt(format_args!("${} = {} == ${}\n", self.params.2, self.params.0, self.params.1)),
        }
    }
}

#[derive(Clone)]
enum Instruction2 {
    OpInstruction(Instruction),
    Goto(usize),
    If(Instruction, usize),
}

impl Instruction2 {
    fn run(&self, state : &mut State) {
        match self {
            Instruction2::OpInstruction(inst) => inst.run(state),
            Instruction2::Goto(to) => state.ip = *to,
            Instruction2::If(op, offset) => { op.run(state); state.ip = *offset + &state.regs[op.params.2]},
        }
    }
}

impl fmt::Debug for Instruction2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Instruction2::OpInstruction(inst) => write!(f, "{:?}", inst),
            Instruction2::Goto(to) => writeln!(f, "goto {}", to + 1),
            Instruction2::If(op, offset) => writeln!(f, "if {:?} then goto {} else goto {}", op, offset + 2, offset + 1),
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

#[derive(Copy, Clone, Eq, PartialEq)]
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
