use regex::Regex;
use std::str::FromStr;
use std::iter::FromIterator;
use std::collections::HashMap;

pub fn step1(input : String) {
    let mut groups = parse_input(input);

    loop {
        // selection
        let mut selecters: Vec<usize> = Vec::from_iter(0..groups.len());
        selecters.sort_by(|g1, g2| groups[*g1].effective_power().cmp(&groups[*g2].effective_power()).reverse().then(groups[*g1].initiative.cmp(&groups[*g2].initiative).reverse()));

        let mut selectables: Vec<usize> = Vec::from_iter(0..groups.len());
        let mut fights: Vec<(usize, usize)> = Vec::new();
        for selecter in selecters {
            let mut others: Vec<usize> = selectables.iter().filter(|i| groups[**i].army != groups[selecter].army).map(|i| *i).collect();
            others.sort_by(|&g1, &g2|
                groups[selecter].compute_damage_to(&groups[g1]).cmp(&groups[selecter].compute_damage_to(&groups[g2])).reverse()
                    .then(groups[g1].effective_power().cmp(&groups[g2].effective_power()).reverse())
                    .then(groups[g1].initiative.cmp(&groups[g2].initiative).reverse())
            );
            if let Some(selected) = others.get(0) {
                if groups[selecter].compute_damage_to(&groups[*selected]) > 0 {
                    fights.push((selecter, *selected));
                    selectables.retain(|i| *i != *selected);
                }
            }
        }
        // attacking
        fights.sort_by(|f1, f2| groups[f1.0].initiative.cmp(&groups[f2.0].initiative).reverse());
        for (attacker, defendent) in fights {
            let damage = groups[attacker].compute_damage_to(&groups[defendent]);
            println!("group {} attacks {} with damage {}", groups[attacker].id, groups[defendent].id, damage);
            groups[defendent].take_damage(damage);
        }

        // remove dead
        groups.retain(|g| g.units > 0);

        println!("Immune system:");
        groups.iter().filter(|g| g.army == Army::ImmuneSystem).for_each(|g| println!("group {} contains {} units", g.id, g.units));
        println!("Infection:");
        groups.iter().filter(|g| g.army == Army::Infection).for_each(|g| println!("group {} contains {} units", g.id, g.units));

        if groups.iter().filter(|g| g.army == Army::Infection).count() == 0 {
            println!("no more infection");
            break;
        }
        if groups.iter().filter(|g| g.army == Army::ImmuneSystem).count() == 0 {
            println!("no more immune system");
            break;
        }
    }
    let remaining : usize = groups.iter().map(|g| g.units).sum();
    println!("remaining units: {}", remaining);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum AttackKind {
    Fire,
    Cold,
    Slashing,
    Radiation,
    Bludgeoning,
}

impl AttackKind {
    fn from_str(s : &str) -> Option<AttackKind> {
        match s {
            "fire" => Some(AttackKind::Fire),
            "cold" => Some(AttackKind::Cold),
            "slashing" => Some(AttackKind::Slashing),
            "radiation" => Some(AttackKind::Radiation),
            "bludgeoning" => Some(AttackKind::Bludgeoning),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Army {
    ImmuneSystem,
    Infection,
}

#[derive(Debug)]
struct Group {
    id : String,
    army: Army,
    units: usize,
    hit_points : usize,
    weaknesses : Vec<AttackKind>,
    immunities : Vec<AttackKind>,
    attack_kind : AttackKind,
    attack_damage : usize,
    initiative : usize,
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.attack_damage
    }

    fn compute_damage_to(&self, other : &Self) -> usize {
        if other.immunities.contains(&self.attack_kind) {
            0
        } else if other.weaknesses.contains(&self.attack_kind) {
            self.attack_damage * self.units * 2
        } else {
            self.attack_damage * self.units
        }
    }

    fn take_damage(&mut self, damage : usize) {
        let nb_units = damage / self.hit_points;
        if nb_units < self.units {
            self.units -= nb_units;
        } else {
            self.units = 0;
        }
    }
}

fn parse_input(input : String) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut current_army = Army::ImmuneSystem;
    let mut current_line : Option<String> = None;
    let mut lines = Vec::new();

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }
        if &line[0..1] == " " {
            match current_line {
                Some(ref mut l) => l.push_str(line),
                None => panic!("bad_continuation"),
            }
        } else {
            if let Some(l) = current_line {
                lines.push((current_army, l));
                current_line = None;
            }
            if line == "Immune System:" {
                current_army = Army::ImmuneSystem;
            } else if line == "Infection:" {
                current_army = Army::Infection;
            } else {
                current_line = Some(String::from(line));
            }
        }
    }
    if let Some(l) = current_line {
        lines.push((current_army, l));
    }

    let line_re = Regex::new(r"(\d+) units each with (\d+) hit points(?: \(([^)]+)\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();

    let mut count = HashMap::new();
    count.insert(Army::Infection, 0usize);
    count.insert(Army::ImmuneSystem, 0usize);
    for (army, line) in lines {
        count.entry(army).and_modify(|c| *c += 1);
        let caps = match line_re.captures(&line) {
            None => panic!(format!("{}", line)),
            Some(caps) => caps,
        };
        let units= usize::from_str(&caps[1]).unwrap();
        let hit_points = usize::from_str(&caps[2]).unwrap();
        let (weaknesses, immunities) = parse_weak_imm(caps.get(3).map(|m| m.as_str()).unwrap_or(""));
        let attack_damage = usize::from_str(&caps[4]).unwrap();
        let attack_kind = AttackKind::from_str(&caps[5]).unwrap();
        let initiative = usize::from_str(&caps[6]).unwrap();
        groups.push(Group {
            id: format!("{} {}", if army == Army::Infection { "Infection" } else { "ImmuneSys" }, count[&army]),
            army,
            units,
            hit_points,
            weaknesses,
            immunities,
            attack_kind,
            attack_damage,
            initiative,
        });
    }

    groups
}

fn parse_weak_imm(s : &str) -> (Vec<AttackKind>, Vec<AttackKind>) {
    let mut weaknesses= Vec::new();
    let mut immunities= Vec::new();

    if s == "" {
        return (weaknesses, immunities);
    }

    for group in s.split("; ") {
        let (v, s2) = if group.starts_with("weak to ") {
            (&mut weaknesses, group.trim_start_matches("weak to "))
        } else if group.starts_with("immune to ") {
            (&mut immunities, group.trim_start_matches("immune to "))
        } else {
            panic!(format!("{}", group))
        };
        let mut attacks : Vec<AttackKind> = s2.split(", ").map(|a| AttackKind::from_str(a).unwrap()).collect();
        v.append(&mut attacks);
    }

    (weaknesses, immunities)
}