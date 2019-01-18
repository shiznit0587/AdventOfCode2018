use std::cmp::Ordering;
use std::collections::HashMap;

pub fn day13(lines: &mut Vec<String>) {
    println!("Running Day 13 - a");

    let mut map: Map = vec![vec![TrackType::None; 150]; 150];
    let mut carts: Vec<Cart> = Vec::new();

    let mut cart_id = 0;
    for (j, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            map[i][j] = match c {
                '|' | '^' | 'v' => TrackType::Vertical,
                '-' | '<' | '>' => TrackType::Horizontal,
                '/' => TrackType::ForwardSlash,
                '\\' => TrackType::BackSlash,
                '+' => TrackType::Intersection,
                _ => TrackType::None,
            };

            match c {
                '^' | 'v' | '<' | '>' => {
                    carts.insert(
                        cart_id,
                        Cart::new(cart_id, (i, j), Direction::from_char(c).unwrap()),
                    );
                    cart_id += 1;
                }
                _ => {}
            }
        }
    }

    // _print_map(&map, &carts);
    while carts.iter().filter(|c| c.alive).count() > 1 {
        cycle(&map, &mut carts);
        // _print_map(&map, &carts);
    }

    println!("Running Day 13 - b");

    println!(
        "Last Cart Location = {:?}",
        carts.iter().filter(|c| c.alive).next().unwrap().location
    );
}

fn cycle(map: &Map, carts: &mut Vec<Cart>) {
    let mut queue = carts.iter().filter(|c| c.alive).collect::<Vec<&Cart>>();
    queue.sort();
    let mut queue = queue.iter().map(|c| c.id).collect::<Vec<usize>>();

    let all_alive = queue.len() == carts.len();

    let mut occupied = carts
        .iter()
        .filter(|c| c.alive)
        .map(|c| (c.location, c.id))
        .collect::<HashMap<Point, usize>>();

    while let Some(cart_id) = queue.pop() {
        let cart = carts.get_mut(cart_id).unwrap();
        occupied.remove(&cart.location);
        cart.travel(&map);

        if occupied.contains_key(&cart.location) {
            if all_alive {
                println!("Crash at location {:?}", cart.location);
            }

            let crashed_id = *occupied.get(&cart.location).unwrap();
            carts.get_mut(crashed_id).unwrap().alive = false;
            carts.get_mut(cart_id).unwrap().alive = false;
        } else {
            occupied.insert(cart.location, cart_id);
        }
    }
}

fn _print_map(map: &Map, carts: &Vec<Cart>) {
    let carts = carts
        .iter()
        .filter(|c| c.alive)
        .map(|c| (c.location, c))
        .collect::<HashMap<Point, &Cart>>();

    for j in 0..150 {
        for i in 0..150 {
            if carts.contains_key(&(i, j)) {
                print!("{}", carts[&(i, j)]._to_char());
            } else {
                print!("{}", map[i][j]._to_char());
            }
        }
        println!();
    }
}

type Point = (usize, usize);
type Map = Vec<Vec<TrackType>>;

#[derive(Eq, PartialEq, Clone)]
struct Cart {
    id: usize,
    location: Point,
    direction: Direction,
    intersect: IntersectionBehavior,
    alive: bool,
}

impl Cart {
    fn new(id: usize, location: Point, dir: Direction) -> Self {
        Cart {
            id: id,
            location: location,
            direction: dir,
            intersect: IntersectionBehavior::TurnLeft,
            alive: true,
        }
    }

    fn travel(&mut self, map: &Map) {
        match self.direction {
            Direction::North => self.location.1 -= 1,
            Direction::South => self.location.1 += 1,
            Direction::East => self.location.0 += 1,
            Direction::West => self.location.0 -= 1,
        }

        match map[self.location.0][self.location.1] {
            TrackType::ForwardSlash => {
                self.direction = match self.direction {
                    Direction::North | Direction::South => self.direction.turn_right(),
                    Direction::East | Direction::West => self.direction.turn_left(),
                }
            }
            TrackType::BackSlash => {
                self.direction = match self.direction {
                    Direction::North | Direction::South => self.direction.turn_left(),
                    Direction::East | Direction::West => self.direction.turn_right(),
                }
            }
            TrackType::Intersection => {
                self.direction = match self.intersect {
                    IntersectionBehavior::TurnLeft => self.direction.turn_left(),
                    IntersectionBehavior::GoStraight => self.direction.clone(),
                    IntersectionBehavior::TurnRight => self.direction.turn_right(),
                };
                self.intersect = self.intersect.next();
            }
            _ => {}
        }
    }

    fn _to_char(&self) -> char {
        self.direction._to_char()
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        (other.location.1)
            .cmp(&self.location.1)
            .then(other.location.0.cmp(&self.location.0))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn _to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::North),
            'v' => Some(Direction::South),
            '>' => Some(Direction::East),
            '<' => Some(Direction::West),
            _ => None,
        }
    }
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
enum IntersectionBehavior {
    TurnLeft,
    GoStraight,
    TurnRight,
}

impl IntersectionBehavior {
    fn next(&self) -> Self {
        match self {
            IntersectionBehavior::TurnLeft => IntersectionBehavior::GoStraight,
            IntersectionBehavior::GoStraight => IntersectionBehavior::TurnRight,
            IntersectionBehavior::TurnRight => IntersectionBehavior::TurnLeft,
        }
    }
}

#[derive(Clone)]
enum TrackType {
    None,
    Vertical,
    Horizontal,
    ForwardSlash,
    BackSlash,
    Intersection,
}

impl TrackType {
    fn _to_char(&self) -> char {
        match self {
            TrackType::None => ' ',
            TrackType::Vertical => '|',
            TrackType::Horizontal => '-',
            TrackType::ForwardSlash => '/',
            TrackType::BackSlash => '\\',
            TrackType::Intersection => '+',
        }
    }
}
