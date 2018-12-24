use regex::Regex;
use std::str::FromStr;

pub fn step1(input : String) {
    let nanobots = parse_input(input);
    let strongest = nanobots.iter().max_by_key(|n| n.radius).unwrap();
    let in_radius = nanobots.iter().filter(|n| n.pos.dist(&strongest.pos) <= strongest.radius).count();
    println!("strongest : {}, in radius : {}", strongest.id, in_radius);
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
}

struct Nanobot {
    id : usize,
    pos: Coord,
    radius : i64,
}
