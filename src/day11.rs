use crate::utils;

pub fn day11(lines: &mut Vec<String>) {
    println!("Running Day 11 - a");

    let serial: i32 = utils::parse(&lines[0]);

    let mut grid: [[i32; 300]; 300] = [[0; 300]; 300];
    for c in iproduct!(0..300, 0..300) {
        grid[c.0][c.1] = calc_power(c.0 as i32, c.1 as i32, serial);
    }

    let mut sums: [[i32; 298]; 298] = [[0; 298]; 298];

    for coord in iproduct!(0..298, 0..298) {
        sums[coord.0][coord.1] = iproduct!(0..3, 0..3)
            .map(|c| grid[coord.0 + c.0][coord.1 + c.1])
            .sum();
    }

    let winner = iproduct!(0..298, 0..298)
        .map(|c| (c, sums[c.0][c.1]))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    println!(
        "Largest Power 3x3 anchor = {:?}, power = {}",
        winner.0, winner.1
    );

    println!("Running Day 11 - b");
}

fn calc_power(x: i32, y: i32, serial: i32) -> i32 {
    ((((x + 10) * y + serial) * (x + 10)) / 100) % 10 - 5
}
