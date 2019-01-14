use crate::utils;

pub fn day11(lines: &mut Vec<String>) {
    println!("Running Day 11 - a");

    // let serial: i32 = 18;
    // let serial: i32 = 42;
    let serial: i32 = utils::parse(&lines[0]);

    let mut grid: Grid = [[0; GRID_SIZE]; GRID_SIZE];
    for c in iproduct!(0..GRID_SIZE, 0..GRID_SIZE) {
        grid[c.0][c.1] = calc_power(c.0 as i32, c.1 as i32, serial);
    }

    let sums = build_sums(&grid, 3);
    let winner = sums.find_winner();

    println!(
        "Largest Power 3x3 anchor = {:?}, power = {}",
        winner.0, winner.1
    );

    println!("Running Day 11 - b");

    let mut patch_size_winner: [Option<Identifier>; GRID_SIZE] = [None; GRID_SIZE];
    let mut sums: Sums = build_sums(&grid, 1);
    patch_size_winner[0] = Some(sums.find_winner());

    for patch_size in 2..GRID_SIZE + 1 {
        sums = iter_sums(&grid, &sums);
        patch_size_winner[patch_size - 1] = Some(sums.find_winner());
    }

    let winner = patch_size_winner
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.unwrap().1.cmp(&b.1.unwrap().1))
        .unwrap();

    println!(
        "Largest Power Square Identifier = {},{},{} with power = {}",
        (winner.1.unwrap().0).0,
        (winner.1.unwrap().0).1,
        winner.0 + 1,
        winner.1.unwrap().1
    );
}

fn calc_power(x: i32, y: i32, serial: i32) -> i32 {
    ((((x + 10) * y + serial) * (x + 10)) / 100) % 10 - 5
}

fn build_sums(grid: &Grid, patch_size: usize) -> Sums {
    let mut sums = Sums::new(patch_size);

    for coord in iproduct!(0..sums.width, 0..sums.width) {
        sums[coord.0][coord.1] = iproduct!(0..patch_size, 0..patch_size)
            .map(|c| grid[coord.0 + c.0][coord.1 + c.1])
            .sum();
    }

    sums
}

fn iter_sums(grid: &Grid, prev_sums: &Sums) -> Sums {
    let mut sums = Sums::new(prev_sums.patch_size + 1);

    for coord in iproduct!(0..sums.width, 0..sums.width) {
        // Get a collection of coords that are the edges to add to the previous sum
        let mut edge_coords: Vec<Point> = Vec::with_capacity(sums.patch_size * 2);
        for i in 0..sums.patch_size {
            edge_coords.push((coord.0 + sums.patch_size - 1, coord.1 + i));
            edge_coords.push((coord.0 + i, coord.1 + sums.patch_size - 1));
        }

        // Remove the duplicate corner
        edge_coords.pop();

        // Sum the values of the edge coords
        let edges_sum: i32 = edge_coords.iter().map(|c| grid[c.0][c.1]).sum();

        // Track the iterative sum
        let prev_sum = prev_sums[coord.0][coord.1];
        sums[coord.0][coord.1] = prev_sum + edges_sum;
    }

    sums
}

const GRID_SIZE: usize = 300;

type Point = (usize, usize);
type Identifier = (Point, i32);
type Grid = [[i32; GRID_SIZE]; GRID_SIZE];

struct Sums {
    sums: Vec<i32>,
    patch_size: usize,
    width: usize,
}

impl Sums {
    fn new(patch_size: usize) -> Self {
        let width = GRID_SIZE - patch_size + 1;
        Sums {
            sums: vec![0; width * width],
            patch_size: patch_size,
            width: width,
        }
    }

    fn find_winner(&self) -> Identifier {
        iproduct!(0..self.width, 0..self.width)
            .map(|c| (c, self[c.0][c.1]))
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
    }

    fn _print(&self) {
        println!("Sums for Patch Size = {}:", self.patch_size);
        for j in 0..self.width {
            let line = (0..self.width)
                .map(|i| format!("{:3} ", self[i][j]))
                .collect::<String>();
            println!("{}", line);
        }
    }
}

impl std::ops::Index<usize> for Sums {
    type Output = [i32];

    fn index(&self, i: usize) -> &[i32] {
        &self.sums[(i * self.width)..((i + 1) * self.width)]
    }
}

impl std::ops::IndexMut<usize> for Sums {
    fn index_mut(&mut self, i: usize) -> &mut [i32] {
        &mut self.sums[(i * self.width)..((i + 1) * self.width)]
    }
}
