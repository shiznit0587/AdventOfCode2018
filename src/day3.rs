use crate::utils;
use regex::Regex;

pub fn day3(lines: &mut Vec<String>) {
    println!("Running Day 3 - a");

    // Rawr
    let rex = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();

    let mut claims: Vec<Claim> = Vec::new();

    for line in lines.iter() {
        let cap = rex.captures(line).unwrap();

        claims.push(Claim {
            id: utils::parse(&cap[1]),
            x: utils::parse(&cap[2]),
            y: utils::parse(&cap[3]),
            w: utils::parse(&cap[4]),
            h: utils::parse(&cap[5]),
        });
    }

    let mut cloth: Vec<i32> = vec![0; 1000 * 1000];

    for claim in claims.iter() {
        for x in claim.x..claim.x + claim.w {
            for y in claim.y..claim.y + claim.h {
                cloth[(x * 1000 + y) as usize] += 1;
            }
        }
    }

    let count = cloth.iter().filter(|&x| *x > 1).count();

    println!("Count = {}", count);

    println!("Running Day 3 - b");

    for claim in claims.iter() {
        let mut unique = true;
        for x in claim.x..claim.x + claim.w {
            for y in claim.y..claim.y + claim.h {
                if cloth[(x * 1000 + y) as usize] > 1 {
                    unique = false;
                }
            }
        }
        if unique {
            println!("Non-Overlapping Claim ID = {}", claim.id);
        }
    }
}

struct Claim {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

// Unused, kept for example
impl std::fmt::Display for Claim {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "#{} @{},{}: {}x{}",
            self.id, self.x, self.y, self.w, self.h
        )
    }
}
