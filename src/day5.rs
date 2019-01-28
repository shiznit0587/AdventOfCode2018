pub fn day5(lines: &mut Vec<String>) {
    println!("Running Day 5 - a");

    let mut polymer = lines.get(0).unwrap().to_owned();
    // let mut polymer = "dabAcCaCBAcCcaDA".to_owned();

    // Optimization: All 'improved' polymers will still have _at least_ the same reactions
    // as the unimproved polymer. So, assign back, and let the improved polymers not
    // duplicate the same effort.
    polymer = react_polymer(&polymer);

    println!("Remaining Polymer Units = {}", polymer.len());

    println!("Running Day 5 - b");

    let shortestPolymerLength = (0..26)
        // .map(|i| (((i as u8) + 65) as char, ((i as u8) + 97) as char))
        // .map(|t| _improve_polymer_orig(&polymer, t))
        // .map(|p| _react_polymer_orig(&p))
        .map(|i| ((i as u8) + 65, (i as u8) + 97))
        .map(|u| react_improved_polymer(&polymer, u))
        .map(|p| p.len())
        .min()
        .unwrap();

    println!("Shortest Polymer Length = {}", shortestPolymerLength);
}

fn _react_polymer_orig(polymer: &String) -> String {
    let mut polymer = polymer.chars().map(|c| c as u8).collect::<Vec<u8>>();

    let mut idx;
    idx = 0;
    while idx < polymer.len() - 1 {
        if (polymer[idx] as i32 - polymer[idx + 1] as i32).abs() == 32 {
            polymer.remove(idx);
            polymer.remove(idx);
            if idx > 0 {
                idx -= 1;
            }
        } else {
            idx += 1;
        }
    }

    polymer.iter().map(|c| *c as char).collect::<String>()
}

fn react_polymer(polymer: &String) -> String {
    let mut polymer = polymer.chars().map(|c| c as u8).collect::<Vec<u8>>();
    let len = polymer.len();

    let mut c1 = 0;
    let mut c2 = 0;

    while c2 < len {
        if c2 == len {
            polymer[c1] = polymer[c2];
            c1 += 1;
        } else if c1 > 0 && will_react(polymer[c1 - 1], polymer[c2]) {
            c1 -= 1;
            c2 += 1;
        } else {
            polymer[c1] = polymer[c2];
            c1 += 1;
            c2 += 1;
        }
    }

    polymer
        .iter()
        .take(c1)
        .map(|c| *c as char)
        .collect::<String>()
}

fn react_improved_polymer(polymer: &String, remove: (u8, u8)) -> String {
    let mut polymer = polymer.chars().map(|c| c as u8).collect::<Vec<u8>>();
    let len = polymer.len();

    let mut c1 = 0;
    let mut c2 = 0;

    while c2 < len {
        if c2 == len {
            if !is_removed(polymer[c2], remove) {
                polymer[c1] = polymer[c2];
                c1 += 1;
            }
        } else if c1 > 0 && will_react(polymer[c1 - 1], polymer[c2]) {
            c1 -= 1;
            c2 += 1;
        } else if !is_removed(polymer[c2], remove) {
            polymer[c1] = polymer[c2];
            c1 += 1;
            c2 += 1;
        } else {
            c2 += 1;
        }
    }

    polymer
        .iter()
        .take(c1)
        .map(|c| *c as char)
        .collect::<String>()
}

#[inline]
fn will_react(a: u8, b: u8) -> bool {
    (a - b == 32 || b - a == 32)
}

#[inline]
fn is_removed(c: u8, unit: (u8, u8)) -> bool {
    (c == unit.0 || c == unit.1)
}

fn _improve_polymer_orig(polymer: &String, unitType: (char, char)) -> String {
    polymer
        .chars()
        .filter(|c| *c != unitType.0 && *c != unitType.1)
        .collect::<String>()
}
