use std::io::BufRead;

pub fn readDay(day: i32) -> std::io::Result<Vec<String>> {
    let f = std::fs::File::open(format!("inputs/Day{}.txt", day))?;
    let f = std::io::BufReader::new(f);

    let mut lines: Vec<String> = Vec::new();

    for line in f.lines() {
        lines.push(line.unwrap());
    }

    Ok(lines)
}

pub fn tryParse_i32(string: &str) -> i32 {
    string.trim().parse().unwrap_or_default()
}
