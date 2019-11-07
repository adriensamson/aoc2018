use std::str::FromStr;

pub fn step1(input : String) {
    let points = parse_input(&input);

    let mut constellations: Vec<Vec<&Point>> = points.iter().map(|p| vec!{p}).collect();
    let mut nb = constellations.len();

    loop {
        constellations = merge_constellations(constellations);
        if constellations.len() == nb {
            break;
        }
        nb = constellations.len();
    }

    println!("{}", nb);
}

#[derive(Debug)]
struct Point {
    x0: i64,
    x1: i64,
    x2: i64,
    x3: i64,
}

impl Point {
    pub fn dist(&self, other : &Point) -> i64 {
        return (self.x0 - other.x0).abs()
            + (self.x1 - other.x1).abs()
            + (self.x2 - other.x2).abs()
            + (self.x3 - other.x3).abs();
    }
}

fn constellations_dist(const1 : &Vec<&Point>, const2 : &Vec<&Point>) -> i64 {
    const1.iter().flat_map(|p1| const2.iter().map(move |p2| p1.dist(p2))).min().unwrap()
}

fn merge_constellations(consts : Vec<Vec<&Point>>) -> Vec<Vec<&Point>> {
    let mut new_consts = Vec::new();

    for c2 in consts {
        let mut merged = false;
        for c1 in &mut new_consts {
            if constellations_dist(c1, &c2) <= 3 {
                c1.append(&mut c2.clone());
                merged = true;
                break;
            }
        }
        if !merged {
            new_consts.push(c2.clone());
        }
    }

    return new_consts;
}

fn parse_input(input : &str) -> Vec<Point> {
    let mut points = Vec::new();

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }
        let coords : Vec<i64> = line.split(",").map(|s| i64::from_str(s).unwrap()).collect();
        if coords.len() == 4 {
            points.push(Point {
                x0: coords[0],
                x1: coords[1],
                x2: coords[2],
                x3: coords[3],
            });
        }
    }

    return points;
}