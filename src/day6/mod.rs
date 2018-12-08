use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn step1(input : String) {
    let coords = parse_coords(&input);
    let map = draw_all(&coords);
    let largest = find_largest_finished(&map, get_min_max(&coords));
    println!("largest: {}", largest);
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn north_east(&self, dist : usize, t : usize) -> Coord {
        Coord {
            x: self.x + (t as i32),
            y: self.y + (t as i32) - (dist as i32),
        }
    }
    fn south_east(&self, dist : usize, t : usize) -> Coord {
        Coord {
            x: self.x + (dist as i32) - (t as i32),
            y: self.y + (t as i32),
        }
    }
    fn south_west(&self, dist : usize, t : usize) -> Coord {
        Coord {
            x: self.x - (t as i32),
            y: self.y + (dist as i32) - (t as i32),
        }
    }
    fn nort_west(&self, dist : usize, t : usize) -> Coord {
        Coord {
            x: self.x - (dist as i32) + (t as i32),
            y: self.y - (t as i32),
        }
    }

    fn points_at(&self, dist : usize) -> Vec<Coord> {
        let mut v = Vec::new();
        for t in 0..dist {
            v.push(self.north_east(dist, t));
        }
        for t in 0..dist {
            v.push(self.south_east(dist, t));
        }
        for t in 0..dist {
            v.push(self.south_west(dist, t));
        }
        for t in 0..dist {
            v.push(self.nort_west(dist, t));
        }
        v
    }
}

fn parse_coords(input : &str) -> Vec<Coord> {
    let re = Regex::new(r"(\d+), (\d+)").unwrap();
    input.lines().map(|l| {
        let caps = re.captures(l).unwrap();
        Coord {
            x: i32::from_str(&caps[1]).unwrap(),
            y: i32::from_str(&caps[2]).unwrap(),
        }
    }).collect()
}

fn draw(oldmap : &HashMap<Coord, Option<usize>>, coords: &[Coord], dist : usize) -> HashMap<Coord, Option<usize>> {
    let mut newmap = oldmap.clone();
    for (i, center) in coords.iter().enumerate() {
        for c in center.points_at(dist) {
            if oldmap.get(&c).is_some() {
                continue;
            }
            if newmap.get(&c).is_none() {
                newmap.insert(c, Some(i));
            } else {
                newmap.insert(c, None);
            }
        }
    }

    newmap
}

fn get_min_max(coords : &[Coord]) -> (Coord, Coord) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for c in coords.iter() {
        min_x = min_x.min(c.x);
        min_y = min_y.min(c.y);
        max_x = max_x.max(c.x);
        max_y = max_y.max(c.y);
    }

    (Coord {x: min_x, y: min_y}, Coord {x: max_x, y: max_y})
}

fn is_full(map : &HashMap<Coord, Option<usize>>, min_max: (Coord, Coord)) -> bool {
    let (Coord {x: min_x, y: min_y}, Coord {x: max_x, y: max_y}) = min_max;

    let count = map.keys().filter(|Coord{x, y}| min_x <= *x && *x <= max_x && min_y <= *y && *y <= max_y).count();
    println!("{} - {} / {} - {} = {} vs {}", min_x, max_x, min_y, max_y, (1 + max_x - min_x) * (1 + max_y - min_y), count);
    count == (((1 + max_x - min_x) * (1 + max_y - min_y)) as usize)
}

fn draw_all(coords : &[Coord]) -> HashMap<Coord, Option<usize>> {
    let min_max = get_min_max(coords);
    let mut map = HashMap::new();
    for (i, &coord) in coords.iter().enumerate() {
        map.insert(coord, Some(i));
    }

    let mut dist = 0;
    while !is_full(&map, min_max) {
        dist += 1;
        println!("{}", dist);
        map = draw(&map, coords, dist);
    }
    println!("finished");
    map
}

fn find_largest_finished(map : &HashMap<Coord, Option<usize>>, (min, max) : (Coord, Coord)) -> usize {
    let mut nb_map  : HashMap<usize, (usize, bool)>= HashMap::new();
    for (&coord, &who) in map.iter() {
        if let Some(i) = who {
            let mut nb_inf = nb_map.entry(i).or_default();
            nb_inf.0 += 1;
            if coord.x <= min.x || max.x <= coord.x || coord.y <= min.y || max.y <= coord.y {
                nb_inf.1 = true;
            }
        }
    }

    nb_map.iter()
        .filter(|(_, (_, inf))| !*inf)
        .map(|(k, (n, _))| (k, n))
        .fold((None, 0), |acc, (k, n)| {
            if *n > acc.1 {
                (Some(*k), *n)
            } else {
                acc
            }
        }).1
}