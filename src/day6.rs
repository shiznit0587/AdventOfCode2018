use crate::utils;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn day6() -> std::io::Result<()> {
    println!("Running Day 6 - a");

    let rex = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();

    let coords = utils::readDay(6)?
        .iter()
        .map(|l| rex.captures(l).unwrap())
        .map(|c| (utils::tryParse_i32(&c[1]), utils::tryParse_i32(&c[2])))
        .collect_vec();

    let mut grid: HashMap<Point, usize> = HashMap::new();

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
    grid = grid
        .into_iter()
        .filter(|(_, i)| xBounds.0 < coords[*i].0 && coords[*i].0 < xBounds.1)
        .filter(|(_, i)| yBounds.0 < coords[*i].1 && coords[*i].1 < yBounds.1)
        .collect::<HashMap<Point, usize>>();

    // Group grid entries by their closest coord id.
    let counts = grid.into_iter().map(|g| (g.1, g.0)).into_group_map();

    // Find the largest area.
    let max = counts.values().map(|v| v.len()).max().unwrap();

    println!("Largest Area = {}", max);

    println!("Running Day 6 - b");

    Ok(())
}

fn calcManhattan(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

type Point = (i32, i32);
