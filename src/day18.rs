pub fn day18(lines: &mut Vec<String>) {
    println!("Running Day 18 - a");

    let mut map = lines
        .iter()
        .map(|l| l.chars().map(Acre::from_char).collect())
        .collect::<Vec<Vec<Acre>>>();

    // _print(&map);

    let mut neighbor_coords = Vec::with_capacity(map.len());

    // pre-cache coordinates of all valid neighbors for each cell
    for i in 0..map.len() {
        neighbor_coords.push(Vec::with_capacity(map[i].len()));
        for j in 0..map[i].len() {
            let i = i as isize;
            let j = j as isize;
            let mut neighbors = Vec::new();
            for (i_off, j_off) in iproduct!(0 as isize..3, 0 as isize..3) {
                let p = (i + i_off - 1, j + j_off - 1);
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

    let mut map2 = map.clone();

    let mut prev_map: &Vec<Vec<Acre>>;
    let mut cur_map: &mut Vec<Vec<Acre>> = &mut map;

    for i in 0..10 {
        let cur = i % 2;
        if cur == 0 {
            prev_map = &map;
            cur_map = &mut map2;
        } else {
            prev_map = &map2;
            cur_map = &mut map;
        }

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
        // _print(cur_map);
    }

    let n_trees = count_map_acres(cur_map, Acre::is_trees);
    let n_yards = count_map_acres(cur_map, Acre::is_yard);

    println!("Total Resource Value = {}", n_trees * n_yards);

    println!("Running Day 18 - b");
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
    Open = 0,
    Trees = 1,
    Yard = 2,
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
}
