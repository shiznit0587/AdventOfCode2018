use std::collections::HashMap;

pub fn day18(lines: &mut Vec<String>) {
    println!("Running Day 18 - a");

    // Parse the map, then make a copy of it.
    // When processing a generation, we swap which map we're reading from and writing to.
    let mut map = lines
        .iter()
        .map(|l| l.chars().map(Acre::from_char).collect())
        .collect::<Vec<Vec<Acre>>>();
    let mut map2 = map.clone();

    // Optimization: Pre-cache coordinates of all valid neighbors for each cell
    let mut neighbor_coords = Vec::with_capacity(map.len());

    for i in 0..map.len() {
        neighbor_coords.push(Vec::with_capacity(map[i].len()));
        for j in 0..map[i].len() as isize {
            let i = i as isize;
            let mut neighbors = Vec::new();
            for (i_off, j_off) in iproduct!(-1..2, -1..2) {
                let p = (i + i_off, j + j_off);
                if p.0 != i || p.1 != j {
                    if 0 <= p.0
                        && p.0 < map.len() as isize
                        && 0 <= p.1
                        && p.1 < map[i as usize].len() as isize
                    {
                        neighbors.push((p.0 as usize, p.1 as usize));
                    }
                }
            }
            neighbor_coords[i as usize].push(neighbors);
        }
    }

    let mut prev_map: &Vec<Vec<Acre>>;
    let mut cur_map: &mut Vec<Vec<Acre>>;
    let mut gens_by_value = HashMap::new();
    let mut value_by_gen = Vec::new();
    let mut generation = 0;
    let mut cycle_length = None;

    while cycle_length.is_none() {
        // Choose maps to read from & write to.
        let cur = generation % 2;
        if cur == 0 {
            prev_map = &map;
            cur_map = &mut map2;
        } else {
            prev_map = &map2;
            cur_map = &mut map;
        }

        // Process generation.
        for i in 0..prev_map.len() {
            for j in 0..prev_map[i].len() {
                let n_trees = count_acres(&neighbor_coords[i][j], &prev_map, Acre::is_trees);
                let n_yards = count_acres(&neighbor_coords[i][j], &prev_map, Acre::is_yard);
                cur_map[i][j] = match prev_map[i][j] {
                    Acre::Open => {
                        if n_trees >= 3 {
                            Acre::Trees
                        } else {
                            Acre::Open
                        }
                    }
                    Acre::Trees => {
                        if n_yards >= 3 {
                            Acre::Yard
                        } else {
                            Acre::Trees
                        }
                    }
                    Acre::Yard => {
                        if n_yards > 0 && n_trees > 0 {
                            Acre::Yard
                        } else {
                            Acre::Open
                        }
                    }
                };
            }
        }

        // Track this generation's value.
        let value = get_value(cur_map);
        value_by_gen.push(value);
        let gens = gens_by_value.entry(value).or_insert_with(|| Vec::new());
        gens.push(generation);

        // Attempt to identify a cycle.
        // We identify a cycle as when the map has the same value three times and the generations lapsed between them match.
        if gens.len() >= 3 {
            let end = gens.len() - 1;
            if gens[end] - gens[end - 1] == gens[end - 1] - gens[end - 2] {
                cycle_length = Some(gens[end] - gens[end - 1]);
            }
        }

        generation += 1;
    }

    println!(
        "Total Resource Value after 10 minutes = {}",
        value_by_gen[10]
    );

    println!("Running Day 18 - b");

    // Using what we know about the cycle length and cycle start,
    // find the correct value in the cycle for the 1 billionth generation.
    let cycle_start = generation - cycle_length.unwrap();
    let cycle_offset = (1_000_000_000 - cycle_start) % cycle_length.unwrap() - 1;

    println!(
        "Total Resource Value after 1,000,000,000 minutes = {}",
        value_by_gen[cycle_start + cycle_offset]
    );
}

fn get_value(map: &Vec<Vec<Acre>>) -> usize {
    count_map_acres(map, Acre::is_trees) * count_map_acres(map, Acre::is_yard)
}

fn count_map_acres(map: &Vec<Vec<Acre>>, comp: fn(&Acre) -> bool) -> usize {
    map.iter()
        .flat_map(|l| l.iter())
        .filter(|a| comp(a))
        .count()
}

fn count_acres(
    n: &Vec<(usize, usize)>,
    map: &Vec<Vec<Acre>>,
    comparison: fn(&Acre) -> bool,
) -> usize {
    n.iter()
        .map(|p| &map[p.0][p.1])
        .filter(|a| comparison(a))
        .count()
}

#[derive(Clone)]
enum Acre {
    Open,
    Trees,
    Yard,
}

impl Acre {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Acre::Open,
            '|' => Acre::Trees,
            '#' => Acre::Yard,
            _ => panic!(),
        }
    }

    fn _to_char(&self) -> char {
        match self {
            Acre::Open => '.',
            Acre::Trees => '|',
            Acre::Yard => '#',
        }
    }

    fn is_trees(&self) -> bool {
        match self {
            Acre::Trees => true,
            _ => false,
        }
    }

    fn is_yard(&self) -> bool {
        match self {
            Acre::Yard => true,
            _ => false,
        }
    }
}

fn _print(map: &Vec<Vec<Acre>>) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]._to_char());
        }
        println!();
    }
    println!();
}
