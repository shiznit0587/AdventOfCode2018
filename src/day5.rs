use crate::utils;

pub fn day5() -> std::io::Result<()> {
    println!("Running Day 5 - a");

    let lines = utils::readDay(5)?;

    // This day has only one line.
    // But, it involves a lot of string manipulation
    // I should break it down into a vector or ints, and remove pairs that are 26 apart
    // I should continuously traverse the entire list doing this until it can't be done anymore

    let line = lines.get(0).unwrap();
    // let line = "dabAcCaCBAcCcaDA";

    let mut _input = line.chars().map(|c| c as u8).collect::<Vec<u8>>();

    // println!(
    // "input = {}",
    // _input.iter().map(|c| *c as char).collect::<String>()
    // );

    let mut idx: usize;
    let mut removalMade = true;
    while removalMade {
        removalMade = false;
        idx = 0;
        while idx < _input.len() - 1 {
            if (_input[idx] as i32 - _input[idx + 1] as i32).abs() == 32 {
                // println!(
                // "Removing {} and {} at {}",
                // _input[idx] as char,
                // _input[idx + 1] as char,
                // idx
                // );
                _input.remove(idx);
                _input.remove(idx);

                // println!(
                // "input = {}",
                // _input.iter().map(|c| *c as char).collect::<String>()
                // );
                removalMade = true;
            } else {
                idx += 1;
            }
        }
    }

    // println!(
    // "input = {}",
    // _input.iter().map(|c| *c as char).collect::<String>()
    // );

    println!("Remaining Polymer Units = {}", _input.len());

    println!("Running Day 5 - b");

    Ok(())
}
