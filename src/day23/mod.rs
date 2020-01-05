use regex::Regex;
use std::str::FromStr;

pub fn step1(input : String) {
    let nanobots = parse_input(input);
    let strongest = nanobots.iter().max_by_key(|n| n.radius).unwrap();
    let in_radius = nanobots.iter().filter(|n| n.pos.dist(&strongest.pos) <= strongest.radius).count();
    println!("strongest : {}, in radius : {}", strongest.id, in_radius);
}

pub fn step2(input : String) {
    let nanobots = parse_input(input);
    let nb = nanobots.len();

    let (first, others) = nanobots.split_first().unwrap();

    let mut pyramids = vec![(Pyramid::from_center(&first.pos, first.radius), 0u64)];

    for n in others {
        let p = Pyramid::from_center(&n.pos, n.radius);
        for (p0, nb) in pyramids.clone() {
            if let Some(intersect) = p.intersect(&p0) {
                pyramids.push((intersect, nb + 1));
            }
        }
        pyramids.sort_by(|p0, p1| p0.1.cmp(&p1.1).reverse());
        pyramids.truncate(nb * 10);
    }

    pyramids.sort_by(|p0, p1| p0.1.cmp(&p1.1).reverse());

    println!("{:?}", pyramids[0]);
    let point = Coord{
        x: (pyramids[0].0.ppn.min + pyramids[0].0.pnp.min) / 2,
        y: (pyramids[0].0.ppn.min + pyramids[0].0.npp.min) / 2,
        z: (pyramids[0].0.pnp.min + pyramids[0].0.npp.min) / 2,
    };
    println!("{:?}", point);
    println!("{}", point.dist(&Coord{x:0, y:0, z:0}));
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

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug)]
struct MinMax {
    min: i64,
    max: i64,
}

impl MinMax {
    fn intersect(&self, other : &MinMax) -> Option<MinMax> {
        let min = self.min.max(other.min);
        let max = self.max.min(other.max);
        if min <= max {
            Some(MinMax {min, max})
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Pyramid {
    ppp: MinMax,
    ppn: MinMax,
    pnp: MinMax,
    npp: MinMax,
}

impl Pyramid {
    pub fn from_center(center : &Coord, radius: i64) -> Pyramid {
        Pyramid {
            ppp: MinMax {min: center.x + center.y + center.z - radius, max: center.x + center.y + center.z + radius},
            ppn: MinMax {min: center.x + center.y - center.z - radius, max: center.x + center.y - center.z + radius},
            pnp: MinMax {min: center.x - center.y + center.z - radius, max: center.x - center.y + center.z + radius},
            npp: MinMax {min: -center.x + center.y + center.z - radius, max: -center.x + center.y + center.z + radius},
        }
    }

    pub fn intersect(&self, other : &Pyramid) -> Option<Pyramid> {
        match (self.ppp.intersect(&other.ppp), self.ppn.intersect(&other.ppn), self.pnp.intersect(&other.pnp), self.npp.intersect(&other.npp)) {
            (Some(ppp), Some(ppn), Some(pnp), Some(npp)) => Some(Pyramid {ppp, ppn, pnp, npp}),
            _ => None,
        }
    }
}