use crate::utils;

pub fn day8(lines: &mut Vec<String>) {
    println!("Running Day 8 - a");

    let code = lines
        .first()
        .unwrap()
        .split(" ")
        .map(|s| utils::parse(s))
        .collect::<Vec<u32>>();

    let licenses = calc_licenses(&code);

    println!("License = {}", licenses.0);

    println!("Running Day 8 - b");

    println!("License = {}", licenses.1);
}

fn calc_licenses(code: &Vec<u32>) -> (u32, u32) {
    let mut cursor = 0;
    let mut license_a = 0;
    let license_b = visit_node(code, &mut cursor, &mut license_a);
    (license_a, license_b)
}

fn visit_node(code: &Vec<u32>, cursor: &mut usize, license_a: &mut u32) -> u32 {
    let header = (code[*cursor] as usize, code[*cursor + 1] as usize);
    *cursor += 2;

    let mut child_values: Vec<u32> = Vec::with_capacity(header.0);
    for _ in 0..header.0 {
        child_values.push(visit_node(code, cursor, license_a));
    }

    let mut value = 0;
    for _ in 0..header.1 {
        let meta = code[*cursor];
        if header.0 == 0 {
            value += meta;
        } else if 0 < meta && meta <= child_values.len() as u32 {
            value += child_values[(meta - 1) as usize];
        }
        *license_a += meta;
        *cursor += 1;
    }

    value
}
