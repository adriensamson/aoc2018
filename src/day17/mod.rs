use std::collections::HashMap;
use regex::Regex;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use core::fmt::Write;
use std::str::FromStr;

pub fn step1(input : String) {
    let mut map = parse_veins(&input);
    map.do_flow();
    println!("{}", map);
    println!("{}", map.hm.iter().filter(|(k, v)| map.min_y <= k.1 && k.1 <= map.max_y && match v {State::WaterStill | State::WaterFlowing => true, _ => false}).count());
}

pub fn step2(input : String) {
    let mut map = parse_veins(&input);
    map.do_flow();
    println!("{}", map);
    println!("{}", map.hm.iter().filter(|(k, v)| map.min_y <= k.1 && k.1 <= map.max_y && match v {State::WaterStill => true, _ => false}).count());
}

#[derive(Debug)]
enum State {
    Clay,
    WaterFlowing,
    WaterStill,
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn next(&self, c : (u32, u32)) -> (u32, u32) {
        match self {
            Direction::Left => (c.0 - 1, c.1),
            Direction::Right => (c.0 + 1, c.1),
        }
    }
}

struct Map {
    hm: HashMap<(u32, u32), State>,
    min_y: u32,
    max_y: u32,
}

impl Map {
    fn from_map(hm : HashMap<(u32, u32), State>) -> Self {
        Map {
            min_y: hm.keys().map(|k| k.1).min().unwrap(),
            max_y: hm.keys().map(|k| k.1).max().unwrap(),
            hm,
        }
    }

    fn do_flow(&mut self) {
        self.do_flow_full((500, 0));
    }

    fn do_flow_full(&mut self, from : (u32, u32)) -> bool {
        if from.1 >= self.max_y() {
            return false;
        }
        if self.do_flow_under(from) {
            match (self.do_flow_horiz(from, Direction::Left), self.do_flow_horiz(from, Direction::Right)) {
                (Some(l), Some(r)) => {
                    for x in l..=r {
                        self.hm.insert((x, from.1), State::WaterStill);
                    }
                    true
                },
                _ => false,
            }
        } else {
            false
        }
    }

    fn do_flow_under(&mut self, from : (u32, u32)) -> bool {
        if from.1 >= self.max_y() {
            return false;
        }
        let under = (from.0, from.1 + 1);
        match self.hm.get(&under) {
            None => {
                self.hm.insert(under, State::WaterFlowing);
                self.do_flow_full(under)
            },
            Some(State::Clay) | Some(State::WaterStill) => true,
            _ => false,
        }
    }

    fn do_flow_horiz(&mut self, from : (u32, u32), dir : Direction) -> Option<u32> {
        let mut current = (from.0, from.1);
        loop {
            self.hm.insert(current, State::WaterFlowing);
            match self.hm.get(&dir.next(current)) {
                None => {
                    if !self.do_flow_under(current) {
                        return None;
                    }
                },
                Some(State::Clay) => {
                    return Some(current.0);
                },
                _ => (),
            }
            current = dir.next(current);
        }

    }

    fn max_y(&self) -> u32 {
        self.hm.keys().map(|k| k.1).max().unwrap()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let min_x = self.hm.keys().map(|k| k.0).min().unwrap();
        let max_x = self.hm.keys().map(|k| k.0).max().unwrap();
        for y in self.min_y..=self.max_y {
            for x in min_x..=max_x {
                let c = match self.hm.get(&(x, y)) {
                    None => '.',
                    Some(State::Clay) => '#',
                    Some(State::WaterFlowing) => '|',
                    Some(State::WaterStill) => '~',
                };
                f.write_char(c).unwrap();
            }
            f.write_char('\n').unwrap();
        }

        Ok(())
    }
}

fn parse_veins(input : &str) -> Map {
    let mut map = HashMap::new();
    let h_re = Regex::new(r"y=(\d+), x=(\d+)\.\.(\d+)").unwrap();
    let v_re = Regex::new(r"x=(\d+), y=(\d+)\.\.(\d+)").unwrap();

    for line in input.lines() {
        if let Some(caps) = h_re.captures(line) {
            let y = u32::from_str(&caps[1]).unwrap();
            let x_from = u32::from_str(&caps[2]).unwrap();
            let x_to = u32::from_str(&caps[3]).unwrap();
            for x in x_from..=x_to {
                map.insert((x, y), State::Clay);
            }
        }
        if let Some(caps) = v_re.captures(line) {
            let x = u32::from_str(&caps[1]).unwrap();
            let y_from = u32::from_str(&caps[2]).unwrap();
            let y_to = u32::from_str(&caps[3]).unwrap();
            for y in y_from..=y_to {
                map.insert((x, y), State::Clay);
            }
        }
    }

    Map::from_map(map)
}