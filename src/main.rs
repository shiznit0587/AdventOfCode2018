// suppress for the whole module with inner attribute...
#![allow(non_snake_case)]

use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

pub mod utils;

fn main() -> std::io::Result<()> {
    time_day(day1::day1, 1)?;
    time_day(day2::day2, 2)?;
    time_day(day3::day3, 3)?;
    time_day(day4::day4, 4)?;
    time_day(day5::day5, 5)?;
    time_day(day6::day6, 6)?;
    time_day(day7::day7, 7)?;
    time_day(day8::day8, 8)?;

    Ok(())
}

fn time_day(day_fn: fn(&mut Vec<String>) -> (), day: i32) -> std::io::Result<()> {
    println!("--- Day {} ---", day);
    let mut lines = utils::read_day(day)?;
    let now = Instant::now();

    day_fn(&mut lines);

    let elapsed = now.elapsed();
    println!("--------------");
    println!("Day {} :: Duration = {:?}\n", day, elapsed);

    Ok(())
}
