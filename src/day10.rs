use crate::utils;
use regex::Regex;
use std::collections::HashSet;

pub fn day10(lines: &mut Vec<String>) {
    println!("Running Day 10 - a");

    let rex = Regex::new(r"\w+=< *(-?\d+), *(-?\d+)> \w+=< *(-?\d+), *(-?\d+)>").unwrap();

    let mut stars: Vec<Star> = Vec::new();

    for line in lines {
        let caps = rex.captures(line).unwrap();
        stars.push(Star {
            position: (utils::parse(&caps[1]), utils::parse(&caps[2])),
            velocity: (utils::parse(&caps[3]), utils::parse(&caps[4])),
        });
    }

    let mut min_area = 0x7FFFFFFFFFFFFFFF;
    let mut seconds = 0;

    loop {
        advance_sky(&mut stars);
        let sky_area = get_area(&stars);
        if sky_area < min_area {
            min_area = sky_area;
            seconds += 1;
        } else {
            reverse_sky(&mut stars);
            print_sky(&stars);
            break;
        }
    }

    println!("Running Day 10 - b");

    println!("Seconds = {}", seconds);
}

fn advance_sky(stars: &mut Vec<Star>) {
    for star in stars {
        star.position.0 += star.velocity.0;
        star.position.1 += star.velocity.1;
    }
}

fn reverse_sky(stars: &mut Vec<Star>) {
    for star in stars {
        star.position.0 -= star.velocity.0;
        star.position.1 -= star.velocity.1;
    }
}

fn print_sky(stars: &Vec<Star>) {
    let bounds = get_bounds(&stars);

    let mut sky: HashSet<(i32, i32)> = HashSet::new();

    for star in stars {
        let star = (star.position.0 - bounds.0, star.position.1 - bounds.1);
        sky.insert(star);
    }

    for j in 0..bounds.3 + 1 {
        let mut line: Vec<char> = vec!['.'; (bounds.2 + 1) as usize];
        for i in 0..bounds.2 + 1 {
            if sky.contains(&(i, j)) {
                line[i as usize] = '#';
            }
        }
        println!("{}", line.iter().collect::<String>());
    }

    println!("");
}

fn get_bounds(stars: &Vec<Star>) -> (i32, i32, i32, i32) {
    let xBounds = (
        stars.iter().map(|s| s.position.0).min().unwrap(),
        stars.iter().map(|s| s.position.0).max().unwrap(),
    );

    let yBounds = (
        stars.iter().map(|s| s.position.1).min().unwrap(),
        stars.iter().map(|s| s.position.1).max().unwrap(),
    );

    (
        xBounds.0,
        yBounds.0,
        xBounds.1 - xBounds.0,
        yBounds.1 - yBounds.0,
    )
}

fn get_area(stars: &Vec<Star>) -> i64 {
    let bounds = get_bounds(stars);
    bounds.2 as i64 * bounds.3 as i64
}

struct Star {
    position: (i32, i32),
    velocity: (i32, i32),
}
