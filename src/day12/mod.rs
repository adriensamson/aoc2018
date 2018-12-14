use std::collections::HashMap;
use regex::Regex;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

pub fn step1(input : String) {
    let (initial, rules) = parse_input(&input);
    let mut state = initial;
    println!("0: {} [{}]", state, state.sum());
    for i in 1..=20 {
        state = state.apply_rules(&rules);
        println!("{}: {} [{}]", i, &state, state.sum());
    }
}

struct State {
    offset : i32,
    plants : Vec<bool>,
}

type Rules = HashMap<(bool, bool, bool, bool, bool), bool>;

impl State {
    fn from(s : &str) -> State {
        State {
            offset: 0,
            plants: s.chars().map(|c| c == '#').collect()
        }
    }

    fn apply_rules(&self, rules : &Rules) -> State {
        let mut offset = self.offset - 2;
        let mut plants = Vec::new();

        plants.push(*rules.get(&(false, false, false, false, self.plants[1])).unwrap_or(&false));
        plants.push(*rules.get(&(false, false, false, self.plants[0], self.plants[1])).unwrap_or(&false));
        plants.push(*rules.get(&(false, false, self.plants[0], self.plants[1], self.plants[2])).unwrap_or(&false));
        plants.push(*rules.get(&(false, self.plants[0], self.plants[1], self.plants[2], self.plants[3])).unwrap_or(&false));
        for i in 2..(self.plants.len() - 2) {
            plants.push(*rules.get(&(self.plants[i - 2], self.plants[i - 1], self.plants[i], self.plants[i + 1], self.plants[i + 2])).unwrap_or(&false));
        }
        plants.push(*rules.get(&(self.plants[self.plants.len() - 4], self.plants[self.plants.len() - 3], self.plants[self.plants.len() - 2], self.plants[self.plants.len() - 1], false)).unwrap_or(&false));
        plants.push(*rules.get(&(self.plants[self.plants.len() - 3], self.plants[self.plants.len() - 2], self.plants[self.plants.len() - 1], false, false)).unwrap_or(&false));
        plants.push(*rules.get(&(self.plants[self.plants.len() - 2], self.plants[self.plants.len() - 1], false, false, false)).unwrap_or(&false));
        plants.push(*rules.get(&(self.plants[self.plants.len() - 1], false, false, false, false)).unwrap_or(&false));

        for _i in 0..4 {
            if offset < 0 && !plants[0] {
                offset += 1;
                plants.remove(0);
            }
            if plants.len() > 10 && !plants[plants.len() - 1] {
                plants.pop();
            }
        }


        State {
            offset, plants
        }
    }

    fn sum(&self) -> i32 {
        self.plants.iter().enumerate().filter(|(_, v)| **v).map(|(k, _)| k as i32 + self.offset).sum()
    }
}

impl Display for State {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> Result<(), Error> {
        f.write_str(&format!("[{}] ", self.offset)).unwrap();
        for p in &self.plants {
            f.write_str(if *p { "#" } else { "." }).unwrap();
        }

        Result::Ok(())
    }
}

fn is_sharp(c : &str) -> bool {
    c == "#"
}

fn parse_input(s : &str) -> (State, Rules) {
    let mut initial = None;
    let mut rules = HashMap::new();

    let initial_re = Regex::new(r"initial state: ([#.]+)").unwrap();
    let rule_re = Regex::new(r"([#.])([#.])([#.])([#.])([#.]) => ([#.])").unwrap();

    for line in s.lines() {
        if let Some(caps) = initial_re.captures(line) {
            initial = Some(State::from(&caps[1]));
        }
        if let Some(caps) = rule_re.captures(line) {
            let left = (is_sharp(&caps[1]), is_sharp(&caps[2]), is_sharp(&caps[3]), is_sharp(&caps[4]), is_sharp(&caps[5]));
            rules.insert(left, is_sharp(&caps[6]));
        }
    }

    (initial.unwrap(), rules)
}