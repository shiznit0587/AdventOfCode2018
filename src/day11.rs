use crate::utils;

pub fn day11(lines: &mut Vec<String>) {
    println!("Running Day 11 - a");

    let serial: i32 = utils::parse(&lines[0]);

    let mut grid: [[i32; 300]; 300] = [[0; 300]; 300];
    for c in iproduct!(0..300, 0..300) {
        grid[c.0][c.1] = calc_power(c.0 as i32, c.1 as i32, serial);
    }

    let sums = build_sums(&grid, 3);
    let winner = find_winner(&sums, 3);

    println!(
        "Largest Power 3x3 anchor = {:?}, power = {}",
        winner.0, winner.1
    );

    println!("Running Day 11 - b");
}

fn calc_power(x: i32, y: i32, serial: i32) -> i32 {
    ((((x + 10) * y + serial) * (x + 10)) / 100) % 10 - 5
}

fn build_sums(grid: &[[i32; 300]; 300], patch_size: u32) -> Vec<i32> {
    let sums_width = 300 - patch_size + 1;
    let mut sums = vec![0; (sums_width * sums_width) as usize];

    for coord in iproduct!(0..sums_width, 0..sums_width) {
        sums[(coord.0 * sums_width + coord.1) as usize] = iproduct!(0..patch_size, 0..patch_size)
            .map(|c| grid[(coord.0 + c.0) as usize][(coord.1 + c.1) as usize])
            .sum();
    }

    sums
}

fn find_winner(sums: &Vec<i32>, patch_size: u32) -> ((u32, u32), i32) {
    let sums_width = 300 - patch_size + 1;

    iproduct!(0..sums_width, 0..sums_width)
        .map(|c| (c, sums[(c.0 * sums_width + c.1) as usize]))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
}
