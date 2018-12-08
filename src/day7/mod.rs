use std::collections::HashMap;
use regex::Regex;

pub fn step1(input : String) {
    let mut map = parse_input(input);
    let order = find_order(&mut map);
    println!("{}", order);
}

pub fn step2(input : String) {
    let mut map = parse_input(input);
    let time = find_order_timed(&mut map);
    println!("{}", time);
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

const STEP_TIME : i32 = 60;
const WORKERS : usize = 5;

fn find_order_timed(map : &mut HashMap<char, Step>) -> i32 {
    let mut s = String::new();
    let mut running : Vec<(char, i32)> = Vec::new();
    let mut time = 0;
    loop {
        println!("time = {}", time);
        for r in &running {
            if r.1 <= time {
                map.get_mut(&r.0).unwrap().mark_done();
                s.push(r.0);
                println!("finished: {} -> {}", r.0, s);
            }
        }
        running = running.iter().filter(|r| r.1 > time).map(|r| *r).collect();
        for av in find_available_timed(map, &running.iter().map(|(c, _)| *c).collect()) {
            if running.len() >= WORKERS {
                println!("not enough workers");
                break;
            }
            println!("start {} (depends on {:?}) until {}", av, map.get(&av).unwrap().reqs, time + STEP_TIME + char_to_time(av));
            running.push((av, time + STEP_TIME + char_to_time(av)));
        }
        println!("running : {:?}", running);
        if running.len() == 0 {
            return time;
        }
        time += 1;
    }
}

fn char_to_time(c : char) -> i32 {
    c as i32 - b'A' as i32 + 1
}

fn find_available_timed(map : &HashMap<char, Step>, running: &Vec<char>) -> Vec<char> {
    let mut available : Vec<&Step> = map.values().filter(|s| !running.iter().any(|&c| c == s.name) && !s.done && s.available(map)).collect();
    if available.len() == 0 {
        return Vec::new();
    }
    available.sort();
    available.iter().map(|s| s.name).collect()
}