use crate::utils;

pub fn day14(lines: &mut Vec<String>) {
    println!("Running Day 14 - a");

    let target = utils::parse::<usize>(&lines[0]);

    let mut recipes: Vec<usize> = vec![0; target + 11];
    recipes[0] = 3;
    recipes[1] = 7;

    let mut count: usize = 2;
    let mut elves: (usize, usize) = (0, 1);

    while count < target + 10 {
        let sum = recipes[elves.0] + recipes[elves.1];
        let digits = split_digits(sum);
        for i in 0..digits.len() {
            recipes[count + i] = digits[i];
        }
        count += digits.len();

        elves.0 = (elves.0 + recipes[elves.0] + 1) % count;
        elves.1 = (elves.1 + recipes[elves.1] + 1) % count;
        // _print_recipes(&recipes, count, elves);
    }

    let scores = (target..target + 10)
        .map(|i| recipes[i])
        .map(|r| ((r + 48) as u8) as char)
        .collect::<String>();

    println!("Scores = {}", scores);

    println!("Running Day 14 - b");

    let target = split_digits(target);
    let mut recipes: Vec<usize> = vec![0; 1000000];
    recipes[0] = 3;
    recipes[1] = 7;

    count = 2;
    elves = (0, 1);
    let mut index = None;
    let mut digits = (None, None);

    while index.is_none() {
        if recipes.len() < count + 2 {
            recipes.extend_from_slice(&vec![0; 1000000]);
        }

        let sum = recipes[elves.0] + recipes[elves.1];
        split_digits_b(sum, &mut digits);
        let mut digit_count = 1;
        if digits.0.is_some() {
            recipes[count] = digits.0.unwrap();
            count += 1;
            digit_count += 1;
        }
        recipes[count] = digits.1.unwrap();
        count += 1;

        elves.0 = (elves.0 + recipes[elves.0] + 1) % count;
        elves.1 = (elves.1 + recipes[elves.1] + 1) % count;
        // _print_recipes(&recipes, count, elves);

        for i in 0..digit_count {
            let end = count - i;
            if end < target.len() {
                break;
            }
            let start = count - target.len() - i;

            if &recipes[start..end] == &target[..] {
                index = Some(start);
                break;
            }
        }
    }

    println!("Recipes Left of Input = {}", index.unwrap());
}

fn split_digits(mut num: usize) -> Vec<usize> {
    let mut digits = Vec::new();

    while num >= 10 {
        digits.push(num % 10);
        num /= 10;
    }

    digits.push(num);
    digits.reverse();

    digits
}

fn split_digits_b(num: usize, digits: &mut (Option<usize>, Option<usize>)) {
    digits.1 = Some(num % 10);
    if num >= 10 {
        digits.0 = Some(num / 10);
    } else {
        digits.0 = None;
    }
}

fn _print_recipes(recipes: &Vec<usize>, count: usize, elves: (usize, usize)) {
    for i in 0..count {
        if i == elves.0 {
            print!("({})", recipes[i]);
        } else if i == elves.1 {
            print!("[{}]", recipes[i]);
        } else {
            print!(" {} ", recipes[i]);
        }
    }
    println!();
}
