use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use core::fmt::Write;
use std::collections::HashMap;

pub fn step1(input : String) {
    let mut map = Map::from_str(&input);
    println!("{}", map);
    for i in 1..=10 {
        map = map.tick();
        println!("{}", i);
        println!("{}", map);
    }
    println!("{}", map.resource_value());
}

pub fn step2(input : String) {
    let mut map = Map::from_str(&input);
    let mut seen = HashMap::new();
    seen.insert(map.clone(), 0);
    let mut i = 1;
    let loop_size = loop {
        map = map.tick();
        let j = seen.entry(map.clone()).or_insert(i);
        if i != *j {
            println!("found loop {} -> {}", i, j);
            break *j - i;
        }
        i += 1;
    };
    let remaining = (1000000000 - i) % loop_size;
    for _ in 0..remaining {
        map = map.tick();
    }
    println!("{}", map.resource_value());
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum State {
    Open,
    Trees,
    Lumberyard,
}

impl State {
    fn from_char(c : char) -> State {
        match c {
            '.' => State::Open,
            '|' => State::Trees,
            '#' => State::Lumberyard,
            _ => panic!(),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            State::Open => f.write_char('.'),
            State::Trees => f.write_char('|'),
            State::Lumberyard => f.write_char('#'),
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Map {
    rows : Vec<Vec<State>>,
}

impl Map {
    fn from_str(s : &str) -> Map {
        let rows = s.lines().map(|line| line.chars().map(State::from_char).collect()).collect();

        Map {rows}
    }

    fn adjacents(&self, x : usize, y : usize) -> Vec<(usize, usize)> {
        let mut adj = Vec::new();
        if y > 0 {
            if x > 0 {
                adj.push((x - 1, y - 1));
            }
            adj.push((x, y - 1));
            if x < self.rows[0].len() - 1 {
                adj.push((x + 1, y - 1));
            }
        }
        if x < self.rows[0].len() - 1 {
            adj.push((x + 1, y));
        }
        if y < self.rows.len() - 1 {
            if x < self.rows[0].len() - 1 {
                adj.push((x + 1, y + 1));
            }
            adj.push((x, y + 1));
            if x > 0 {
                adj.push((x - 1, y + 1));
            }
        }
        if x > 0 {
            adj.push((x - 1, y));
        }

        adj
    }

    fn tick(&self) -> Map {
        let mut map = self.clone();
        for y in 0..self.rows.len() {
            for x in 0..self.rows[0].len() {
                let adj = self.adjacents(x, y);
                let adj_states = adj.iter().map(|(x, y)| self.rows[*y][*x]);
                match self.rows[y][x] {
                    State::Open => {
                        if adj_states.filter(|s| *s == State::Trees).count() >= 3 {
                            map.rows[y][x] = State::Trees;
                        }
                    },
                    State::Trees => {
                        if adj_states.filter(|s| *s == State::Lumberyard).count() >= 3 {
                            map.rows[y][x] = State::Lumberyard;
                        }
                    },
                    State::Lumberyard => {
                        let has_lumberyard = adj_states.clone().any(|s| s == State::Lumberyard);
                        let has_trees = adj_states.clone().any(|s| s == State::Trees);
                        if !has_lumberyard || !has_trees {
                            map.rows[y][x] = State::Open;
                        }
                    },
                }
            }
        }

        map
    }

    fn resource_value(&self) -> usize {
        let n_l : usize = self.rows.iter().map(|r| r.iter().filter(|s| **s == State::Lumberyard).count()).sum();
        let n_t : usize = self.rows.iter().map(|r| r.iter().filter(|s| **s == State::Trees).count()).sum();
        n_l * n_t
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in self.rows.iter() {
            for s in row.iter() {
                f.write_str(&format!("{}", s)).unwrap();
            }
            f.write_str("\n").unwrap();
        }
        Ok(())
    }
}