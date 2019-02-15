use crate::utils;
use regex::Regex;
use std::collections::HashSet;

pub fn day17(lines: &mut Vec<String>) {
    println!("Running Day 17 - a");

    let rex = Regex::new(r"([xy])=(\d+), [xy]=(\d+)..(\d+)").unwrap();
    let mut clay = HashSet::new();
    for line in lines.iter() {
        let c = rex.captures(line).unwrap();
        let points: Vec<Point> = match &c[1] {
            "x" => (utils::parse(&c[3])..utils::parse::<usize>(&c[4]) + 1)
                .map(|y| (utils::parse(&c[2]), y))
                .collect(),
            "y" => (utils::parse(&c[3])..utils::parse::<usize>(&c[4]) + 1)
                .map(|x| (x, utils::parse(&c[2])))
                .collect(),
            _ => panic!(),
        };
        for p in points {
            clay.insert(p);
        }
    }

    let mut ground = Map::new(clay);

    let mut stack: Vec<Point> = Vec::new();
    stack.push((500, ground.y_bounds.0));

    while let Some(drop) = stack.pop() {
        stack.push(drop); // peek
        if ground[drop].is_dry() {
            ground[drop] = GroundState::Wet;
        }
        let mut queued = false;
        if ground[drop].is_wet() {
            match ground[(drop.0, drop.1 + 1)] {
                GroundState::Clay | GroundState::Water => {
                    if should_settle(&ground, drop) {
                        settle_row(&mut ground, drop);
                    }
                }
                GroundState::Dry => {
                    stack.push((drop.0, drop.1 + 1));
                    queued = true;
                }
                _ => {}
            }

            if !queued && ground[(drop.0, drop.1 + 1)].is_resting() {
                if ground[(drop.0 - 1, drop.1)].is_dry() {
                    stack.push((drop.0 - 1, drop.1));
                    queued = true;
                }
                if ground[(drop.0 + 1, drop.1)].is_dry() {
                    stack.push((drop.0 + 1, drop.1));
                    queued = true;
                }
            }
        }
        if !queued {
            stack.pop();
        }
    }

    let count = ground
        .tiles
        .iter()
        .flatten()
        .filter(|g| g.was_visited())
        .count();

    println!("Tiles Reached = {}", count);

    println!("Running Day 17 - b");

    let count = ground
        .tiles
        .iter()
        .flatten()
        .filter(|g| g.is_water())
        .count();

    println!("Retained Water = {}", count);
}

fn should_settle(ground: &Map, p: Point) -> bool {
    clay_left(ground, p) && clay_right(ground, p)
}

fn clay_left(ground: &Map, p: Point) -> bool {
    resting_below(ground, p)
        && match ground[(p.0 - 1, p.1)] {
            GroundState::Clay => true,
            GroundState::OutOfBounds => false,
            _ => clay_left(ground, (p.0 - 1, p.1)),
        }
}

fn clay_right(ground: &Map, p: Point) -> bool {
    resting_below(ground, p)
        && match ground[(p.0 + 1, p.1)] {
            GroundState::Clay => true,
            GroundState::OutOfBounds => false,
            _ => clay_right(ground, (p.0 + 1, p.1)),
        }
}

fn resting_below(ground: &Map, p: Point) -> bool {
    ground[(p.0, p.1 + 1)].is_resting()
}

fn settle_row(ground: &mut Map, p: Point) {
    if ground[p].can_settle() {
        ground[p] = GroundState::Water;
    }
    if ground[(p.0 - 1, p.1)].can_settle() {
        settle_row(ground, (p.0 - 1, p.1));
    }
    if ground[(p.0 + 1, p.1)].can_settle() {
        settle_row(ground, (p.0 + 1, p.1));
    }
}

type Point = (usize, usize);

#[derive(Clone)]
enum GroundState {
    Dry,
    Wet,
    Water,
    Clay,
    OutOfBounds,
}

impl GroundState {
    fn is_dry(&self) -> bool {
        match self {
            GroundState::Dry => true,
            _ => false,
        }
    }

    fn is_wet(&self) -> bool {
        match self {
            GroundState::Wet => true,
            _ => false,
        }
    }

    fn is_water(&self) -> bool {
        match self {
            GroundState::Water => true,
            _ => false,
        }
    }

    fn was_visited(&self) -> bool {
        match self {
            GroundState::Wet | GroundState::Water => true,
            _ => false,
        }
    }

    fn is_resting(&self) -> bool {
        match self {
            GroundState::Water | GroundState::Clay => true,
            _ => false,
        }
    }

    fn can_settle(&self) -> bool {
        match self {
            GroundState::Dry | GroundState::Wet => true,
            _ => false,
        }
    }

    fn _to_char(&self) -> char {
        match self {
            GroundState::Dry => '.',
            GroundState::Wet => '|',
            GroundState::Water => '~',
            GroundState::Clay => '#',
            _ => ' ',
        }
    }
}

struct Map {
    tiles: Vec<Vec<GroundState>>,
    x_bounds: Point,
    y_bounds: Point,
}

impl Map {
    fn new<IN>(clay: IN) -> Self
    where
        IN: IntoIterator<Item = Point>,
    {
        let clay: Vec<Point> = clay.into_iter().collect();
        let x_bounds = (
            clay.iter().map(|c| c.0).min().unwrap() - 1,
            clay.iter().map(|c| c.0).max().unwrap() + 1,
        );
        let y_bounds = (
            clay.iter().map(|c| c.1).min().unwrap(),
            clay.iter().map(|c| c.1).max().unwrap(),
        );
        let mut map = Map {
            tiles: vec![
                vec![GroundState::Dry; y_bounds.1 - y_bounds.0 + 1];
                x_bounds.1 - x_bounds.0 + 1
            ],
            x_bounds: x_bounds,
            y_bounds: y_bounds,
        };

        for p in clay {
            map[p] = GroundState::Clay;
        }

        map
    }
}

impl std::ops::Index<Point> for Map {
    type Output = GroundState;

    fn index(&self, i: Point) -> &GroundState {
        if self.x_bounds.0 <= i.0
            && i.0 <= self.x_bounds.1
            && self.y_bounds.0 <= i.1
            && i.1 <= self.y_bounds.1
        {
            &self.tiles[i.0 - self.x_bounds.0][i.1 - self.y_bounds.0]
        } else {
            &GroundState::OutOfBounds
        }
    }
}

impl std::ops::IndexMut<Point> for Map {
    fn index_mut(&mut self, i: Point) -> &mut GroundState {
        &mut self.tiles[i.0 - self.x_bounds.0][i.1 - self.y_bounds.0]
    }
}

impl Map {
    fn _print(&self) {
        for y in 0..(self.y_bounds.1 - self.y_bounds.0 + 1) {
            for x in 0..(self.x_bounds.1 - self.x_bounds.0 + 1) {
                print!("{}", self.tiles[x][y]._to_char());
            }
            println!();
        }
        println!();
    }
}
