use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

pub fn step1(input : String) {
    let nights = parse_input(input);
    let nights_by_guard = group_by_guard(&nights);

    let guard = find_most_asleep_guard(&nights_by_guard);
    let minute = find_most_asleep_minute(&nights_by_guard[&guard]);
    println!("{} * {} = {}", guard, minute, guard * minute);
}

#[derive(Debug, Clone)]
struct Night {
    guard : usize,
    asleeps: Vec<(usize, usize)>
}

impl Night {
    fn asleep_duration(&self) -> usize {
        self.asleeps.iter().fold(0, |tot, (start, end)| tot + end - start)
    }
}

fn parse_input(input : String) -> Vec<Night> {
    let guard_re = Regex::new(r"\[1518-\d+-\d+ \d+:\d+] Guard #(\d+) begins shift").unwrap();
    let asleep_re = Regex::new(r"\[1518-\d+-\d+ \d+:(\d+)] falls asleep").unwrap();
    let wake_re = Regex::new(r"\[1518-\d+-\d+ \d+:(\d+)] wakes up").unwrap();

    let mut lines : Vec<&str> = input.lines().collect();
    lines.sort();
    let mut nights = Vec::new();

    let mut guard = None;
    let mut asleep = None;
    let mut asleeps = Vec::new();

    for line in lines {
        if let Some(caps) = guard_re.captures(line) {
            if asleep.is_some() {
                panic!("new guard while asleep");
            }
            if let Some(g) = guard {
                nights.push(Night {guard: g, asleeps});
                asleeps = Vec::new();
            }
            guard = Some(usize::from_str(&caps[1]).unwrap());
        }
        if let Some(caps) = asleep_re.captures(line) {
            if guard.is_none() {
                panic!("asleep but no guard");
            }
            if asleep.is_some() {
                panic!("already asleep");
            }
            asleep = Some(usize::from_str(&caps[1]).unwrap());
        }
        if let Some(caps) = wake_re.captures(line) {
            if guard.is_none() {
                panic!("asleep but no guard");
            }
            match asleep {
                None => panic!("not asleep"),
                Some(a) => {
                    asleeps.push((a, usize::from_str(&caps[1]).unwrap()));
                    asleep = None;
                }
            }
        }
    }

    nights
}

fn group_by_guard(nights : &Vec<Night>) -> HashMap<usize, Vec<Night>> {
    let mut map: HashMap<usize, Vec<Night>> = HashMap::new();

    for night in nights {
        map.entry(night.guard).or_default().push(night.clone());
    }

    map
}

fn find_most_asleep_guard(night_by_guards : &HashMap<usize, Vec<Night>>) -> usize {
    night_by_guards.iter().fold((0, 0), |(g, d), (&newg, nights)| {
        let newd = nights.iter().fold(0, |tot, n| tot + n.asleep_duration());
        if newd > d {
            (newg, newd)
        } else {
            (g, d)
        }
    }).0
}

fn find_most_asleep_minute(nights : &Vec<Night>) -> usize {
    let mut minutes = Vec::new();
    for _i in 0usize..59 {
        minutes.push(0u32);
    }
    nights.iter().for_each(|night| {
        night.asleeps.iter().for_each(|(from, to)| {
            let mut m = *from;
            while m < *to {
                minutes[m] += 1;
                m += 1;
            }
        })
    });

    minutes.iter().fold((0, 0, 0), |(m1, n1, i), &n2| {
        if n2 > n1 {
            (i, n2, i + 1)
        } else {
            (m1, n1, i + 1)
        }
    }).0
}