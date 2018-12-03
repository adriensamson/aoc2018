use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

pub fn step1(input : String) {
    let claims = get_claims(input);
    let mut map : HashMap<(u32, u32), usize> = HashMap::new();

    for claim in claims.iter() {
        let mut i = 0;
        while i < claim.rect.width {
            let mut j = 0;
            while j < claim.rect.height {
                *map.entry((claim.rect.left + i, claim.rect.top + j)).or_default() += 1;
                j += 1;
            }
            i += 1;
        }
    }

    let count = map.iter().filter(|(_k, v)| **v > 1).count();
    println!("{}", count);
}

pub fn step2(input : String) {
    let claims = get_claims(input);
    for claim in claims.iter() {
        if !claims.iter().any(|c| c != claim && intersects(&c.rect, &claim.rect)) {
            println!("{}", claim.id);
        }
    }
}

fn get_claims(input : String) -> Vec<Claim> {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    input.lines().map(parse_line(re)).collect()
}

#[derive(Debug, PartialEq)]
struct Rect {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[derive(PartialEq)]
struct Claim {
    id: u32,
    rect : Rect,
}

fn parse_line(re : Regex) -> impl Fn(&str) -> Claim {
    move |line| {
        if let Some(cap) = re.captures(line) {
            return Claim {
                id: u32::from_str(&cap[1]).unwrap(),
                rect: Rect {
                    left: u32::from_str(&cap[2]).unwrap(),
                    top: u32::from_str(&cap[3]).unwrap(),
                    width: u32::from_str(&cap[4]).unwrap(),
                    height: u32::from_str(&cap[5]).unwrap(),
                }
            }
        }
        panic!();
    }
}

fn intersects(rect1 : &Rect, rect2 : &Rect) -> bool {
    let left = rect1.left.max(rect2.left);
    let right = (rect1.left + rect1.width).min(rect2.left + rect2.width);
    let top = rect1.top.max(rect2.top);
    let bottom = (rect1.top + rect1.height).min(rect2.top + rect2.height);
    left < right && top < bottom
}