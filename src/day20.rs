use std::collections::HashSet;

pub fn day20(lines: &mut Vec<String>) {
    println!("Running Day 20 - a");

    let mut doors: HashSet<Door> = HashSet::new();
    let tokens: Vec<Token> = lines[0].chars().filter_map(|c| Token::parse(c)).collect();

    branch(&tokens, 0, (0, 0), &mut doors);

    _print(&doors);

    println!("Running Day 20 - b");
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
                let to = d.walk(&from);
                p = to;
                doors.insert(Door::new(from, to));
                cursor += 1;
            }
            Token::GroupStart => cursor = branch(tokens, cursor + 1, p, doors),
            Token::Or => cursor = branch(tokens, cursor + 1, start_p, doors),
            Token::GroupEnd => break,
        }
    }

    cursor + 1
}

fn _print(doors: &HashSet<Door>) {
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
            if x == 0 && y == 0 {
                room = 'X';
            }
            if doors.contains(&Door {
                from: (x, y),
                to: (x + 1, y),
            }) {
                print!("{}|", room);
            } else {
                print!("{}#", room);
            }
        }
        println!("");
        print!("#");
        for x in x_bounds.0..x_bounds.1 {
            if doors.contains(&Door {
                from: (x, y),
                to: (x, y + 1),
            }) {
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
