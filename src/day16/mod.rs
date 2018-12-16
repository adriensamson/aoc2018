use regex::Regex;
use std::str::FromStr;

pub fn step1(input : String) {
    let (samples, _) = parse_input(&input);
    let n = samples.iter().filter(|s| sample_possible_ops(s).len() >= 3).count();
    println!("{}", n)
}

pub fn step2(input : String) {
    let (samples, prog) = parse_input(&input);
    let mut possibles : Vec<Option<Vec<&OpFn>>> = (0..16).map(|_| None).collect();
    for sample in samples.iter() {
        let opcode = sample.inst.0 as usize;
        let ops = sample_possible_ops(sample);
        possibles[opcode] = match possibles[opcode] {
            None => Some(ops),
            Some(ref before) => Some(before.iter().filter(|f| ops.contains(f)).map(|&f| f).collect()),
        }
    }

    let mut possibles : Vec<Vec<&OpFn>> = possibles.iter().map(|o| o.clone().unwrap()).collect();

    let mut known : Vec<Option<&OpFn>> = (0..16).map(|_| None).collect();
    while known.iter().any(|k| k.is_none()) {
        for i in 0..16 {
            if known[i].is_some() {
                continue;
            }
            if possibles[i].len() == 1 {
                let f = possibles[i][0];
                known[i] = Some(f);
                for j in 0..16 {
                    if i == j {
                        continue;
                    }
                    possibles[j] = possibles[j].iter().filter(|&&f2| f2 != f).map(|&f| f).collect();
                }
            }
        }
    }

    let ops : Vec<&OpFn> = known.iter().map(|o| o.clone().unwrap()).collect();

    for (i, fns) in possibles.iter().enumerate() {
        println!("{}: {}", i, fns.len());
    }

    let mut regs = Registers::from_tuple((0, 0, 0, 0));
    for inst in prog {
        regs = ops[inst.0 as usize]((inst.1, inst.2, inst.3), regs);
    }
    println!("reg0 : {}", regs.r0);
}


#[derive(Copy, Clone, Eq, PartialEq)]
struct Registers {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
}

impl Registers {
    fn from_tuple(t : (u32, u32, u32, u32)) -> Registers {
        Registers {
            r0: t.0,
            r1: t.1,
            r2: t.2,
            r3: t.3,
        }
    }

    fn get(&self, n : u32) -> u32 {
        match n {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            _ => panic!(),
        }
    }

    fn with(&self, n : u32, v : u32) -> Registers {
        match n {
            0 => Registers { r0: v, ..self.clone()},
            1 => Registers { r1: v, ..self.clone()},
            2 => Registers { r2: v, ..self.clone()},
            3 => Registers { r3: v, ..self.clone()},
            _ => panic!(),
        }
    }
}

type Instruction = (u32, u32, u32, u32);

struct Sample {
    before : Registers,
    inst : Instruction,
    after : Registers,
}

fn parse_int(s : &str) -> u32 {
    u32::from_str(s).unwrap()
}

fn parse_input(s : &str) -> (Vec<Sample>, Vec<Instruction>) {
    let mut samples = Vec::new();
    let mut prog = Vec::new();

    let before_re = Regex::new(r"Before:\s+\[(\d+), (\d+), (\d+), (\d+)]").unwrap();
    let after_re = Regex::new(r"After:\s+\[(\d+), (\d+), (\d+), (\d+)]").unwrap();
    let inst_re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();

    let mut before = None;
    let mut inst = None;
    for line in s.lines() {
        if let Some(caps) = before_re.captures(line) {
            if before.is_some() {
                panic!("before not none");
            }
            before = Some(Registers::from_tuple((parse_int(&caps[1]), parse_int(&caps[2]), parse_int(&caps[3]), parse_int(&caps[4]))));
        } else if let Some(caps) = after_re.captures(line) {
            if before.is_none() {
                panic!("missing before");
            }
            if inst.is_none() {
                panic!("missing inst");
            }
            let after = Registers::from_tuple((parse_int(&caps[1]), parse_int(&caps[2]), parse_int(&caps[3]), parse_int(&caps[4])));
            samples.push(Sample {
                before: before.unwrap(),
                inst: inst.unwrap(),
                after,
            });
            before = None;
            inst = None;
        } else if let Some(caps) = inst_re.captures(line) {
            let line_inst = (parse_int(&caps[1]), parse_int(&caps[2]), parse_int(&caps[3]), parse_int(&caps[4]));
            match before {
                None => prog.push(line_inst),
                Some(_) => inst = Some(line_inst),
            }
        }
    }

    (samples, prog)
}

fn addr(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, input.get(args.0) + input.get(args.1))
}
fn addi(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, input.get(args.0) + args.1)
}

fn mulr(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, input.get(args.0) * input.get(args.1))
}
fn muli(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, input.get(args.0) * args.1)
}

fn banr(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, input.get(args.0) & input.get(args.1))
}
fn bani(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, input.get(args.0) & args.1)
}

fn borr(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, input.get(args.0) | input.get(args.1))
}
fn bori(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, input.get(args.0) | args.1)
}

fn setr(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, input.get(args.0))
}
fn seti(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, args.0)
}

fn gtrr(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, if input.get(args.0) > input.get(args.1) { 1 } else { 0 })
}
fn gtir(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, if args.0 > input.get(args.1) { 1 } else { 0 })
}
fn gtri(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, if input.get(args.0) > args.1 { 1 } else { 0 })
}

fn eqrr(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, if input.get(args.0) == input.get(args.1) { 1 } else { 0 })
}
fn eqir(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, if args.0 == input.get(args.1) { 1 } else { 0 })
}
fn eqri(args : (u32, u32, u32), input : Registers) -> Registers {
    input.with(args.2, if input.get(args.0) == args.1 { 1 } else { 0 })
}

type OpFn = fn((u32, u32, u32), Registers) -> Registers;

fn sample_possible_ops(sample : &Sample) -> Vec<&OpFn> {
    [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtrr, gtir, gtri, eqrr, eqir, eqri].iter().filter(|f| {
        f((sample.inst.1, sample.inst.2, sample.inst.3), sample.before) == sample.after
    }).collect()
}