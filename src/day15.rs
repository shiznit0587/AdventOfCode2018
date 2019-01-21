pub fn day15(lines: &mut Vec<String>) {
    println!("Running Day 15 - a");

    let mut map = Map::new(&lines);

    map._print();

    let mut ended = false;
    let mut round = 0;
    while !ended {
        let mut queue = build_queue(&map);

        for entry in queue {
            // check if we're next to an enemy
            // If we are, we don't move.
            // Find all enemy units (targets)
            let targets = (match map[entry] {
                Tile::Goblin(_) => &map.elves,
                Tile::Elf(_) => &map.goblins,
                _ => panic!(),
            })
            .iter()
            .filter(|u| u.alive)
            .collect::<Vec<&Unit>>();

            // If any targets are already in range, don't move.

            // I think I can do that by getting the locations adjacent to the target, and
            // determining if entry's current location is in that list.
            let mut target_points = Vec::new();
            for t in targets.iter() {
                let mut points = Vec::new();
                if t.location.1 > 0 {
                    points.push((t.location.0, t.location.1 - 1));
                }
                if t.location.0 > 0 {
                    points.push((t.location.0 - 1, t.location.1));
                }
                if t.location.0 + 1 < map.width() {
                    points.push((t.location.0 + 1, t.location.1));
                }
                if t.location.1 + 1 < map.height() {
                    points.push((t.location.0, t.location.1 + 1));
                }
                for p in points {
                    if map[p].is_open() {
                        target_points.push(p);
                    }
                }
            }

            if target_points.contains(&(entry.0, entry.1)) {
                // Don't move, you're already in a target point
            }
        }

        round += 1;
        ended = true;
    }

    println!("Running Day 15 - b");
}

fn build_queue(map: &Map) -> Vec<Point> {
    let mut queue = Vec::new();
    for p in iproduct!(0..map.width(), 0..map.height()) {
        match map[p] {
            Tile::Goblin(_) | Tile::Elf(_) => queue.push(Point(p.0, p.1)),
            _ => (),
        }
    }
    queue
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

    fn is_unit(&self) -> bool {
        match self {
            Tile::Goblin(_) | Tile::Elf(_) => true,
            _ => false,
        }
    }

    fn is_wall(&self) -> bool {
        match self {
            Tile::Wall => true,
            _ => false,
        }
    }

    fn is_open(&self) -> bool {
        match self {
            Tile::Open => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone)]
struct Point(usize, usize);

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
                Tile::Goblin(i) => {
                    map.goblins.push(Unit::new(i, Point(p.1, p.0)));
                }
                Tile::Elf(i) => {
                    map.elves.push(Unit::new(i, Point(p.1, p.0)));
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

    fn _print(&self) {
        for line in self.tiles.iter() {
            println!("{}", line.iter().map(|t| t._to_char()).collect::<String>());
        }
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
    id: usize,
    location: Point,
    hp: isize,
    alive: bool,
}

impl Unit {
    fn new(id: usize, location: Point) -> Self {
        Unit {
            id: id,
            location: location,
            hp: 300,
            alive: true,
        }
    }

    fn identify_targets(&self, map: &Map) -> Vec<Tile> {
        match map.tiles[self.location.0][self.location.1] {
            Tile::Elf(_) => {
                // Get all remaining Goblins
                // I could also do this on map, based on 'alive' units
            }
            _ => {}
        }
        Vec::new()
    }
}
