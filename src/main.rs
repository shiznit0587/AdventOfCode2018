// suppress for the whole module with inner attribute...
#![allow(non_snake_case)]

mod day1;

pub mod utils;

fn main() -> std::io::Result<()> {
    day1::day1()?;

    Ok(())
}
