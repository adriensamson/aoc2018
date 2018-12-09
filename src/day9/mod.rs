use regex::Regex;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use core::fmt::Write;
use std::collections::VecDeque;

pub fn step1(input : String) {
    let (players, last_marble) = get_config(&input);
    play(players, last_marble);
}

pub fn step2(input : String) {
    let (players, last_marble) = get_config(&input);
    play(players, last_marble * 100);
}

fn play(players: usize, last_marble : usize) {
    let mut scores : Vec<usize> = (0..players).map(|_| 0).collect();
    let mut circle = Circle::new();
    for m in 1..=last_marble {
        scores[m % players] += circle.next(m);
        //println!("[{}] {}", m % players, circle);
    }
    let max = scores.iter().fold(0, |acc, o| acc.max(*o));
    println!("max = {}", max);
}

fn get_config(s : &str) -> (usize, usize) {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let cap = re.captures(s).unwrap();
    (usize::from_str(&cap[1]).unwrap(), usize::from_str(&cap[2]).unwrap())
}

struct Circle {
    before : VecDeque<usize>,
    after : VecDeque<usize>,
    current : usize,
}

impl Circle {
    fn new() -> Circle {
        Circle {
            before: VecDeque::new(),
            after: VecDeque::new(),
            current: 0,
        }
    }

    fn do_insert(&mut self, m : usize) {
        if self.after.len() >= 1 {
            self.before.push_back(self.current);
            self.before.push_back(self.after.pop_front().unwrap());
            self.current = m;
        } else {
            self.before.push_back(self.current);
            self.after.push_front(self.before.pop_front().unwrap());
            std::mem::swap(&mut self.before, &mut self.after);
            self.current = m;
        }
    }

    fn do_remove(&mut self) -> usize {
        //println!("{}", self);
        if self.before.len() >= 7 {
            self.after.push_front(self.current);
            for _ in 0..5 {
                self.after.push_front(self.before.pop_back().unwrap());

            }
            self.current = self.before.pop_back().unwrap();
            self.before.pop_back().unwrap()
        } else {
            let len = self.before.len();
            self.after.push_front(self.current);
            for _ in 0..len {
                self.after.push_front(self.before.pop_back().unwrap());
            }
            std::mem::swap(&mut self.before, &mut self.after);
            for _ in len..5 {
                self.after.push_front(self.before.pop_back().unwrap());
            }
            self.current = self.before.pop_back().unwrap();
            self.before.pop_back().unwrap()
        }
    }

    fn next(&mut self, m : usize) -> usize {
        if m % 23 == 0 {
            let removed = self.do_remove();
            println!("removed: {}", removed);
            m + removed
        } else {
            self.do_insert(m); 0
        }
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let s = format!("{:?} *{}* {:?}", self.before, self.current, self.after);
        f.write_str(&s)
    }
}