use std::collections::HashMap;
use regex::Regex;

pub fn step1(input : String) {
    let mut map = parse_input(input);
    let order = find_order(&mut map);
    println!("{}", order);
}

#[derive(Ord, Eq)]
struct Step {
    name: char,
    reqs: Vec<char>,
    done: bool,
}

impl Step {
    fn new(name : char) -> Step {
        Step {
            name,
            reqs: Vec::new(),
            done: false,
        }
    }

    fn available(&self, map : &HashMap<char, Step>) -> bool {
        self.reqs.iter().all(|s| map.get(s).unwrap().done)
    }

    fn add_req(&mut self, step : char) {
        self.reqs.push(step);
    }

    fn mark_done(&mut self) {
        self.done = true;
    }
}

impl ::std::cmp::PartialEq for Step {
    fn eq(&self, other: &Step) -> bool {
        self.name == other.name
    }
}

impl ::std::cmp::PartialOrd for Step {
    fn partial_cmp(&self, other: &Step) -> Option<::std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

fn first_char(s : &str) -> char {
    let cs : Vec<char> = s.chars().take(1).collect();
    cs[0]
}

fn parse_input(input : String) -> HashMap<char, Step> {
    let mut map : HashMap<char, Step> = HashMap::new();
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let r = first_char(&caps[1]);
            let s = first_char(&caps[2]);

            map.entry(r).or_insert(Step::new(r));

            let step = map.entry(s).or_insert(Step::new(s));
            step.add_req(r);
        }
    }

    map
}

fn find_order(map : &mut HashMap<char, Step>) -> String {
    let mut s = String::new();
    loop {
        match find_first_avaible(map) {
            None => { return s; },
            Some(c) => {
                let mut first = map.get_mut(&c).unwrap();
                first.mark_done();
                s.push(first.name);
            }
        }
    }
}

fn find_first_avaible(map : &HashMap<char, Step>) -> Option<char> {
    let mut available : Vec<&Step> = map.values().filter(|s| !s.done && s.available(map)).collect();
    if available.len() == 0 {
        return None;
    }
    available.sort();
    Some(available[0].name)
}