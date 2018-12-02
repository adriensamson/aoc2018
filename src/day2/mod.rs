use std::collections::HashMap;

pub fn step1(input : String) {
    let lines : Vec<&str> = input.lines().collect();

    let n2 = lines.iter().filter(|l| has_exactly_n_times_char(l, &2)).count();
    let n3 = lines.iter().filter(|l| has_exactly_n_times_char(l, &3)).count();

    println!("{} * {} = {}", n2, n3, n2 * n3);
}

pub fn step2(input : String) {
    let lines : Vec<&str> = input.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let w1 = lines[i];
        let mut j = i + 1;
        while j < lines.len() {
            let w2 = lines[j];
            match almost_same(w1, w2) {
                Some(s) => {
                    println!("{}", s);
                    return;
                },
                None => ()
            }
            j += 1;
        }
        i += 1;
    }
}

fn has_exactly_n_times_char(s : &str, n : &usize) -> bool {
    let mut map = HashMap::new();
    for c in s.chars() {
        let count = map.get(&c).unwrap_or(&0usize) + 1;
        map.insert(c, count);
    }
    map.iter().any(|(_k, v)| v == n)
}

fn almost_same(s1 : &str, s2 : &str) -> Option<String> {
    let mut n_diff = 0;
    let mut chars1 = s1.chars();
    let mut chars2 = s2.chars();
    let mut rs = String::new();
    loop {
        match (chars1.next(), chars2.next()) {
            (Some(c1), Some(c2)) => {
                if c1 != c2 {
                    n_diff += 1;
                    if n_diff >= 2 {
                        return None
                    }
                } else {
                    rs.push(c1);
                }
            }
            (None, None) => break,
            _ => return None
        }
    }
    if n_diff == 1 {
        return Some(rs)
    }
    None
}