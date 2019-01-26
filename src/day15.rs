use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub fn day15(lines: &mut Vec<String>) {
    println!("Running Day 15 - a");

    let orig_map = Map::new(&lines);

    let mut map = orig_map.clone();
    let rounds = simulate_combat(&mut map);

    println!(
        "Combat finished after {} rounds, outcome = {}",
        rounds,
        calc_outcome(&map, rounds)
    );

    println!("Running Day 15 - b");

    let mut result = (false, 0);
    let mut strength = 2;

    while !result.0 {
        strength += 1;
        map = orig_map.clone();
        result = simulate_elf_victory(strength, &mut map);
    }

    println!(
        "Combat finished with strength {} after {} rounds, outcome = {}",
        strength,
        result.1,
        calc_outcome(&map, result.1)
    );
}

fn calc_outcome(map: &Map, rounds: usize) -> usize {
    rounds
        * itertools::chain(&map.elves, &map.goblins)
            .filter(|u| u.alive)
            .map(|u| u.hp)
            .sum::<usize>()
}

// Simulates a full combat.
// Modifies map in place.
// Returns the number of completed rounds.
fn simulate_combat(map: &mut Map) -> usize {
    let mut round = 0;
    while !simulate_round(map) {
        round += 1;
    }
    round
}

// Simulates combat until a single elf dies, or the elves win.
// Modifies map in place.
// Returns (were elves victorious, the number of completed rounds).
fn simulate_elf_victory(strength: usize, map: &mut Map) -> (bool, usize) {
    map.elves.iter_mut().for_each(|u| u.strength = strength);

    let mut victory = true;
    let mut ended = false;
    let mut round = 0;

    while victory && !ended {
        ended = simulate_round(map);
        victory = map.elves.iter().filter(|u| !u.alive).count() == 0;
        round += 1;
    }

    (victory, round - 1)
}

// Simulates a round of combat, ending early when no enemies are found for a unit.
// Returns whether combat has ended entirely.
fn simulate_round(map: &mut Map) -> bool {
    let mut queue = build_queue(&map);

    while let Some(mut entry) = queue.pop() {
        if map[entry].is_open() {
            continue;
        }

        let targets = find_targets(entry, &map);

        if targets.is_empty() {
            return true;
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

    false
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

// Find all remaining target units of the opposite type as the one located at Point p.
fn find_targets(p: Point, map: &Map) -> Vec<&Unit> {
    (match map[p] {
        Tile::Goblin(_) => &map.elves,
        Tile::Elf(_) => &map.goblins,
        _ => panic!(),
    })
    .iter()
    .filter(|u| u.alive)
    .collect()
}

// Determine if a target unit is adjacent to Point p.
fn target_adjacent(p: Point, map: &Map, targets: &Vec<&Unit>) -> bool {
    let target_locations: Vec<Point> = targets.iter().map(|t| t.location).collect();
    get_neighbors(p, &map)
        .iter()
        .find(|p| target_locations.contains(&p))
        .is_some()
}

// Find all open points adjacent to a target.
// Result is a Vec of unique Points sorted in read order.
fn find_target_points(targets: Vec<&Unit>, map: &Map) -> Vec<Point> {
    targets
        .iter()
        .flat_map(|t| get_open_neighbors(t.location, &map))
        .unique()
        .sorted_by(|a, b| b.cmp(&a))
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

fn _find_destination_bfs(
    start: Point,
    target_points: &BinaryHeap<Point>,
    map: &Map,
) -> Option<Point> {
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

fn find_destination(start: Point, target_points: &Vec<Point>, map: &Map) -> Option<Point> {
    // Queue stores (node, bfs) tuples.
    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();

    // Each entry in BFS_Map contains (prev, bfs) tuple.
    let mut bfs_map: Vec<Vec<Option<(Point, usize)>>> = vec![vec![None; map.height()]; map.width()];

    // Prime the pump
    queue.push_back((start, 0));
    bfs_map[start.0][start.1] = Some((start, 0));
    let mut target_bfs = None;

    while let Some((node, bfs)) = queue.pop_front() {
        // Check if we found a closest target point (in read order).
        if target_points.contains(&node) {
            target_bfs = Some(bfs);
            break;
        }

        // Visit all neighbors to node, and queue them (in read order).
        let neighbors = get_open_neighbors(node, &map);
        for neighbor in neighbors {
            if bfs_map[neighbor.0][neighbor.1].is_none() {
                queue.push_back((neighbor, bfs + 1));
                bfs_map[neighbor.0][neighbor.1] = Some((node, bfs + 1));
            }
        }
    }
    if target_bfs.is_none() {
        return None;
    }
    let target_bfs = target_bfs.unwrap();

    // Find the first target we reached in target_bfs, in reading order.
    let tp = *target_points
        .iter()
        .filter(|p| bfs_map[p.0][p.1].is_some())
        .filter(|p| bfs_map[p.0][p.1].unwrap().1 == target_bfs)
        .next()
        .unwrap();

    let mut node = tp;
    let mut prev = bfs_map[tp.0][tp.1].unwrap().0;

    // Walk the same path we took to get here, backward.
    while prev != start {
        node = prev;
        prev = bfs_map[node.0][node.1].unwrap().0;
    }
    Some(node)
}

// Get neighboring points to p, within map's bounds (in read order).
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

// Gets open neighboring points to p, within map's bounds (in read order).
fn get_open_neighbors(p: Point, map: &Map) -> Vec<Point> {
    get_neighbors(p, &map)
        .into_iter()
        .filter(|i| map[*i].is_open())
        .collect()
}

// Gets enemy neighbors to p (in read order).
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
    use pathfinding::utils::absdiff;
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

#[derive(Clone)]
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

    fn attack(&mut self, a: Point, b: Point) {
        let unit_a_strength = match self[a] {
            Tile::Goblin(id) => self.goblins[id].strength,
            Tile::Elf(id) => self.elves[id].strength,
            _ => panic!(),
        };

        let unit_b = match self[b] {
            Tile::Goblin(id) => &mut self.goblins[id],
            Tile::Elf(id) => &mut self.elves[id],
            _ => panic!(),
        };

        if unit_b.hp > unit_a_strength {
            unit_b.hp -= unit_a_strength;
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

#[derive(Clone)]
struct Unit {
    location: Point,
    strength: usize,
    hp: usize,
    alive: bool,
}

impl Unit {
    fn new(location: Point) -> Self {
        Unit {
            location: location,
            strength: 3,
            hp: 200,
            alive: true,
        }
    }
}
