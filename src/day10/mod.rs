use regex::Regex;
use std::str::FromStr;
use std::collections::HashSet;

pub fn step1(input : String) {
    let points = parse_points(&input);
    let mut t = 0;
    let mut coords = get_points_at(&points, 0);
    let mut area = get_area(&coords);
    loop {
        t += 1;
        let new_coords = get_points_at(&points, t);
        let new_area = get_area(&new_coords);
        if new_area > area {
            break;
        }
        coords = new_coords;
        area = new_area;
    }
    display(&coords);
}

struct Point {
    origin: (i64, i64),
    velocity: (i64, i64),
}

impl Point {
    fn pos_at(&self, t : i64) -> (i64, i64) {
        (self.origin.0 + t * self.velocity.0, self.origin.1 + t * self.velocity.1)
    }
}

fn parse_points(input : &str) -> Vec<Point> {
    let re = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
    let parse_line = |s : &str| {
        let caps = re.captures(s).unwrap();
        return Point {
            origin: (i64::from_str(&caps[1]).unwrap(), i64::from_str(&caps[2]).unwrap()),
            velocity: (i64::from_str(&caps[3]).unwrap(), i64::from_str(&caps[4]).unwrap()),
        }
    };
    input.lines().map(parse_line).collect()
}

fn get_points_at(points : &Vec<Point>, t : i64) -> Vec<(i64, i64)> {
    let mut t_points = Vec::new();
    for p in points {
        let t_p = p.pos_at(t);
        t_points.push(t_p);
    }
    t_points
}

fn get_min_max(coords : &Vec<(i64, i64)>) -> ((i64, i64), (i64, i64)) {
    let mut min_x = coords[0].0;
    let mut max_x = coords[0].0;
    let mut min_y = coords[0].1;
    let mut max_y = coords[0].1;
    for t_p in coords {
        min_x = min_x.min(t_p.0);
        max_x = max_x.max(t_p.0);
        min_y = min_y.min(t_p.1);
        max_y = max_y.max(t_p.1);
    }
    ((min_x, min_y), (max_x, max_y))
}

fn get_area(coords : &Vec<(i64, i64)>) -> i64 {
    let ((min_x, min_y), (max_x, max_y)) = get_min_max(coords);
    (max_x - min_x) * (max_y - min_y)
}

fn display(points : &Vec<(i64, i64)>) {
    let ((min_x, min_y), (max_x, max_y)) = get_min_max(points);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}