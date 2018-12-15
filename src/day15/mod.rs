use std::collections::HashSet;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use core::fmt::Write;

pub fn step1(input : String) {
    let mut map = parse_map(&input);
    let mut i = 0;
    loop {
        println!("{}", i);
        //println!("{}", map);
        match map.run_round() {
            Ok(_) => i += 1,
            Err(_) => break,
        }
    }
    let hp_sum : i32 = map.units.iter().map(|u| u.hp as i32).sum();
    println!("{}", i * hp_sum);
}

pub fn step2(input : String) {
    let init_map = parse_map(&input);
    let n_elves = init_map.units.iter().filter(|u| u.kind == UnitKind::Elf).count();
    let mut elf_power = 4;
    let outcome = loop {
        println!("elf power = {}", elf_power);
        let mut map = init_map.clone();
        map.elf_power = elf_power;
        let mut i = 0;
        loop {
            match map.run_round() {
                Ok(_) => i += 1,
                Err(_) => break,
            }
        }
        if map.units.iter().filter(|u| u.kind == UnitKind::Elf).count() == n_elves {
            println!("{}", map);
            let hp_sum : i32 = map.units.iter().map(|u| u.hp as i32).sum();
            break i * hp_sum;
        }
        elf_power += 1;
    };

    println!("outcome = {}", outcome);

}


#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct Coord {
    y : u8,
    x : u8,
}

impl Coord {
    fn new(x : u8, y : u8) -> Coord {
        Coord {y, x}
    }

    fn adjacents(&self) -> Vec<Coord> {
        let mut adj = Vec::new();
        if self.y > 0 {
            adj.push(Coord::new(self.x, self.y - 1));
        }
        if self.x > 0 {
            adj.push(Coord::new(self.x - 1, self.y));
        }
        adj.push(Coord::new(self.x + 1, self.y));
        adj.push(Coord::new(self.x, self.y + 1));

        adj
    }
}

type Walls = HashSet<Coord>;

#[derive(Eq, PartialEq, Copy, Clone)]
enum UnitKind {
    Elf,
    Goblin,
}

impl UnitKind {
    fn other(&self) -> UnitKind {
        match self {
            UnitKind::Goblin => UnitKind::Elf,
            UnitKind::Elf => UnitKind::Goblin,
        }
    }
}

#[derive(Clone)]
struct Unit {
    kind: UnitKind,
    pos : Coord,
    hp : u8,
}

impl Unit {
    fn new(kind : UnitKind, pos : Coord) -> Unit {
        Unit {
            kind, pos,
            hp : 200,
        }
    }
}

#[derive(Clone)]
struct Map {
    walls : Walls,
    units : Vec<Unit>,
    elf_power : u8,
}

enum ShortestPathMove {
    Arrived,
    NoMove,
    MoveTo(Coord),
}

impl Map {
    fn run_round(&mut self) -> Result<(), ()> {
        self.units.sort_by_key(|u| u.pos);
        let mut i = 0;
        while i < self.units.len() {
            let other_kind = self.units[i].kind.other();
            if self.units.iter().filter(|u| u.kind == other_kind).count() == 0 {
                return Err(());
            }
            // move
            match self.shortest_path_move(self.units[i].pos, other_kind) {
                ShortestPathMove::Arrived => (),
                ShortestPathMove::MoveTo(new_coord) => self.units[i].pos = new_coord,
                ShortestPathMove::NoMove => (),
            }

            // attack
            let mut adj_targets: Vec<usize> = self.units[i].pos.adjacents().iter().filter_map(
                |a| self.units.iter().enumerate().filter_map(|(i, u)| {
                    if u.pos == *a && u.kind == other_kind {
                        Some(i)
                    } else {
                        None
                    }
                }).take(1).last()
            ).collect();
            adj_targets.sort_by_key(|j| (self.units[*j].hp, self.units[*j].pos));
            if let Some(j) = adj_targets.get(0) {
                let power = match self.units[i].kind {
                    UnitKind::Goblin => 3,
                    UnitKind::Elf => self.elf_power,
                };
                if self.units[*j].hp > power {
                    self.units[*j].hp -= power;
                } else {
                    self.units.remove(*j);
                    if *j < i {
                        i -= 1;
                    }
                }
            }

            i += 1;
        }
        Ok(())
    }

    fn shortest_path_move(&self, from : Coord, to : UnitKind) -> ShortestPathMove {
        if from.adjacents().iter().any(|a| self.units.iter().any(|u| u.kind == to && u.pos == *a)) {
            return ShortestPathMove::Arrived;
        }

        let mut seen_coords = HashSet::new();
        seen_coords.insert(from);
        let mut paths = Vec::new();
        paths.push((vec![from], false));
        loop {
            let mut new_paths = Vec::new();
            for (outer_path, _) in paths.iter() {
                for c in outer_path.last().unwrap().adjacents() {
                    if !seen_coords.contains(&c) && self.is_free(&c) {
                        let adj_target = c.adjacents().iter().any(|a| self.units.iter().any(|u| u.kind == to && u.pos == *a));
                        let mut p = outer_path.clone();
                        p.push(c);
                        new_paths.push((p, adj_target));
                        seen_coords.insert(c);
                    }
                }
            }
            paths = new_paths;
            if paths.len() == 0 || paths.iter().any(|(_, f)| *f) {
                break;
            }
        }
        if paths.len() == 0 {
            return ShortestPathMove::NoMove;
        }
        let coords : Vec<Coord> = paths.iter().filter(|(_, f)| *f).map(|(p, _)| p[1]).collect();
        ShortestPathMove::MoveTo(coords[0])
    }

    fn is_free(&self, c : &Coord) -> bool {
        !self.walls.contains(c) && self.unit_at_pos(c).is_none()
    }

    fn unit_at_pos(&self, pos : &Coord) -> Option<&Unit> {
        self.units.iter().filter(|u| u.pos == *pos).last()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let min = self.walls.iter().min().unwrap();
        let max = self.walls.iter().max().unwrap();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let c = Coord::new(x, y);
                if self.walls.contains(&c) {
                    f.write_char('#').unwrap();
                } else {
                    if let Some(u) = self.unit_at_pos(&c) {
                        match u.kind {
                            UnitKind::Elf => f.write_char('E').unwrap(),
                            UnitKind::Goblin => f.write_char('G').unwrap(),
                        }
                    } else {
                        f.write_char('.').unwrap()
                    }
                }
            }
            f.write_char('\n').unwrap();
        }

        Ok(())
    }
}

fn parse_map(input : &str) -> Map {
    let mut map = Map {
        walls: HashSet::new(),
        units: Vec::new(),
        elf_power: 3,
    };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord::new(x as u8, y as u8);
            match c {
                '#' => {map.walls.insert(coord); },
                'E' => map.units.push(Unit::new(UnitKind::Elf, coord)),
                'G' => map.units.push(Unit::new(UnitKind::Goblin, coord)),
                _ => (),
            }
        }
    }

    map
}