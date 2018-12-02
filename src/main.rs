use std::env;
use std::str::FromStr;

mod day1;
mod day2;

const DEFAULT_DAY : i32 = 2;
const DEFAULT_STEP : i32 = 2;

fn main() {
    let args : Vec<String> = env::args().collect();
    match (args.get(1). map(parse_int).unwrap_or(DEFAULT_DAY), args.get(2).map(parse_int).unwrap_or(DEFAULT_STEP)) {
        (1, 1) => day1::step1(),
        (1, 2) => day1::step2(),
        (2, 1) => day2::step1(),
        (2, 2) => day2::step2(),
        _ => println!("Unknown day or step"),
    }
}

fn parse_int(s : &String) -> i32 {
    i32::from_str(s).unwrap()
}