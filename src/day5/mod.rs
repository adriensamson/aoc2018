use std::collections::HashMap;

pub fn step1(input : String) {
    println!("{}", size_after_all_reactions(&input));
}

pub fn step2(input : String) {
    let mut min_sizes = HashMap::new();
    for cu8 in b'a'..b'z' {
        let c = char::from(cu8);
        min_sizes.insert(c, size_after_all_reactions(&without_char(&input, c)));
    }

    let (c, n) = min_sizes.iter().fold((' ', input.len()), |acc, (&c, &n)| {
        if n < acc.1 {
            (c, n)
        } else {
            acc
        }
    });
    println!("{} : {}", c, n);
}

fn size_after_all_reactions(input : &str) -> usize {
    let mut res = String::from(input);
    let mut last_size = res.len();
    loop {
        res = do_reactions(&res);
        if last_size == res.len() {
            return res.len();
        }
        last_size = res.len();
    }
}

fn do_reactions(input : &str) -> String {
    let mut out = String::new();
    let mut last_char = None;
    for c in input.chars() {
        match last_char {
            None => {
                last_char = Some(c);
            },
            Some(prev) => {
                if prev != c && prev.eq_ignore_ascii_case(&c) {
                    last_char = None;
                } else {
                    out.push(prev);
                    last_char = Some(c);
                }
            }
        }
    }
    if let Some(c) = last_char {
        out.push(c);
    }
    out
}

fn without_char(input : &str, char : char) -> String {
    input.chars().filter(|c| !c.eq_ignore_ascii_case(&char)).collect()
}