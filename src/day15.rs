use itertools::Itertools;
use pathfinding::utils::absdiff;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn day15(lines: &mut Vec<String>) {
    println!("Running Day 15 - a");

    let mut map = Map::new(&lines);

    // map._print();

    let mut ended = false;
    let mut round = 0;
    while !ended {
        let mut queue = build_queue(&map);

        while let Some(mut entry) = queue.pop() {
            if map[entry].is_open() {
                continue;
            }

            let targets = find_targets(entry, &map);

            if targets.is_empty() {
                ended = true;
                break;
            }

            if !target_adjacent(entry, &map, &targets) {
                let target_points = find_target_points(targets, &map);
                if target_points.is_empty() {
                    continue;
                }

                let dest = find_destination(entry, &target_points, &map);
                if dest.is_some() {
                    map.swap(entry, dest.unwrap());
                    entry = dest.unwrap();
                }
            }

            let mut enemies = get_enemy_neighbors(entry, &map);
            if !enemies.is_empty() {
                enemies.sort_by(|a, b| a.hp.cmp(&b.hp).then(b.location.cmp(&a.location)));
                let target = enemies.first().unwrap();
                map.attack(entry, target.location);
            }
        }

        // map._print();
        round += 1;
    }

    round -= 1;

    let hp_sum: usize = itertools::chain(&map.elves, &map.goblins)
        .filter(|u| u.alive)
        .map(|u| u.hp)
        .sum();

    println!(
        "Combat finished after {} rounds, hp sum = {}, outcome = {}",
        round,
        hp_sum,
        round * hp_sum
    );

    println!("Running Day 15 - b");
}

// Build a queue of points relating to all remaining units, in read order.
fn build_queue(map: &Map) -> BinaryHeap<Point> {
    let mut queue = BinaryHeap::new();
    for p in iproduct!(0..map.width(), 0..map.height()) {
        match map[p] {
            Tile::Goblin(_) | Tile::Elf(_) => queue.push(Point(p.0, p.1)),
            _ => (),
        }
    }
    queue
}

// Find all remaining target units of the opposite type as the one located at start.
fn find_targets(start: Point, map: &Map) -> Vec<&Unit> {
    (match map[start] {
        Tile::Goblin(_) => &map.elves,
        Tile::Elf(_) => &map.goblins,
        _ => panic!(),
    })
    .iter()
    .filter(|u| u.alive)
    .collect()
}

fn target_adjacent(entry: Point, map: &Map, targets: &Vec<&Unit>) -> bool {
    let target_locations: Vec<Point> = targets.iter().map(|t| t.location).collect();
    get_neighbors(entry, &map)
        .iter()
        .find(|p| target_locations.contains(&p))
        .is_some()
}

// Find all open points adjacent to a target.
// Result is a BinaryHeap of unique Points sorted in read order.
fn find_target_points(targets: Vec<&Unit>, map: &Map) -> BinaryHeap<Point> {
    targets
        .iter()
        .flat_map(|t| get_open_neighbors(t.location, &map))
        .unique()
        .collect()
}

// Find a adjacent destination Point to move a unit from start to that is one step closer,
// in read order, to the closest reachable target, in read order.
fn _find_destination_astar(
    start: Point,
    target_points: &BinaryHeap<Point>,
    map: &Map,
) -> Option<Point> {
    use pathfinding::directed::astar;

    let choices: Vec<(usize, Point)> = target_points
        .iter()
        .filter_map(|tp| {
            let a_stars = astar::astar_bag_collect(
                &start,
                |&p| get_open_neighbors(p, &map).into_iter().map(|p| (p, 1)),
                |p| _calc_manhattan(p, &tp),
                |p| *p == *tp,
            );

            match a_stars {
                Some(a_star_info) => {
                    let mut paths = a_star_info.0;
                    paths.sort_by(|a, b| b[1].cmp(&a[1]));
                    Some((a_star_info.1, paths[0][1]))
                }
                None => None,
            }
        })
        .sorted_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)))
        .collect();

    match choices.first() {
        Some(c) => Some(c.1),
        None => None,
    }
}

fn find_destination(start: Point, target_points: &BinaryHeap<Point>, map: &Map) -> Option<Point> {
    use pathfinding::directed::bfs;

    let bfs_results = target_points
        .iter()
        .map(|&tp| {
            (
                tp,
                bfs::bfs(&start, |&p| get_open_neighbors(p, &map), |p| *p == tp),
            )
        })
        .filter(|(_, bfs)| bfs.is_some())
        .map(|(tp, bfs)| (tp, bfs.unwrap().len()))
        .sorted_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)))
        .collect::<Vec<(Point, usize)>>();

    if bfs_results.first().is_none() {
        return None;
    }

    let (tp, distance) = bfs_results.first().unwrap();

    Some(
        *get_open_neighbors(start, &map)
            .iter()
            .map(|d| {
                (
                    d,
                    bfs::bfs(d, |&p| get_open_neighbors(p, &map), |p| *p == *tp),
                )
            })
            .filter(|(_, bfs)| bfs.is_some())
            .map(|(d, bfs)| (d, bfs.unwrap().len()))
            .filter(|(_, bfs)| *bfs == distance - 1)
            .next()
            .unwrap()
            .0,
    )
}

