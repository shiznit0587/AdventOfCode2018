use crate::utils;
use regex::Regex;

pub fn day9(lines: &mut Vec<String>) {
    println!("Running Day 9 - a");

    let rex = Regex::new(r"(\d+).* (\d+)").unwrap();
    let caps = rex.captures(lines.get(0).unwrap()).unwrap();
    let player_count: usize = utils::parse(&caps[1]);
    let last_marble: usize = utils::parse(&caps[2]);

    let scores = solve_a(player_count, last_marble);
    print_winner(&scores);

    println!("Running Day 9 - b");

    let scores = solve_b(player_count, last_marble * 100);
    print_winner(&scores);
}

fn solve_a(player_count: usize, last_marble: usize) -> Vec<usize> {
    let mut cursor = 0;
    let mut cur_player = 0;
    let mut cur_marble = 1;

    let mut scores = vec![0; player_count];
    let mut marbles: Vec<usize> = Vec::with_capacity((last_marble + 1) as usize);
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

fn solve_b(player_count: usize, last_marble: usize) -> Vec<usize> {
    let mut cursor = 0;
    let mut cur_player = 0;
    let mut cur_marble = 1;

    let mut scores: Vec<usize> = vec![0; player_count];
    let mut marbles: Vec<Marble> = vec![
        Marble {
            prev: None,
            next: None
        };
        (last_marble + 1) as usize
    ];

    marbles.get_mut(0).unwrap().prev = Some(0);
    marbles.get_mut(0).unwrap().next = Some(0);

    while cur_marble <= last_marble {
        if cur_marble % 23 == 0 {
            scores[cur_player] += cur_marble;
            for _ in 0..7 {
                cursor = marbles[cursor].prev.unwrap();
            }
            scores[cur_player] += cursor;

            let prev = marbles.get(cursor).unwrap().prev.unwrap();
            let next = marbles.get(cursor).unwrap().next.unwrap();

            cursor = next;

            marbles.get_mut(prev).unwrap().next = Some(next);
            marbles.get_mut(next).unwrap().prev = Some(prev);
        } else {
            cursor = marbles[cursor].next.unwrap();
            let next = marbles.get(cursor).unwrap().next.unwrap();

            let mut inserted = marbles.get_mut(cur_marble).unwrap();
            inserted.prev = Some(cursor);
            inserted.next = Some(next);

            marbles.get_mut(next).unwrap().prev = Some(cur_marble);
            marbles.get_mut(cursor).unwrap().next = Some(cur_marble);

            cursor = marbles[cursor].next.unwrap();
        }

        cur_player = (cur_player + 1) % player_count;
        cur_marble += 1;
    }

    scores
}

fn print_winner(scores: &Vec<usize>) {
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

#[derive(Clone)]
struct Marble {
    prev: Option<usize>,
    next: Option<usize>,
}
