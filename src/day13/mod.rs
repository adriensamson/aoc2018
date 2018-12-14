use std::collections::HashMap;
use std::cmp::Ordering;

pub fn step1(input : String) {
    let (map, mut carts) = parse_map(&input);
    'big_loop: loop {
        carts.sort_by_key(|c| c.pos);
        for i in 0..carts.len() {
            carts[i].advance(&map);
            for (j, cart2) in carts.iter().enumerate() {
                if i != j && carts[i].pos == cart2.pos {
                    println!("Collision at {},{}", carts[i].pos.0, carts[i].pos.1);
                    break 'big_loop;
                }
            }
        }
    }
}

pub fn step2(input : String) {
    let (map, mut carts) = parse_map(&input);
    loop {
        carts.sort_by(|c1, c2| c1.cmp_pos(c2));
        let mut i = 0;
        while i < carts.len() {
            carts[i].advance(&map);
            let mut j = 0;
            let mut removed = false;
            while j < carts.len() {
                if i != j && carts[i].pos == carts[j].pos {
                    println!("Collision at {},{}", carts[i].pos.0, carts[i].pos.1);
                    if i < j {
                        carts.remove(j);
                        carts.remove(i);
                    } else {
                        carts.remove(i);
                        carts.remove(j);
                        i -= 1;
                    }
                    removed = true;
                }
                j += 1;
            }
            if !removed {
                i += 1;
            }
        }
        if carts.len() == 1 {
            break;
        }
    }
    println!("Last position at {},{}", carts[0].pos.0, carts[0].pos.1);
}

type Coord = (usize, usize);

fn coord_advance(coord : &Coord, dir : &Direction) -> Coord {
    match dir {
        Direction::Top => (coord.0, coord.1 - 1),
        Direction::Bottom => (coord.0, coord.1 + 1),
        Direction::Left => (coord.0 - 1, coord.1),
        Direction::Right => (coord.0 + 1, coord.1),
    }
}

enum Rail {
    Straight,
    Curve,
    CurveAnti,
    Intersection,
}

type Map = HashMap<Coord, Rail>;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

impl Direction {
    fn turn(&self, t : &Turn) -> Direction {
        match (self, t) {
            (_, Turn::Straight) => *self,
            (Direction::Left, Turn::Left) => Direction::Bottom,
            (Direction::Left, Turn::Right) => Direction::Top,
            (Direction::Top, Turn::Left) => Direction::Left,
            (Direction::Top, Turn::Right) => Direction::Right,
            (Direction::Right, Turn::Left) => Direction::Top,
            (Direction::Right, Turn::Right) => Direction::Bottom,
            (Direction::Bottom, Turn::Left) => Direction::Right,
            (Direction::Bottom, Turn::Right) => Direction::Left,
        }
    }

    fn curve(&self) -> Direction {
        match self {
            Direction::Left => Direction::Bottom,
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Top,
            Direction::Bottom => Direction::Left,
        }
    }

    fn curve_anti(&self) -> Direction {
        match self {
            Direction::Left => Direction::Top,
            Direction::Top => Direction::Left,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Right,
        }
    }
}

enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

struct Cart {
    pos : Coord,
    direction : Direction,
    next_turn : Turn,
}

impl Cart {
    fn new(pos : Coord, direction : Direction) -> Cart {
        Cart {
            pos, direction, next_turn: Turn::Left,
        }
    }

    fn advance(&mut self, map : &Map) {
        self.pos = coord_advance(&self.pos, &self.direction);
        match map[&self.pos] {
            Rail::Straight => (),
            Rail::Intersection => {
                self.direction = self.direction.turn(&self.next_turn);
                self.next_turn = self.next_turn.next();
            }
            Rail::Curve => self.direction = self.direction.curve(),
            Rail::CurveAnti => self.direction = self.direction.curve_anti(),
        }
    }

    fn cmp_pos(&self, other : &Self) -> Ordering {
        if self.pos.1 < other.pos.1 {
            Ordering::Less
        } else if self.pos.1 < other.pos.1 {
            Ordering::Greater
        } else if self.pos.0 < other.pos.0 {
            Ordering::Less
        } else if self.pos.0 > other.pos.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn parse_map(s : &str) -> (Map, Vec<Cart>) {
    let mut map = HashMap::new();
    let mut carts = Vec::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '-' | '|' | '^' | 'v' | '<' | '>' => {map.insert((x, y), Rail::Straight);},
                '/' => {map.insert((x, y), Rail::Curve); },
                '\\' => {map.insert((x, y), Rail::CurveAnti); },
                '+' => {map.insert((x, y), Rail::Intersection); },
                _ => (),
            }
            match c {
                '^' => carts.push(Cart::new((x, y), Direction::Top)),
                'v' => carts.push(Cart::new((x, y), Direction::Bottom)),
                '<' => carts.push(Cart::new((x, y), Direction::Left)),
                '>' => carts.push(Cart::new((x, y), Direction::Right)),
                _ => (),
            }
        }
    }

    (map, carts)
}