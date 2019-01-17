use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day13(lines: &mut Vec<String>) {
    println!("Running Day 13 - a");

    let mut map: Map = vec![vec![TrackType::None; 150]; 150];
    let mut carts: Carts = BinaryHeap::new();

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
                '^' => carts.push(Cart::new((i, j), Direction::North)),
                'v' => carts.push(Cart::new((i, j), Direction::South)),
                '<' => carts.push(Cart::new((i, j), Direction::West)),
                '>' => carts.push(Cart::new((i, j), Direction::East)),
                _ => {}
            }
        }
    }

    // _print_map(&map, &carts);
    while let Some(c) = cycle(&map, &mut carts) {
        // print_map(&map, &c);
        carts = c;
    }

    println!("Running Day 13 - b");
}

fn cycle(map: &Map, carts: &mut Carts) -> Option<Carts> {
    let mut occupied = carts.iter().map(|c| c.location).collect::<HashSet<Point>>();
    let mut new_carts = BinaryHeap::new();

    while let Some(mut cart) = carts.pop() {
        occupied.remove(&cart.location);
        cart.travel(&map);

        if occupied.contains(&cart.location) {
            println!("Crash at location {:?}", cart.location);
            return None;
        }

        occupied.insert(cart.location);
        new_carts.push(cart);
    }

    Some(new_carts)
}

fn _print_map(map: &Map, carts: &Carts) {
    let carts = carts
        .iter()
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
type Carts = BinaryHeap<Cart>;

#[derive(Eq, PartialEq, Clone)]
struct Cart {
    location: Point,
    direction: Direction,
    intersect: IntersectionBehavior,
}

impl Cart {
    fn new(location: Point, dir: Direction) -> Self {
        Cart {
            location: location,
            direction: dir,
            intersect: IntersectionBehavior::TurnLeft,
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
        match self.direction {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
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
