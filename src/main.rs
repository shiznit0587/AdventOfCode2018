// suppress for the whole module with inner attribute...
#![allow(non_snake_case)]

use std::ops::AddAssign;
use std::time::{Duration, Instant};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

pub mod utils;

fn main() -> std::io::Result<()> {
    println!("\n🎅🎅🎅🎅🎅 ADVENT OF CODE 2018 🎅🎅🎅🎅🎅\n");

    let mut timings: Timings = Timings(Duration::new(0, 0), Duration::new(0, 0));

    timings += time_day(day1::day1, 1)?;
    timings += time_day(day2::day2, 2)?;
    timings += time_day(day3::day3, 3)?;
    timings += time_day(day4::day4, 4)?;
    timings += time_day(day5::day5, 5)?;
    timings += time_day(day6::day6, 6)?;
    timings += time_day(day7::day7, 7)?;
    timings += time_day(day8::day8, 8)?;
    timings += time_day(day9::day9, 9)?;
    timings += time_day(day10::day10, 10)?;

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
