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

    let (first, others) = nanobots.split_first().unwrap();

    let mut pyramids = vec![(Pyramid::from_center(&first.pos, first.radius), 0u64)];

    for n in others {
        let p = Pyramid::from_center(&n.pos, n.radius);
        let mut new_pyramids = vec![];
        for (p0, nb) in pyramids {
            let (boths, ones) = p0.intersect(&p);
            for both in boths {
                new_pyramids.push((both, nb + 1));
            }
            for one in ones {
                new_pyramids.push((one, nb));
            }
        }
        pyramids = new_pyramids;
        println!("{}", pyramids.len());
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
    fn includes(&self, value : i64) -> bool {
        self.min <= value && value <= self.max
    }

    fn combine(&self, other : &MinMax) -> Vec<MinMax> {
        if other.max < self.min || self.max < other.min {
            return vec![self.clone(), other.clone()];
        }
        let mut vals = vec![self.min, self.max, other.min, other.max];
        vals.sort();
        let mut res = vec![];
        if vals[0] < vals[1] {
            res.push(MinMax{min: vals[0], max: vals[1]})
        }
        res.push(MinMax{min: vals[1], max: vals[2]});
        if vals[2] < vals[3] {
            res.push(MinMax{min: vals[2], max: vals[3]})
        }
        return res;
    }
}

#[derive(Copy, Clone, Debug)]
struct Pyramid {
//    ppp: MinMax,
    ppn: MinMax,
    pnp: MinMax,
    npp: MinMax,
}

impl Pyramid {
    pub fn from_center(center : &Coord, radius: i64) -> Pyramid {
        Pyramid {
//            ppp: MinMax {min: center.x + center.y + center.z - radius, max: center.x + center.y + center.z + radius},
            ppn: MinMax {min: center.x + center.y - center.z - radius, max: center.x + center.y - center.z + radius},
            pnp: MinMax {min: center.x - center.y + center.z - radius, max: center.x - center.y + center.z + radius},
            npp: MinMax {min: -center.x + center.y + center.z - radius, max: -center.x + center.y + center.z + radius},
        }
    }

    pub fn contains(&self, point : &Coord) -> bool {
//        self.ppp.includes(point.x + point.y + point.z) &&
        self.ppn.includes(point.x + point.y - point.z) &&
        self.pnp.includes(point.x - point.y + point.z) &&
        self.npp.includes(-point.x + point.y + point.z)
    }

    pub fn intersect(&self, other : &Pyramid) -> (Vec<Pyramid>, Vec<Pyramid>) {
//        let ppps = self.ppp.combine(&other.ppp);
        let ppns = self.ppn.combine(&other.ppn);
        let pnps = self.pnp.combine(&other.pnp);
        let npps = self.npp.combine(&other.npp);

        let mut both = vec![];
        let mut one = vec![];

//        for ppp in &ppps {
//            let ppp_self = self.ppp.includes(ppp.min) && self.ppp.includes(ppp.max);
//            let ppp_other = other.ppp.includes(ppp.min) && other.ppp.includes(ppp.max);
            for ppn in &ppns {
                let ppn_self = self.ppn.includes(ppn.min) && self.ppn.includes(ppn.max);
                let ppn_other = other.ppn.includes(ppn.min) && other.ppn.includes(ppn.max);
                for pnp in &pnps {
                    let pnp_self = self.pnp.includes(pnp.min) && self.pnp.includes(pnp.max);
                    let pnp_other = other.pnp.includes(pnp.min) && other.pnp.includes(pnp.max);
                    for npp in &npps {
                        let npp_self = self.npp.includes(npp.min) && self.npp.includes(npp.max);
                        let npp_other = other.npp.includes(npp.min) && other.npp.includes(npp.max);
                        let in_self = /*ppp_self &&*/ ppn_self && pnp_self && npp_self;
                        let in_other = /*ppp_other &&*/ ppn_other && pnp_other && npp_other;
                        if in_self && in_other {
                            both.push(Pyramid{/*ppp: ppp.clone(),*/ ppn: ppn.clone(), pnp: pnp.clone(), npp: npp.clone()});
                        } else if in_self || in_other {
                            one.push(Pyramid{/*ppp: ppp.clone(),*/ ppn: ppn.clone(), pnp: pnp.clone(), npp: npp.clone()});
                        }
                    }
                }
            }
//        }
        return (both, one);
    }
}