use std::str::FromStr;
use regex::Regex;

pub fn step1(input : String) {
    let (ip_reg, prog) = parse_prog(&input);

    let mut ip = 0usize;
    let mut regs = vec![0usize; 6];
    while ip < prog.len() {
        regs[ip_reg] = ip;
        let inst = prog[ip];
        inst.0(inst.1, &mut regs);
        ip = regs[ip_reg];
        ip += 1;
    }
    println!("{}", regs[0]);
}

fn parse_prog(input : &str) -> (usize, Vec<(OpFn, (usize, usize, usize))>) {
    let mut ip = None;
    let mut insts = Vec::new();
    let ip_re = Regex::new(r"#ip (\d)").unwrap();
    let inst_re = Regex::new(r"(\w+)\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();

    for line in input.lines() {
        if let Some(caps) = ip_re.captures(line) {
            ip = Some(usize::from_str(&caps[1]).unwrap());
        }
        if let Some(caps) = inst_re.captures(line) {
            insts.push((get_op(&caps[1]), (usize::from_str(&caps[2]).unwrap(), usize::from_str(&caps[3]).unwrap(), usize::from_str(&caps[4]).unwrap())))
        }
    }
    (ip.unwrap(), insts)
}

fn get_op(name : &str) -> OpFn {
    match name {
        "addr" => addr,
        "addi" => addi,
        "mulr" => mulr,
        "muli" => muli,
        "banr" => banr,
        "bani" => bani,
        "borr" => borr,
        "bori" => bori,
        "setr" => setr,
        "seti" => seti,
        "gtrr" => gtrr,
        "gtri" => gtri,
        "gtir" => gtir,
        "eqrr" => eqrr,
        "eqri" => eqri,
        "eqir" => eqir,
        _ => panic!(),
    }
}

fn addr(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = regs[args.0] + regs[args.1];
}
fn addi(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = regs[args.0] + args.1;
}

fn mulr(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = regs[args.0] * regs[args.1];
}
fn muli(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = regs[args.0] * args.1;
}

fn banr(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = regs[args.0] & regs[args.1];
}
fn bani(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = regs[args.0] & args.1;
}

fn borr(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = regs[args.0] | regs[args.1];
}
fn bori(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = regs[args.0] | args.1;
}

fn setr(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = regs[args.0];
}
fn seti(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = args.0;
}

fn gtrr(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = if regs[args.0] > regs[args.1] { 1 } else { 0 };
}
fn gtir(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = if args.0 > regs[args.1] { 1 } else { 0 };
}
fn gtri(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = if regs[args.0] > args.1 { 1 } else { 0 };
}

fn eqrr(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = if regs[args.0] == regs[args.1] { 1 } else { 0 };
}
fn eqir(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = if args.0 == regs[args.1] { 1 } else { 0 };
}
fn eqri(args : (usize, usize, usize), regs : &mut Vec<usize>) {
    regs[args.2] = if regs[args.0] == args.1 { 1 } else { 0 };
}

type OpFn = fn((usize, usize, usize), &mut Vec<usize>) -> ();