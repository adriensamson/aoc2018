use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

pub fn step1(input : String) {
    let nanobots = parse_input(input);
    let strongest = nanobots.iter().max_by_key(|n| n.radius).unwrap();
    let in_radius = nanobots.iter().filter(|n| n.pos.dist(&strongest.pos) <= strongest.radius).count();
    println!("strongest : {}, in radius : {}", strongest.id, in_radius);
}

pub fn step2(input : String) {
    let nanobots = parse_input(input);
    let mut map = HashMap::new();
    for n in nanobots.iter() {
        for d in 0..n.radius {
            for c in n.pos.at_dist(d) {
                *map.entry(c).or_insert(0usize) += 1;
            }
        }
    }
    let max_in_range = map.values().max().unwrap();
    let origin = Coord { x: 0, y: 0, z: 0};
    let better_coord = map.iter().filter(|(_, n)| **n == *max_in_range).map(|(c, _)| c).min_by_key(|c| c.dist(&origin)).unwrap();
    println!("{}", better_coord.dist(&origin));
}

fn parse_input(input : String) -> Vec<Nanobot> {
    let mut vec = Vec::new();
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    for (id, line) in input.lines().enumerate() {
        if let Some(caps) = re.captures(line) {
            vec.push(Nanobot {
                id,
                pos: Coord {
                    x: i64::from_str(&caps[1]).unwrap(),
                    y: i64::from_str(&caps[2]).unwrap(),
                    z: i64::from_str(&caps[3]).unwrap(),
                },
                radius: i64::from_str(&caps[4]).unwrap(),
            });
        }
    }

    vec
}

#[derive(Hash, Copy, Clone, Eq, PartialEq)]
struct Coord {
    x : i64,
    y : i64,
    z : i64,
}

impl Coord {
    fn dist(&self, other : &Self) -> i64 {
        self.x.max(other.x) - self.x.min(other.x)
        + self.y.max(other.y) - self.y.min(other.y)
        + self.z.max(other.z) - self.z.min(other.z)
    }

    fn at_dist(&self, d : i64) -> Vec<Coord> {
        let mut vec = Vec::new();
        for dx in -d..=d {
            let dx_abs = dx.abs();
            for dy in -(d - dx_abs)..=(d - dx_abs) {
                let dz = d - dx_abs - dy.abs();
                vec.push(Coord {x: self.x + dx, y: self.y + dy, z: self.z + dz});
                if -dz != dz {
                    vec.push(Coord {x: self.x + dx, y: self.y + dy, z: self.z - dz});
                }
            }
        }
        vec
    }
}

struct Nanobot {
    id : usize,
    pos: Coord,
    radius : i64,
}
