use crate::utils;
use std::collections::HashSet;

pub fn day1() -> std::io::Result<()> {
    println!("Running Day 1 - a");

    let lines = utils::readDay(1)?;

    let mut day1a: i32 = 0;

    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut day1b: i32 = 0;
    let mut day1bComplete: bool = false;
    let mut day1aComplete: bool = false;

    while !day1aComplete || !day1bComplete {
        for line in lines.iter() {
            let sign = &line[..1];

            let mult = match sign {
                "-" => -1,
                "+" => 1,
                _ => return Ok(()),
            };

            let val = &line[1..];
            let val: i32 = utils::tryParse_i32(val);

            day1a += mult * val;

            if !day1bComplete && !frequencies.insert(day1a) {
                day1b = day1a;
                day1bComplete = true;
            }
        }

        if !day1aComplete {
            day1aComplete = true;
            println!("Total = {}", day1a);
        }
    }

    println!("Running Day 1 - b");
    println!("First Frequency Reached Twice = {}", day1b);

    Ok(())
}
