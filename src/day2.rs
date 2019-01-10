use crate::utils;

pub fn day2() -> std::io::Result<()> {
    println!("Running Day 2 - a");

    let lines = utils::read_day(2)?;

    let mut twoCount = 0;
    let mut threeCount = 0;

    for line in lines.iter() {
        let mut counts = vec![0; 26];

        for c in line.chars() {
            let c = (c as usize) - 97;
            counts[c] += 1;
        }

        let mut twoFound = false;
        let mut threeFound = false;

        for count in counts.iter() {
            if !twoFound && *count == 2 {
                twoCount += 1;
                twoFound = true;
            }

            if !threeFound && *count == 3 {
                threeCount += 1;
                threeFound = true;
            }
        }
    }

    println!("Checksum = {}", twoCount * threeCount);
    println!("Running Day 2 - b");

    for idxA in 0..lines.len() {
        for idxB in idxA + 1..lines.len() {
            let mut mismatchIdx = Option::None;
            let mut completeMismatch = false;

            let lineA = &lines[idxA];
            let lineB = &lines[idxB];

            let charsIter = lineA.chars().zip(lineB.chars());

            for (idx, (charA, charB)) in charsIter.enumerate() {
                if charA != charB {
                    match mismatchIdx {
                        None => mismatchIdx = Some(idx),
                        Some(_) => completeMismatch = true,
                    }
                }
            }

            if completeMismatch {
                continue;
            }

            match mismatchIdx {
                None => continue,
                Some(idx) => {
                    // let lineC = format!("{}{}", &lineA[0..idx], &lineA[idx + 1..]);
                    let lineC = [&lineA[0..idx], &lineA[idx + 1..]].concat();
                    println!("Common Letters = {}", lineC);
                }
            }
        }
    }

    Ok(())
}
