use std::fmt::{Display, Formatter, Error};
use std::str::FromStr;


pub fn step1(input : String) {
    let n_recipes = usize::from_str(&input).unwrap();
    let mut recipes = Recipes::new();
    while recipes.recipes.len() < n_recipes + 10 {
        recipes.make_recipes();
    }
    let next_ten = &recipes.recipes[n_recipes..(n_recipes+10)];
    println!("{}{}{}{}{}{}{}{}{}{}", next_ten[0], next_ten[1], next_ten[2], next_ten[3], next_ten[4], next_ten[5], next_ten[6], next_ten[7], next_ten[8], next_ten[9]);
}

struct Recipes {
    elf1_pos : usize,
    elf2_pos : usize,
    recipes : Vec<u8>,
}

impl Recipes {
    fn new() -> Recipes {
        Recipes {
            elf1_pos: 0,
            elf2_pos: 1,
            recipes : vec![3, 7],
        }
    }

    fn make_recipes(&mut self) {
        let sum = self.recipes[self.elf1_pos] + self.recipes[self.elf2_pos];
        let digits : Vec<char> = format!("{}", sum).chars().collect();
        for d in digits {
            self.recipes.push(u8::from_str(&format!("{}", d)).unwrap());
        }
        self.elf1_pos = (self.elf1_pos + 1 + self.recipes[self.elf1_pos] as usize) % self.recipes.len();
        self.elf2_pos = (self.elf2_pos + 1 + self.recipes[self.elf2_pos] as usize) % self.recipes.len();
    }
}

impl Display for Recipes {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> Result<(), Error> {
        for (i, r) in self.recipes.iter().enumerate() {
            if i == self.elf1_pos {
                f.write_str(&format!("({})", r)).unwrap();
            } else if i == self.elf2_pos {
                f.write_str(&format!("[{}]", r)).unwrap();
            } else {
                f.write_str(&format!(" {} ", r)).unwrap();
            }
        }
        Result::Ok(())
    }
}