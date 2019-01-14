// suppress for the whole module with inner attribute...
#![allow(non_snake_case)]

#[macro_use]
extern crate itertools;

use std::ops::AddAssign;
use std::time::{Duration, Instant};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub mod utils;

fn main() -> std::io::Result<()> {
    println!("\n🎅🎅🎅🎅🎅 ADVENT OF CODE 2018 🎅🎅🎅🎅🎅\n");

    let mut timings: Timings = Timings(Duration::new(0, 0), Duration::new(0, 0));

    // timings += time_day(day1::day1, 1)?;
    // timings += time_day(day2::day2, 2)?;
    // timings += time_day(day3::day3, 3)?;
    // timings += time_day(day4::day4, 4)?;
    // timings += time_day(day5::day5, 5)?;
    // timings += time_day(day6::day6, 6)?;
    // timings += time_day(day7::day7, 7)?;
    // timings += time_day(day8::day8, 8)?;
    // timings += time_day(day9::day9, 9)?;
    // timings += time_day(day10::day10, 10)?;
    timings += time_day(day11::day11, 11)?;
    // timings += time_day(day12::day12, 12)?;
    // timings += time_day(day13::day13, 13)?;
    // timings += time_day(day14::day14, 14)?;
    // timings += time_day(day15::day15, 15)?;
    // timings += time_day(day16::day16, 16)?;
    // timings += time_day(day17::day17, 17)?;
    // timings += time_day(day18::day18, 18)?;
    // timings += time_day(day19::day19, 19)?;
    // timings += time_day(day20::day20, 20)?;
    // timings += time_day(day21::day21, 21)?;
    // timings += time_day(day22::day22, 22)?;
    // timings += time_day(day23::day23, 23)?;
    // timings += time_day(day24::day24, 24)?;
    // timings += time_day(day25::day25, 25)?;

    println!("*************************************\n");
    println!(
        "IO = {:?}, CPU = {:?}, Total = {:?}",
        timings.0,
        timings.1,
        timings.0 + timings.1
    );

    Ok(())
}

fn time_day(day_fn: fn(&mut Vec<String>) -> (), day: i32) -> std::io::Result<Timings> {
    println!("----- Day {:02} -----", day);

    let now = Instant::now();
    let mut lines = utils::read_day(day)?;
    let io = now.elapsed();

    let now = Instant::now();
    day_fn(&mut lines);
    let cpu = now.elapsed();

    println!("------------------");
    println!(
        "Day {} :: IO = {:?}, CPU = {:?}, Total = {:?}\n",
        day,
        io,
        cpu,
        io + cpu
    );

    Ok(Timings(io, cpu))
}

struct Timings(Duration, Duration);

impl AddAssign for Timings {
    fn add_assign(&mut self, rhs: Timings) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
