use crate::utils;
use regex::Regex;

pub fn day9(lines: &mut Vec<String>) {
    println!("Running Day 9 - a");

    let rex = Regex::new(r"(\d+).* (\d+)").unwrap();
    let caps = rex.captures(lines.get(0).unwrap()).unwrap();
    let player_count: usize = utils::parse(&caps[1]);
    let last_marble: u32 = utils::parse(&caps[2]);

    let mut cursor = 0;
    let mut cur_player = 0;
    let mut cur_marble = 0;

    let mut scores = vec![0; player_count];
    let mut marbles: Vec<u32> = Vec::with_capacity((last_marble + 1) as usize);

    marbles.push(0);
    cur_marble += 1;

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

    print_winner(&scores);

    println!("Running Day 9 - b");
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
