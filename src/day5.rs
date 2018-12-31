use crate::utils;

pub fn day5() -> std::io::Result<()> {
    println!("Running Day 5 - a");

    let lines = utils::readDay(5)?;

    let mut polymer = lines.get(0).unwrap().to_owned();
    // let mut polymer = "dabAcCaCBAcCcaDA".to_owned();

    // Optimization: All 'improved' polymers will still have _at least_ the same reactions
    // as the unimproved polymer. So, assign back, and let the improved polymers not
    // duplicate the same effort.
    polymer = reactPolymer(&polymer);

    println!("Remaining Polymer Units = {}", polymer.len());

    println!("Running Day 5 - b");

    let shortestPolymerLength = (0..26)
        .map(|i| (((i as u8) + 65) as char, ((i as u8) + 97) as char))
        .map(|t| improvePolymer(&polymer, t))
        .map(|p| reactPolymer(&p))
        .map(|p| p.len())
        .min_by(|a, b| a.cmp(&b))
        .unwrap();

    println!("Shortest Polymer Length = {}", shortestPolymerLength);

    Ok(())
}

fn reactPolymer(polymer: &String) -> String {
    let mut polymer = polymer.chars().map(|c| c as u8).collect::<Vec<u8>>();

    let mut idx;
    let mut removalMade = true;
    while removalMade {
        removalMade = false;
        idx = 0;
        while idx < polymer.len() - 1 {
            if (polymer[idx] as i32 - polymer[idx + 1] as i32).abs() == 32 {
                polymer.remove(idx);
                polymer.remove(idx);
                removalMade = true;
            } else {
                idx += 1;
            }
        }
    }

    polymer.iter().map(|c| *c as char).collect::<String>()
}

fn improvePolymer(polymer: &String, unitType: (char, char)) -> String {
    polymer
        .chars()
        .filter(|c| *c != unitType.0 && *c != unitType.1)
        .collect::<String>()
}
