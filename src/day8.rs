use crate::utils;

pub fn day8() -> std::io::Result<()> {
    println!("Running Day 8 - a");

    let code = utils::read_day(8)?
        .first()
        .unwrap()
        .split(" ")
        .map(|s| utils::parse(s))
        .collect::<Vec<u32>>();

    let license = calc_license(&code);

    println!("License = {}", license);

    println!("Running Day 8 - b");

    Ok(())
}

fn calc_license(code: &Vec<u32>) -> u32 {
    let mut cursor = 0;
    let mut license = 0;
    visit_node(code, &mut cursor, &mut license);
    license
}

fn visit_node(code: &Vec<u32>, cursor: &mut usize, license: &mut u32) {
    if *cursor + 1 >= code.len() {
        return;
    }

    let header = (code[*cursor] as usize, code[*cursor + 1] as usize);
    *cursor += 2;

    for _ in 0..header.0 {
        visit_node(code, cursor, license);
    }

    for _ in 0..header.1 {
        *license += code[*cursor];
        *cursor += 1;
    }
}
