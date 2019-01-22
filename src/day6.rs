use crate::utils;
use itertools::Itertools;
use pathfinding::utils::absdiff;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day6(lines: &mut Vec<String>) {
    println!("Running Day 6 - a");

    let rex = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();

    let coords = lines
        .iter()
        .map(|l| rex.captures(l).unwrap())
        .map(|c| (utils::parse(&c[1]), utils::parse(&c[2])))
        .collect_vec();

    let mut grid: HashMap<Point, usize> = HashMap::new();
    let mut totals: HashMap<Point, i32> = HashMap::new();

    let xBounds = (
        coords.iter().map(|c| c.0).min().unwrap(),
        coords.iter().map(|c| c.0).max().unwrap(),
    );

    let yBounds = (
        coords.iter().map(|c| c.1).min().unwrap(),
        coords.iter().map(|c| c.1).max().unwrap(),
    );

    for x in xBounds.0..xBounds.1 + 1 {
        for y in yBounds.0..yBounds.1 + 1 {
            let coord = (x, y);

            let manhattans = coords
                .iter()
                .enumerate()
                .map(|(i, c)| ((i, *c), calcManhattan(c, &coord)))
                .collect::<Vec<((usize, Point), i32)>>();

            let total = manhattans.iter().map(|(_, i)| *i).sum();
            totals.insert(coord, total);

            let min = manhattans.iter().map(|i| i.1).min().unwrap();

            let mins = manhattans
                .into_iter()
                .filter(|m| m.1 == min)
                .collect::<Vec<((usize, Point), i32)>>();

            // Don't count coordinates with ties
            if mins.len() > 1 {
                continue;
            }

            grid.insert(coord, (mins[0].0).0);
        }
    }

    // Exclude coords with infinite areas.
    let mut excludes: HashSet<usize> = HashSet::new();
    for x in xBounds.0..xBounds.1 + 1 {
        exclude((x, yBounds.0), &grid, &mut excludes);
        exclude((x, yBounds.1), &grid, &mut excludes);
    }
    for y in yBounds.0..yBounds.1 + 1 {
        exclude((xBounds.0, y), &grid, &mut excludes);
        exclude((xBounds.1, y), &grid, &mut excludes);
    }

    grid = grid
        .into_iter()
        .filter(|(_, i)| !excludes.contains(i))
        .collect::<HashMap<Point, usize>>();

    // Group grid entries by their closest coord id.
    let counts = grid.into_iter().map(|g| (g.1, g.0)).into_group_map();

    // Find the largest area.
    let max = counts.values().map(|v| v.len()).max().unwrap();

    println!("Largest Area = {}", max);

    println!("Running Day 6 - b");

    let regions = totals.values().filter(|&t| *t < 10000).count();

    println!("Region Size = {}", regions);
}

fn calcManhattan(a: &Point, b: &Point) -> i32 {
    absdiff(a.0, b.0) + absdiff(a.1, b.1)
}

fn exclude(coord: Point, grid: &HashMap<Point, usize>, excludes: &mut HashSet<usize>) {
    if grid.contains_key(&coord) {
        excludes.insert(grid[&coord]);
    }
}

type Point = (i32, i32);
