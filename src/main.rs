extern crate regex;
extern crate core;

use std::env;
use std::str::FromStr;
use std::fs::read_to_string;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

const DEFAULT_DAY : i32 = 21;
const DEFAULT_STEP : i32 = 1;
const DEFAULT_FILE : &str = "input";

fn main() {
    let args : Vec<String> = env::args().collect();
    let day = args.get(1). map(parse_int).unwrap_or(DEFAULT_DAY);
    let step = args.get(2).map(parse_int).unwrap_or(DEFAULT_STEP);

    let filename = format!("src/day{}/{}.txt", day, args.get(3).map(|s| s.as_ref()).unwrap_or(DEFAULT_FILE));
    let input = read_to_string(filename).unwrap();

    match (day, step) {
        (1, 1) => day1::step1(input),
        (1, 2) => day1::step2(input),
        (2, 1) => day2::step1(input),
        (2, 2) => day2::step2(input),
        (3, 1) => day3::step1(input),
        (3, 2) => day3::step2(input),
        (4, 1) => day4::step1(input),
        (4, 2) => day4::step2(input),
        (5, 1) => day5::step1(input),
        (5, 2) => day5::step2(input),
        (6, 1) => day6::step1(input),
        (6, 2) => day6::step2(input),
        (7, 1) => day7::step1(input),
        (7, 2) => day7::step2(input),
        (8, 1) => day8::step1(input),
        (8, 2) => day8::step2(input),
        (9, 1) => day9::step1(input),
        (9, 2) => day9::step2(input),
        (10, 1) => day10::step1(input),
        (10, 2) => day10::step2(input),
        (11, 1) => day11::step1(input),
        (11, 2) => day11::step2(input),
        (12, 1) => day12::step1(input),
        (12, 2) => day12::step2(input),
        (13, 1) => day13::step1(input),
        (13, 2) => day13::step2(input),
        (14, 1) => day14::step1(input),
        (14, 2) => day14::step2(input),
        (15, 1) => day15::step1(input),
        (15, 2) => day15::step2(input),
        (16, 1) => day16::step1(input),
        (16, 2) => day16::step2(input),
        (17, 1) => day17::step1(input),
        (17, 2) => day17::step2(input),
        (18, 1) => day18::step1(input),
        (18, 2) => day18::step2(input),
        (19, 1) => day19::step1(input),
        (19, 2) => day19::step2(input),
        (20, 1) => day20::step1(input),
        (20, 2) => day20::step2(input),
        (21, 1) => day21::step1(input),
        (21, 2) => day21::step2(input),
        (22, 1) => day22::step1(input),
        (22, 2) => day22::step2(input),
        (23, 1) => day23::step1(input),
        (23, 2) => day23::step2(input),
        (24, 1) => day24::step1(input),
        (24, 2) => day24::step2(input),
        (25, 1) => day25::step1(input),
        _ => println!("Unknown day or step"),
    }
}

fn parse_int(s : &String) -> i32 {
    i32::from_str(s).unwrap()
}