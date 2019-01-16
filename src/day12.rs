use crate::utils;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

pub fn day12(lines: &mut Vec<String>) {
    println!("Running Day 12 - a");

    let initial_state = &lines[0];

    let initial_state = initial_state
        .chars()
        .skip(15)
        .map(|c| c == '#')
        .collect::<Vec<bool>>();

    let mut pots = Pots::new();

    for i in initial_state.iter().enumerate() {
        pots.set_pot(i.0 as isize, *i.1);
    }

    // println!(" 0: {:?}", pots);

    let rex = Regex::new(r"(.)(.)(.)(.)(.) => (.)").unwrap();

    let rules = lines
        .iter()
        .skip(2)
        .map(|l| rex.captures(l).unwrap())
        .map(|c| Rule::new(&c))
        .map(|r| (r.check, r))
        .collect::<HashMap<u32, Rule>>();

    for _gen in 1..=20 {
        pots.try_grow();
        let prev_pots = pots.clone();

        let pot_range = prev_pots.get_pot_range();
        for pot in pot_range.0..pot_range.1 {
            pots.set_pot(pot, rules[&prev_pots.get_pots(pot)].result);
        }

        // println!("{:2}: {:?}", _gen, pots);
    }

    let pot_range = pots.get_pot_range();
    let sum: isize = (pot_range.0..pot_range.1)
        .filter(|p| pots.get_pot(*p))
        .sum();

    println!("Potted Sum = {}", sum);

    println!("Running Day 12 - b");
}

fn to_char(b: bool) -> char {
    if b {
        '#'
    } else {
        '.'
    }
}

struct Rule {
    check: u32,
    result: bool,
}

impl Rule {
    fn new(c: &regex::Captures) -> Self {
        let mut check: u32 = 0;
        (0..5)
            .filter(|i| c[i + 1] == *"#")
            .for_each(|i| utils::set_bit(&mut check, i));
        Rule {
            check: check,
            result: c[6] == *"#",
        }
    }
}

impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}{}{}{}{} => {}",
            to_char(utils::is_bit_set(&self.check, 0)),
            to_char(utils::is_bit_set(&self.check, 1)),
            to_char(utils::is_bit_set(&self.check, 2)),
            to_char(utils::is_bit_set(&self.check, 3)),
            to_char(utils::is_bit_set(&self.check, 4)),
            to_char(self.result)
        )
    }
}

#[derive(Clone)]
struct Pots {
    pots: Vec<u32>,
    offset: usize,
}

impl Pots {
    fn new() -> Self {
        Pots {
            pots: vec![0; 4],
            offset: 0,
        }
    }

    #[inline]
    fn adapt_i(&self, i: isize) -> (usize, usize) {
        let i = (i + self.offset as isize * 32) as usize;
        (i % 32, i / 32)
    }

    fn get_pots(&self, i: isize) -> u32 {
        let (mut i, mut idx) = self.adapt_i(i);

        return match i {
            0 | 1 | 30 | 31 => {
                if i < 2 {
                    idx -= 1;
                    i += 32;
                }

                let mut pots: u64 = self.pots[idx + 1] as u64;
                pots = pots << 32 | (self.pots[idx] as u64);
                ((pots >> (i - 2)) & 0b11111) as u32
            }
            _ => (self.pots[idx] >> (i - 2)) & 0b11111,
        };
    }

    fn get_pot(&self, i: isize) -> bool {
        let (i, idx) = self.adapt_i(i);
        utils::is_bit_set(&self.pots[idx], i)
    }

    fn set_pot(&mut self, i: isize, value: bool) {
        let (i, idx) = self.adapt_i(i);
        utils::set_bit_to(&mut self.pots[idx], i, value);
    }

    fn try_grow(&mut self) {
        if (self.pots[0] & 0b1111) != 0 {
            self.offset += 1;
            self.pots.push(0);
            let len = self.pots.len();
            for i in 0..len - 1 {
                self.pots[len - i - 1] = self.pots[len - i - 2];
            }
            self.pots[0] = 0;
        }

        if (self.pots[self.pots.len() - 1] & 0xF0000000) != 0 {
            self.pots.push(0);
        }
    }

    fn get_pot_range(&self) -> (isize, isize) {
        (
            self.offset as isize * -32 + 2,
            (self.pots.len() as isize - self.offset as isize) * 32 - 2,
        )
    }
}

impl fmt::Debug for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            self.pots
                .iter()
                .map(|p| (0..32)
                    .map(|i| to_char(utils::is_bit_set(p, i)))
                    .collect::<String>())
                .collect::<String>()
        )
    }
}
