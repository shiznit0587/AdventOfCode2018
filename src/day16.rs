use crate::utils;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn day16(lines: &mut Vec<String>) {
    println!("Running Day 16 - a");

    let mut split = 0;
    for i in 0..lines.len() {
        if lines[i] == "" && lines[i] == lines[i + 1] && lines[i + 1] == lines[i + 2] {
            split = i;
            break;
        }
    }

    let input1 = &lines[..split + 1];
    let input2 = &lines[split + 3..];

    let rex = Regex::new(r"....... \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();

    let mut result = 0;
    let mut samples: Vec<(usize, Vec<Op>)> = Vec::new();

    let mut iter = input1.iter().take(split + 1);
    while let Some((before, instruction, after, _)) = iter.next_tuple::<(_, _, _, _)>() {
        let before = parse_registers(before, &rex);
        let instruction = parse_instruction(&instruction);
        let after = parse_registers(after, &rex);

        let behaving_ops = test_ops(&before, &instruction, &after);

        if behaving_ops.len() >= 3 {
            result += 1;
        }

        samples.push((instruction[0], behaving_ops));
    }

    println!("Samples behaving as 3+ opcodes = {}", result);

    println!("Running Day 16 - b");

    let op_codes = deduce_op_codes(&mut samples);

    let mut r = [0, 0, 0, 0];
    for instruction in input2 {
        let i = parse_instruction(instruction);
        op_codes[&i[0]].get_fn()(&mut r, &i);
    }

    println!("Program Results = {:?}", r);
}

fn parse_registers(registers: &String, rex: &Regex) -> [usize; 4] {
    let c = rex.captures(registers).unwrap();
    [
        utils::parse::<usize>(&c[1]),
        utils::parse::<usize>(&c[2]),
        utils::parse::<usize>(&c[3]),
        utils::parse::<usize>(&c[4]),
    ]
}

fn parse_instruction(instructions: &String) -> [usize; 4] {
    let i = instructions
        .split(' ')
        .map(|n| utils::parse::<usize>(&n))
        .collect_vec();
    [i[0], i[1], i[2], i[3]]
}

fn test_ops(before: &Registers, instruction: &Instruction, after: &Registers) -> Vec<Op> {
    Op::iter()
        .map(|op| (op, before.clone()))
        .map(|(op, mut r)| {
            op.get_fn()(&mut r, &instruction);
            (op, r)
        })
        .filter(|(_, r)| r == after)
        .map(|(op, _)| *op)
        .collect_vec()
}

fn deduce_op_codes(samples: &mut Vec<(usize, Vec<Op>)>) -> HashMap<usize, Op> {
    let mut op_codes = HashMap::new();

    loop {
        let mut found = None;

        for entry in samples.iter() {
            if entry.1.len() == 1 {
                found = Some((entry.0, entry.1[0]));
                // println!("Code {:?}", found);
                break;
            }
        }

        match found {
            Some((op_code, op)) => {
                op_codes.insert(op_code, op);

                for entry in samples.iter_mut() {
                    if entry.0 == op_code {
                        entry.1 = Vec::new();
                    } else {
                        let idx = entry.1.iter().position(|&o| o == op);
                        if idx.is_some() {
                            entry.1.remove(idx.unwrap());
                        }
                    }
                }
            }
            None => break,
        }
    }

    op_codes
}

fn addr(r: &mut Registers, i: &Instruction) {
    r[i[3]] = r[i[1]] + r[i[2]];
}
fn addi(r: &mut Registers, i: &Instruction) {
    r[i[3]] = r[i[1]] + i[2];
}
fn mulr(r: &mut Registers, i: &Instruction) {
    r[i[3]] = r[i[1]] * r[i[2]];
}
fn muli(r: &mut Registers, i: &Instruction) {
    r[i[3]] = r[i[1]] * i[2];
}
fn banr(r: &mut Registers, i: &Instruction) {
    r[i[3]] = r[i[1]] & r[i[2]];
}
fn bani(r: &mut Registers, i: &Instruction) {
    r[i[3]] = r[i[1]] & i[2];
}
fn borr(r: &mut Registers, i: &Instruction) {
    r[i[3]] = r[i[1]] | r[i[2]];
}
fn bori(r: &mut Registers, i: &Instruction) {
    r[i[3]] = r[i[1]] | i[2];
}
fn setr(r: &mut Registers, i: &Instruction) {
    r[i[3]] = r[i[1]];
}
fn seti(r: &mut Registers, i: &Instruction) {
    r[i[3]] = i[1];
}
fn gtir(r: &mut Registers, i: &Instruction) {
    if i[1] > r[i[2]] {
        r[i[3]] = 1;
    } else {
        r[i[3]] = 0;
    }
}
fn gtri(r: &mut Registers, i: &Instruction) {
    if r[i[1]] > i[2] {
        r[i[3]] = 1;
    } else {
        r[i[3]] = 0;
    }
}
fn gtrr(r: &mut Registers, i: &Instruction) {
    if r[i[1]] > r[i[2]] {
        r[i[3]] = 1;
    } else {
        r[i[3]] = 0;
    }
}
fn eqir(r: &mut Registers, i: &Instruction) {
    if i[1] == r[i[2]] {
        r[i[3]] = 1;
    } else {
        r[i[3]] = 0;
    }
}
fn eqri(r: &mut Registers, i: &Instruction) {
    if r[i[1]] == i[2] {
        r[i[3]] = 1;
    } else {
        r[i[3]] = 0;
    }
}
fn eqrr(r: &mut Registers, i: &Instruction) {
    if r[i[1]] == r[i[2]] {
        r[i[3]] = 1;
    } else {
        r[i[3]] = 0;
    }
}

type Registers = [usize; 4];
type Instruction = Registers;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Op {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

impl Op {
    pub fn iter() -> std::slice::Iter<'static, Op> {
        static OPS: [Op; 16] = [
            Op::addr,
            Op::addi,
            Op::mulr,
            Op::muli,
            Op::banr,
            Op::bani,
            Op::borr,
            Op::bori,
            Op::setr,
            Op::seti,
            Op::gtir,
            Op::gtri,
            Op::gtrr,
            Op::eqir,
            Op::eqri,
            Op::eqrr,
        ];
        OPS.into_iter()
    }

    fn get_fn(&self) -> fn(&mut Registers, &Instruction) {
        match self {
            Op::addr => addr,
            Op::addi => addi,
            Op::mulr => mulr,
            Op::muli => muli,
            Op::banr => banr,
            Op::bani => bani,
            Op::borr => borr,
            Op::bori => bori,
            Op::setr => setr,
            Op::seti => seti,
            Op::gtir => gtir,
            Op::gtri => gtri,
            Op::gtrr => gtrr,
            Op::eqir => eqir,
            Op::eqri => eqri,
            Op::eqrr => eqrr,
        }
    }
}
