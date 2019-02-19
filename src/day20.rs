use std::collections::HashSet;
use std::collections::VecDeque;

pub fn day20(lines: &mut Vec<String>) {
    println!("Running Day 20 - a");

    let mut doors = HashSet::new();
    let tokens = lines[0].chars().filter_map(|c| Token::parse(c)).collect();

    branch(&tokens, 0, (0, 0), &mut doors);

    let (doors, start) = normalize(&doors);
    let bfs_map = build_bfs(&doors, start);

    let max_bfs = bfs_map
        .iter()
        .flat_map(|l| l.iter().filter_map(|&d| d))
        .max()
        .unwrap();

    println!("Distance to Furthest Room = {}", max_bfs);

    println!("Running Day 20 - b");

    let count = bfs_map
        .iter()
        .flat_map(|l| l.iter().filter_map(|&d| d))
        .filter(|&d| d >= 1000)
        .count();

    println!("Rooms with Distance >= 1000 = {}", count);
}

fn branch(
    tokens: &Vec<Token>,
    mut cursor: usize,
    start_p: Point,
    doors: &mut HashSet<Door>,
) -> usize {
    let mut p = (start_p.0, start_p.1);

    while cursor < tokens.len() {
        match &tokens[cursor] {
            Token::Direction(d) => {
                let from = (p.0, p.1);
                p = d.walk(&from);
                doors.insert(Door::new(from, p));
                cursor += 1;
            }
            Token::GroupStart => cursor = branch(tokens, cursor + 1, p, doors),
            Token::Or => {
                p = start_p;
                cursor += 1;
            }
            Token::GroupEnd => break,
        }
    }

    cursor + 1
}

fn normalize(doors: &HashSet<Door>) -> (HashSet<Door>, Point) {
    let min_x = doors.iter().map(|d| d.from.0).min().unwrap();
    let min_y = doors.iter().map(|d| d.from.1).min().unwrap();

    let mut new_doors = HashSet::new();
    for door in doors {
        new_doors.insert(Door::new(
            (door.from.0 - min_x, door.from.1 - min_y),
            (door.to.0 - min_x, door.to.1 - min_y),
        ));
    }

    (new_doors, (-min_x, -min_y))
}

fn build_bfs(doors: &HashSet<Door>, start: Point) -> BfsMap {
    let max_x = doors.iter().map(|d| d.to.0).max().unwrap() as usize + 1;
    let max_y = doors.iter().map(|d| d.to.1).max().unwrap() as usize + 1;
    let mut bfs_map = vec![vec![None; max_y]; max_x];

    let mut queue: VecDeque<(UPoint, usize)> = VecDeque::new();

    let start = (start.0 as usize, start.1 as usize);
    queue.push_back((start, 0));
    bfs_map[start.0][start.1] = Some(0);

    while let Some((p, bfs)) = queue.pop_front() {
        let neighbors = get_neighbors(p, doors);
        for n in neighbors {
            if bfs_map[n.0][n.1].is_none() {
                bfs_map[n.0][n.1] = Some(bfs + 1);
                queue.push_back((n, bfs + 1));
            }
        }
    }

    bfs_map
}

fn get_neighbors(p: UPoint, doors: &HashSet<Door>) -> Vec<UPoint> {
    let p = (p.0 as isize, p.1 as isize);
    let mut neighbors = Vec::new();
    if p.0 > 0 && doors.contains(&Door::new(p, (p.0 - 1, p.1))) {
        neighbors.push(((p.0 - 1) as usize, p.1 as usize));
    }
    if p.1 > 0 && doors.contains(&Door::new(p, (p.0, p.1 - 1))) {
        neighbors.push((p.0 as usize, (p.1 - 1) as usize));
    }
    if doors.contains(&Door::new(p, (p.0 + 1, p.1))) {
        neighbors.push(((p.0 + 1) as usize, p.1 as usize));
    }
    if doors.contains(&Door::new(p, (p.0, p.1 + 1))) {
        neighbors.push((p.0 as usize, (p.1 + 1) as usize));
    }
    neighbors
}

fn _print(doors: &HashSet<Door>, p: Point) {
    let x_bounds = (
        doors.iter().map(|d| d.from.0).min().unwrap(),
        doors.iter().map(|d| d.to.0).max().unwrap() + 1,
    );
    let y_bounds = (
        doors.iter().map(|d| d.from.1).min().unwrap(),
        doors.iter().map(|d| d.to.1).max().unwrap() + 1,
    );

    for _ in x_bounds.0..x_bounds.1 {
        print!("##");
    }
    println!("#");

    for y in y_bounds.0..y_bounds.1 {
        print!("#");
        for x in x_bounds.0..x_bounds.1 {
            let mut room = '.';
            if x == p.0 && y == p.1 {
                room = 'X';
            }
            if doors.contains(&Door::new((x, y), (x + 1, y))) {
                print!("{}|", room);
            } else {
                print!("{}#", room);
            }
        }
        println!("");
        print!("#");
        for x in x_bounds.0..x_bounds.1 {
            if doors.contains(&Door::new((x, y), (x, y + 1))) {
                print!("-#");
            } else {
                print!("##");
            }
        }
        println!();
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Door {
    from: Point,
    to: Point,
}

impl Door {
    fn new(a: Point, b: Point) -> Self {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => Door { from: a, to: b },
            _ => Door { from: b, to: a },
        }
    }
}

type Point = (isize, isize);
type UPoint = (usize, usize);
type BfsMap = Vec<Vec<Option<(usize)>>>;

enum Token {
    Direction(Direction),
    GroupStart,
    Or,
    GroupEnd,
}

impl Token {
    fn parse(c: char) -> Option<Token> {
        match c {
            'N' | 'S' | 'E' | 'W' => Some(Token::Direction(Direction::parse(c))),
            '(' => Some(Token::GroupStart),
            '|' => Some(Token::Or),
            ')' => Some(Token::GroupEnd),
            _ => None,
        }
    }
}

enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn walk(&self, p: &Point) -> Point {
        match self {
            Direction::N => (p.0, p.1 - 1),
            Direction::S => (p.0, p.1 + 1),
            Direction::E => (p.0 + 1, p.1),
            Direction::W => (p.0 - 1, p.1),
        }
    }

    fn parse(c: char) -> Direction {
        match c {
            'N' => Direction::N,
            'S' => Direction::S,
            'E' => Direction::E,
            'W' => Direction::W,
            _ => panic!(),
        }
    }
}
