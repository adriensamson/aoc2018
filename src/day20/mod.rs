use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn step1(input : String) {
    let (route, rest) = parse_route(&input[1..]);
    if &rest[0..1] != "$" {
        panic!("incomplete route");
    }

    let mut rooms = HashMap::new();
    rooms.insert((0, 0), 0);
    visit(&route, &mut |e, (from, d)| {
        let to = e.apply(from);
        match rooms.entry(to) {
            Entry::Vacant(v) => {
                v.insert(d + 1);
            },
            Entry::Occupied(mut o) => if d + 1 < *o.get() {
                o.insert(d + 1);
            }
        }
        (to, d + 1)
    }, ((0, 0), 0));

    let mut roomsv : Vec<((i32, i32), i32)> = rooms.iter().map(|(k, v)| (*k, *v)).collect();
    roomsv.sort_by_key(|(_, d)| *d);
    println!("{}", roomsv.last().unwrap().1);
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
enum RouteElement {
    Direction(Direction),
    Options(Vec<Route>),
}

impl Direction {
    fn from_str(s : &str) -> Direction {
        match s {
            "N" => Direction::North,
            "E" => Direction::East,
            "S" => Direction::South,
            "W" => Direction::West,
            _ => panic!(),
        }
    }

    fn apply(&self, (x, y) : (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }
}

type Route = Vec<RouteElement>;

fn parse_route(input : &str) -> (Route, &str) {
    let mut rest = input;
    let mut route = Vec::new();
    loop {
        let split = rest.split_at(1);
        let element = match split.0 {
            "N" | "E" | "S" | "W" => {
                rest = split.1;
                RouteElement::Direction(Direction::from_str(split.0))
            },
            "(" => {
                let r = parse_options(split.1);
                rest = r.1;
                RouteElement::Options(r.0)
            },
            _ => break
        };
        route.push(element);
    }
    (route, rest)
}

fn parse_options(input : &str) -> (Vec<Route>, &str) {
    let mut options = Vec::new();
    let mut rest = input;
    loop {
        let split = rest.split_at(1);
        match split.0 {
            "|" => {
                options.push(Vec::new());
                rest = split.1;
            },
            ")" => {
                options.push(Vec::new());
                rest = split.1;
                break;
            },
            _ => {
                let (r, next) = parse_route(rest);
                options.push(r);
                let split = next.split_at(1);
                match split.0 {
                    "|" => rest = split.1,
                    ")" => {
                        rest = split.1;
                        break;
                    },
                    _ => panic!(format!("found {}", split.0)),
                }
            }
        }
    }
    (options, rest)
}

fn visit<I : Copy, F : FnMut(Direction, I) -> I>(route : &[RouteElement], f : &mut F, i : I) {
    let mut acc = i;
    let mut n = 0;
    while n < route.len() {
        match &route[n] {
            RouteElement::Direction(d) => {
                acc = f(*d, acc);
                n += 1;
            },
            RouteElement::Options(opts) => {
                for opt in opts {
                    let mut next = Vec::from(&route[n+1..]);
                    let mut new_route = opt.clone();
                    new_route.append(&mut next);
                    visit(&new_route, f, acc);
                }
                break;
            }
        }
    }

}