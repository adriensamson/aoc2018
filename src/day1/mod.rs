use std::fs::read_to_string;
use std::str::FromStr;
use std::collections::HashSet;

pub fn step1() {
    let input = read_to_string("src/day1/input.txt").unwrap();

    let sum = input.lines().map(parse_int).fold(0i64, |acc, i| acc + i);
    println!("{}", sum);
}

fn parse_int(s: &str) -> i64 {
    if &s[0..1] == "+" {
        return i64::from_str(&s[1..]).unwrap()
    }
    i64::from_str(s).unwrap()
}

pub fn step2() {
    let input = read_to_string("src/day1/input.txt").unwrap();

    let mut reached = HashSet::new();

    let numbers : Vec<i64> = input.lines().map(parse_int).collect();

    let mut sum = 0i64;
    reached.insert(sum);
    let mut i = 0usize;

    loop {
        sum += numbers[i];
        if reached.contains(&sum) {
            println!("{}", sum);
            break;
        }
        reached.insert(sum);
        i += 1;
        if i >= numbers.len() {
            i = 0;
        }
    }
}