fn get_neighbors(p: Point, map: &Map) -> Vec<Point> {
    let mut points = Vec::new();
    if p.1 > 0 {
        points.push(Point(p.0, p.1 - 1));
    }
    if p.0 > 0 {
        points.push(Point(p.0 - 1, p.1));
    }
    if p.0 + 1 < map.width() {
        points.push(Point(p.0 + 1, p.1));
    }
    if p.1 + 1 < map.height() {
        points.push(Point(p.0, p.1 + 1));
    }
    points
}

fn get_open_neighbors(p: Point, map: &Map) -> Vec<Point> {
    get_neighbors(p, &map)
        .into_iter()
        .filter(|i| map[*i].is_open())
        .collect()
}

fn get_enemy_neighbors(p: Point, map: &Map) -> Vec<&Unit> {
    let neighbors = get_neighbors(p, &map);

    let enemies = match map[p] {
        Tile::Goblin(_) => &map.elves,
        Tile::Elf(_) => &map.goblins,
        _ => panic!(),
    };

    enemies
        .iter()
        .filter(|u| u.alive && neighbors.contains(&u.location))
        .collect()
}

fn _calc_manhattan(a: &Point, b: &Point) -> usize {
    absdiff(a.0, b.0) + absdiff(a.1, b.1)
}

#[derive(Clone)]
enum Tile {
    Wall,
    Open,
    Goblin(usize),
    Elf(usize),
}

impl Tile {
    fn from_char(c: char, goblin_id: &mut usize, elf_id: &mut usize) -> Option<Self> {
        match c {
            '#' => Some(Tile::Wall),
            '.' => Some(Tile::Open),
            'G' => {
                let id = *goblin_id;
                *goblin_id += 1;
                Some(Tile::Goblin(id))
            }
            'E' => {
                let id = *elf_id;
                *elf_id += 1;
                Some(Tile::Elf(id))
            }
            _ => None,
        }
    }

    fn _to_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Open => '.',
            Tile::Goblin(_) => 'G',
            Tile::Elf(_) => 'E',
        }
    }

    fn is_open(&self) -> bool {
        match self {
            Tile::Open => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point(usize, usize);

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        other.1.cmp(&self.1).then(other.0.cmp(&self.0))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Tiles = Vec<Vec<Tile>>;

struct Map {
    tiles: Tiles,
    goblins: Vec<Unit>,
    elves: Vec<Unit>,
}

impl Map {
    fn new(lines: &Vec<String>) -> Self {
        let mut goblins = 0;
        let mut elves = 0;

        let tiles = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| Tile::from_char(c, &mut goblins, &mut elves).unwrap())
                    .collect()
            })
            .collect();

        let mut map = Map {
            tiles: tiles,
            goblins: Vec::new(),
            elves: Vec::new(),
        };

        for p in iproduct!(0..map.height(), 0..map.width()) {
            match map.tiles[p.0][p.1] {
                Tile::Goblin(_) => {
                    map.goblins.push(Unit::new(Point(p.1, p.0)));
                }
                Tile::Elf(_) => {
                    map.elves.push(Unit::new(Point(p.1, p.0)));
                }
                _ => (),
            }
        }

        map
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn swap(&mut self, a: Point, b: Point) {
        let tile_a = self[a].clone();
        let tile_b = self[b].clone();
        self.tiles[a.1][a.0] = tile_b;
        self.tiles[b.1][b.0] = tile_a;

        match self[a] {
            Tile::Goblin(id) => self.goblins[id].location = a,
            Tile::Elf(id) => self.elves[id].location = a,
            _ => {}
        }

        match self[b] {
            Tile::Goblin(id) => self.goblins[id].location = b,
            Tile::Elf(id) => self.elves[id].location = b,
            _ => {}
        }
    }

    fn attack(&mut self, _: Point, b: Point) {
        let unit_b = match self[b] {
            Tile::Goblin(id) => &mut self.goblins[id],
            Tile::Elf(id) => &mut self.elves[id],
            _ => panic!(),
        };

        if unit_b.hp > 3 {
            unit_b.hp -= 3;
        } else {
            unit_b.hp = 0;
            unit_b.alive = false;
            self.tiles[b.1][b.0] = Tile::Open;
        }
    }

    fn _print(&self) {
        for line in self.tiles.iter() {
            println!("{}", line.iter().map(|t| t._to_char()).collect::<String>());
        }
        println!();
    }
}

impl std::ops::Index<Point> for Map {
    type Output = Tile;

    fn index(&self, i: Point) -> &Tile {
        &self.tiles[i.1][i.0]
    }
}

impl std::ops::Index<(usize, usize)> for Map {
    type Output = Tile;

    fn index(&self, i: (usize, usize)) -> &Tile {
        &self.tiles[i.1][i.0]
    }
}

struct Unit {
    location: Point,
    hp: usize,
    alive: bool,
}

impl Unit {
    fn new(location: Point) -> Self {
        Unit {
            location: location,
            hp: 200,
            alive: true,
        }
    }
}
