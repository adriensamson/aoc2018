use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::collections::HashSet;

pub fn step1(input : String) {
    let (depth, target) = parse_input(input);
    let mut map = Map::new(depth, target);
    let mut risk_lvl = 0;
    for y in 0..=target.1 {
        for x in 0..=target.0 {
            risk_lvl += match map.get_state(x, y) {
                State::Rocky => 0,
                State::Wet => 1,
                State::Narrow => 2,
            }
        }
    }
    println!("{}", risk_lvl);
}

pub fn step2(input : String) {
    let (depth, target) = parse_input(input);
    let mut map = Map::new(depth, target);

    println!("{}", find_shortest_path(&mut map));
}

fn parse_input(input : String) -> (usize, (usize, usize)) {
    let mut depth = 0;
    let mut target = (0, 0);
    let depth_re = Regex::new(r"depth: (\d+)").unwrap();
    let target_re = Regex::new(r"target: (\d+),(\d+)").unwrap();
    for line in input.lines() {
        if let Some(caps) = depth_re.captures(line) {
            depth = usize::from_str(&caps[1]).unwrap();
        }
        if let Some(caps) = target_re.captures(line) {
            target = (usize::from_str(&caps[1]).unwrap(), usize::from_str(&caps[2]).unwrap());
        }
    }
    (depth, target)
}

#[derive(Copy, Clone)]
enum State {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
enum Equipment {
    Torch,
    Climbing,
    Neither,
}

struct Map {
    depth: usize,
    target: (usize, usize),
    geo_idx : HashMap<(usize, usize), usize>,
    ero_lvl : HashMap<(usize, usize), usize>,
    states : HashMap<(usize, usize), State>,
}

impl Map {
    fn new(depth : usize, target: (usize, usize)) -> Map {
        Map {depth, target, geo_idx: HashMap::new(), ero_lvl: HashMap::new(), states: HashMap::new()}
    }

    fn get_state(&mut self, x : usize, y : usize) -> State {
        if let Some(state) = self.states.get(&(x, y)) {
            return *state;
        }
        self.compute_state(x, y)
    }

    fn compute_state(&mut self, x : usize, y : usize) -> State {
        let state = match self.get_ero_lvl(x, y) % 3 {
            0 => State::Rocky,
            1 => State::Wet,
            2 => State::Narrow,
            _ => panic!(),
        };
        self.states.insert((x, y), state);
        state
    }

    fn get_geo_idx(&mut self, x : usize, y : usize) -> usize {
        if let Some(idx) = self.geo_idx.get(&(x, y)) {
            return *idx;
        }
        self.compute_geo_idx(x, y)
    }

    fn compute_geo_idx(&mut self, x : usize, y : usize) -> usize {
        let idx = match (x, y) {
            (0, 0) => 0,
            (x, y) if (x, y) == self.target => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => self.get_ero_lvl(x - 1, y) * self.get_ero_lvl(x, y - 1),
        };
        self.geo_idx.insert((x, y), idx);
        idx
    }

    fn get_ero_lvl(&mut self, x : usize, y : usize) -> usize {
        if let Some(lvl) = self.ero_lvl.get(&(x, y)) {
            return *lvl;
        }
        self.compute_ero_lvl(x, y)
    }

    fn compute_ero_lvl(&mut self, x : usize, y : usize) -> usize {
        let lvl = (self.get_geo_idx(x, y) + self.depth) % 20183;
        self.ero_lvl.insert((x, y), lvl);
        lvl
    }
}

fn is_equipment_valid(e : Equipment, s : State) -> bool {
    match (e, s) {
        (Equipment::Climbing, State::Narrow) => false,
        (Equipment::Torch, State::Wet) => false,
        (Equipment::Neither, State::Rocky) => false,
        _ => true,
    }
}

fn find_shortest_path(map : &mut Map) -> usize {
    let mut seen_pos = HashSet::new();
    let mut current_pos = Vec::new();
    current_pos.push(((0usize, 0usize, Equipment::Torch), 0usize));
    loop {
        //current_pos.sort_by_key(|(_, d)| -(*d as i64));
        let pos = current_pos.pop().unwrap();
        seen_pos.insert(pos.0);
        let mut new_pos = Vec::new();
        for next in next_pos(map, pos) {
            if next.0 == (map.target.0, map.target.1, Equipment::Torch) {
                return next.1;
            }
            if !seen_pos.contains(&next.0) {
                new_pos.push(next);
            }
        }
        new_pos.sort();
        new_pos.dedup_by_key(|(p, _)| *p);
        for next in new_pos {
            if let Some((i, d0)) = current_pos.iter().enumerate().filter(|(_, (p, _))| *p == next.0).map(|(i, (_, d0))| (i, *d0)).last() {
                if next.1 < d0 {
                    current_pos.remove(i);
                } else {
                    continue;
                }
            }
            match current_pos.binary_search_by_key(&-(next.1 as i64), |(_, d)| -(*d as i64)) {
                Ok(i) => current_pos.insert(i, next),
                Err(i) => current_pos.insert(i, next),
            }
        }
    }
}

fn next_pos(map : &mut Map, ((x, y, e), d) : ((usize, usize, Equipment), usize)) -> Vec<((usize, usize, Equipment), usize)> {
    let mut nexts = Vec::new();
    if x > 0 && is_equipment_valid(e, map.get_state(x - 1, y)) {
        nexts.push(((x - 1, y, e), d + 1));
    }
    if is_equipment_valid(e, map.get_state(x + 1, y)) {
        nexts.push(((x + 1, y, e), d + 1));
    }
    if y > 0 && is_equipment_valid(e, map.get_state(x, y - 1)) {
        nexts.push(((x, y - 1, e), d + 1));
    }
    if is_equipment_valid(e, map.get_state(x, y + 1)) {
        nexts.push(((x, y + 1, e), d + 1));
    }
    for ne in &[Equipment::Torch, Equipment::Climbing, Equipment::Neither] {
        if *ne != e && is_equipment_valid(*ne, map.get_state(x, y)) {
            nexts.push(((x, y, *ne), d + 7));
        }
    }
    nexts
}