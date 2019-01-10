use std::io::BufRead;
use std::str::FromStr;

pub fn read_day(day: i32) -> std::io::Result<Vec<String>> {
    let f = std::fs::File::open(format!("inputs/Day{}.txt", day))?;
    let f = std::io::BufReader::new(f);

    let mut lines: Vec<String> = Vec::new();

    for line in f.lines() {
        lines.push(line.unwrap());
    }

    Ok(lines)
}

pub fn parse<T>(string: &str) -> T
where
    T: FromStr + Default,
{
    string.trim().parse().unwrap_or_default()
}

pub fn set_bit(bits: &mut u32, bit: usize) {
    *bits |= 0x1 << bit;
}

pub fn unset_bit(bits: &mut u32, bit: usize) {
    *bits &= !(1 << bit);
}

pub fn is_bit_set(bits: &u32, bit: usize) -> bool {
    *bits & (1 << bit) != 0
}

pub fn intersection(a: u32, b: u32) -> u32 {
    a & b
}

pub fn difference(a: u32, b: u32) -> u32 {
    a & !b
}
