// suppress for the whole module with inner attribute...
#![allow(non_snake_case)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub mod utils;

fn main() -> std::io::Result<()> {
    day1::day1()?;
    day2::day2()?;
    day3::day3()?;
    day4::day4()?;
    day5::day5()?;
    day6::day6()?;

    Ok(())
}
