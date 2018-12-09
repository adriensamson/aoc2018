use regex::Regex;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use core::fmt::Write;

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
    marbles : Vec<usize>,
    current_index : usize,
}

impl Circle {
    fn new() -> Circle {
        Circle {
            marbles: vec![0],
            current_index: 0,
        }
    }

    fn do_insert(&mut self, m : usize) {
        let insert_after_idx = (self.current_index + 1) % self.marbles.len();
        self.marbles.insert(insert_after_idx + 1, m);
        self.current_index = insert_after_idx + 1;
    }

    fn do_remove(&mut self) -> usize {
        let remove_idx = (self.current_index + self.marbles.len() - 7) % self.marbles.len();
        let removed = self.marbles.remove(remove_idx);
        self.current_index = remove_idx % self.marbles.len();
        removed
    }

    fn next(&mut self, m : usize) -> usize {
        if m % 23 == 0 {
            m + self.do_remove()
        } else {
            self.do_insert(m); 0
        }
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for (i, &m) in self.marbles.iter().enumerate() {
            if i == self.current_index {
                f.write_char('*');
            } else {
                f.write_char(' ');
            }
            let s = format!("{}", m);
            f.write_str(&s);
        }

        Result::Ok(())
    }
}