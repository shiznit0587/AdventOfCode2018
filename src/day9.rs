use crate::utils;
use regex::Regex;
use std::collections::HashMap;

pub fn day9(lines: &mut Vec<String>) {
    println!("Running Day 9 - a");

    let rex = Regex::new(r"(\d+).* (\d+)").unwrap();
    let caps = rex.captures(lines.get(0).unwrap()).unwrap();
    let player_count: usize = utils::parse(&caps[1]);
    let last_marble: u32 = utils::parse(&caps[2]);

    let scores = solve_a(player_count, last_marble);
    print_winner(&scores);

    println!("Running Day 9 - b");

    let scores = solve_b(player_count, last_marble * 100);
    print_winner(&scores);
}

fn solve_a(player_count: usize, last_marble: u32) -> Vec<u32> {
    let mut cursor = 0;
    let mut cur_player = 0;
    let mut cur_marble = 1;

    let mut scores = vec![0; player_count];
    let mut marbles: Vec<u32> = Vec::with_capacity((last_marble + 1) as usize);
    marbles.push(0);

    while cur_marble <= last_marble {
        if cur_marble % 23 == 0 {
            scores[cur_player] += cur_marble;
            cursor = utils::wrap(cursor as isize - 7, marbles.len());
            scores[cur_player] += marbles.remove(cursor);
        } else {
            if cursor == 0 {
                marbles.push(cur_marble);
                cursor += 1;
            } else {
                cursor += 2;
                if cursor == marbles.len() {
                    marbles.push(cur_marble);
                } else {
                    cursor %= marbles.len();
                    marbles.insert(cursor, cur_marble);
                }
            }
        }

        // _print_marbles(&marbles, cur_player, cursor);
        cur_player = (cur_player + 1) % player_count;
        cur_marble += 1;
    }

    scores
}

fn solve_b(player_count: usize, last_marble: u32) -> Vec<u32> {
    let mut cursor = 0;
    let mut cur_player = 0;
    let mut cur_marble = 1;

    let mut scores: Vec<u32> = vec![0; player_count];
    let mut marbles: HashMap<u32, Marble> =
        HashMap::with_capacity(last_marble as usize / 23 * 21 + 2);

    for i in 0..last_marble + 1 {
        if i % 23 != 0 || i == 0 {
            marbles.insert(
                i,
                Marble {
                    id: i,
                    prev: None,
                    next: None,
                },
            );
        }
    }
    marbles.get_mut(&0).unwrap().prev = Some(0);
    marbles.get_mut(&0).unwrap().next = Some(0);

    while cur_marble <= last_marble {
        if cur_marble % 23 == 0 {
            scores[cur_player] += cur_marble;
            for _ in 0..7 {
                cursor = marbles[&cursor].prev.unwrap();
            }
            scores[cur_player] += cursor;

            let removed = marbles.remove(&cursor).unwrap();
            cursor = removed.next.unwrap();
            marbles.get_mut(&removed.prev.unwrap()).unwrap().next = Some(removed.next.unwrap());
            marbles.get_mut(&removed.next.unwrap()).unwrap().prev = Some(removed.prev.unwrap());
        } else {
            cursor = marbles[&cursor].next.unwrap();

            let mut inserted = marbles.remove(&cur_marble).unwrap();
            inserted.prev = Some(cursor);
            inserted.next = Some(marbles[&cursor].next.unwrap());
            marbles.insert(inserted.id, inserted);

            marbles
                .get_mut(&marbles[&cursor].next.unwrap())
                .unwrap()
                .prev = Some(cur_marble);
            marbles.get_mut(&cursor).unwrap().next = Some(cur_marble);

            cursor = marbles[&cursor].next.unwrap();
        }

        cur_player = (cur_player + 1) % player_count;
        cur_marble += 1;
    }

    scores
}

fn print_winner(scores: &Vec<u32>) {
    let winner = scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    println!("Winning Player = {}, Score = {}", (winner.0 + 1), winner.1);
}

fn _print_marbles(marbles: &Vec<u32>, cur_player: usize, cursor: usize) {
    println!(
        "[{}] {}",
        cur_player,
        marbles
            .iter()
            .enumerate()
            .map(|(i, m)| if i == cursor {
                format!("({:2})", m)
            } else {
                format!(" {:2} ", m)
            })
            .collect::<String>()
    );
}

struct Marble {
    id: u32,
    prev: Option<u32>,
    next: Option<u32>,
}
