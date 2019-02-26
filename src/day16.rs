use crate::emulator::{Emulator, Instruction, InstructionData, Op, Program, Registers};
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
        let instruction = parse_raw_instruction(&instruction);
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

    let mut emulator = Emulator::new();
    let program = Program {
        ip_register: 4,
        instructions: input2
            .iter()
            .map(|i| parse_instruction(i, &op_codes))
            .collect(),
    };
    emulator.run_program(&program);

    println!("Program Results = {:?}", emulator.registers);
}

fn parse_registers(registers: &String, rex: &Regex) -> Registers {
    let c = rex.captures(registers).unwrap();
    [
        utils::parse::<usize>(&c[1]),
        utils::parse::<usize>(&c[2]),
        utils::parse::<usize>(&c[3]),
        utils::parse::<usize>(&c[4]),
        0,
        0,
    ]
}

fn parse_raw_instruction(instruction: &String) -> [usize; 4] {
    let i = instruction
        .split(' ')
        .map(|n| utils::parse::<usize>(&n))
        .collect_vec();
    [i[0], i[1], i[2], i[3]]
}

fn parse_instruction(instruction: &String, op_codes: &HashMap<usize, Op>) -> Instruction {
    let i = instruction
        .split(' ')
        .map(|n| utils::parse::<usize>(&n))
        .collect_vec();
    Instruction {
        op: op_codes[&i[0]],
        data: InstructionData {
            a: i[1],
            b: i[2],
            c: i[3],
        },
    }
}

fn test_ops(before: &Registers, instruction: &[usize; 4], after: &Registers) -> Vec<Op> {
    Op::iter()
        .map(|op| (op, before.clone()))
        .map(|(op, mut r)| {
            op.op(
                &mut r,
                &InstructionData {
                    a: instruction[1],
                    b: instruction[2],
                    c: instruction[3],
                },
            );
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
