use crate::utils;
use regex::Regex;

pub fn day19(lines: &mut Vec<String>) {
    println!("Running Day 19 - a");

    let program = Program::new(lines);
    let mut emulator = Emulator::new();
    emulator.run_program(&program);

    println!("Value of Register 0 = {}", emulator.registers[0]);

    println!("Running Day 19 - b");

    emulator = Emulator::new();
    emulator.registers[0] = 1;
    // emulator.run_program(&program);

    println!("Value of Register 0 = {}", emulator.registers[0]);
}

struct Emulator {
    ip: usize,
    ticks: usize,
    registers: Registers,
}

impl Emulator {
    fn new() -> Self {
        Emulator {
            ip: 0,
            ticks: 0,
            registers: [0; 6],
        }
    }

    fn run_program(&mut self, program: &Program) {
        while
        /*0 <= self.ip &&*/
        self.ip < program.instructions.len() {
            self.run_instruction(program);
        }
    }

    fn run_instruction(&mut self, program: &Program) {
        self.ticks += 1;
        self.registers[program.ip_register] = self.ip;
        let i = &program.instructions[self.ip];
        i.op.op(&mut self.registers, i);
        self.ip = self.registers[program.ip_register];
        self.ip += 1;
    }
}

struct Program {
    ip_register: usize,
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(lines: &Vec<String>) -> Self {
        let rex = Regex::new(r"(\w{4}) (\d+) (\d+) (\d+)").unwrap();

        Program {
            ip_register: utils::parse(lines[0].split(' ').last().unwrap()),
            instructions: lines
                .iter()
                .skip(1)
                .map(|l| Program::parse_instruction(l, &rex))
                .collect(),
        }
    }

    fn parse_instruction(line: &String, rex: &Regex) -> Instruction {
        let c = rex.captures(line).unwrap();
        Instruction {
            op: Op::from_string(&c[1]),
            a: utils::parse(&c[2]),
            b: utils::parse(&c[3]),
            c: utils::parse(&c[4]),
        }
    }
}

type Registers = [usize; 6];

struct Instruction {
    op: Op,
    a: usize,
    b: usize,
    c: usize,
}

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
    fn from_string(s: &str) -> Op {
        match s {
            "addr" => Op::addr,
            "addi" => Op::addi,
            "mulr" => Op::mulr,
            "muli" => Op::muli,
            "banr" => Op::banr,
            "bani" => Op::bani,
            "borr" => Op::borr,
            "bori" => Op::bori,
            "setr" => Op::setr,
            "seti" => Op::seti,
            "gtir" => Op::gtir,
            "gtri" => Op::gtri,
            "gtrr" => Op::gtrr,
            "eqir" => Op::eqir,
            "eqri" => Op::eqri,
            "eqrr" => Op::eqrr,
            _ => panic!(),
        }
    }

    pub fn op(&self, r: &mut Registers, i: &Instruction) {
        let op_fn = match self {
            Op::addr => Op::fn_addr,
            Op::addi => Op::fn_addi,
            Op::mulr => Op::fn_mulr,
            Op::muli => Op::fn_muli,
            Op::banr => Op::fn_banr,
            Op::bani => Op::fn_bani,
            Op::borr => Op::fn_borr,
            Op::bori => Op::fn_bori,
            Op::setr => Op::fn_setr,
            Op::seti => Op::fn_seti,
            Op::gtir => Op::fn_gtir,
            Op::gtri => Op::fn_gtri,
            Op::gtrr => Op::fn_gtrr,
            Op::eqir => Op::fn_eqir,
            Op::eqri => Op::fn_eqri,
            Op::eqrr => Op::fn_eqrr,
        };
        op_fn(r, i);
    }

    fn fn_addr(r: &mut Registers, i: &Instruction) {
        r[i.c] = r[i.a] + r[i.b];
    }
    fn fn_addi(r: &mut Registers, i: &Instruction) {
        r[i.c] = r[i.a] + i.b;
    }
    fn fn_mulr(r: &mut Registers, i: &Instruction) {
        r[i.c] = r[i.a] * r[i.b];
    }
    fn fn_muli(r: &mut Registers, i: &Instruction) {
        r[i.c] = r[i.a] * i.b;
    }
    fn fn_banr(r: &mut Registers, i: &Instruction) {
        r[i.c] = r[i.a] & r[i.b];
    }
    fn fn_bani(r: &mut Registers, i: &Instruction) {
        r[i.c] = r[i.a] & i.b;
    }
    fn fn_borr(r: &mut Registers, i: &Instruction) {
        r[i.c] = r[i.a] | r[i.b];
    }
    fn fn_bori(r: &mut Registers, i: &Instruction) {
        r[i.c] = r[i.a] | i.b;
    }
    fn fn_setr(r: &mut Registers, i: &Instruction) {
        r[i.c] = r[i.a];
    }
    fn fn_seti(r: &mut Registers, i: &Instruction) {
        r[i.c] = i.a;
    }
    fn fn_gtir(r: &mut Registers, i: &Instruction) {
        if i.a > r[i.b] {
            r[i.c] = 1;
        } else {
            r[i.c] = 0;
        }
    }
    fn fn_gtri(r: &mut Registers, i: &Instruction) {
        if r[i.a] > i.b {
            r[i.c] = 1;
        } else {
            r[i.c] = 0;
        }
    }
    fn fn_gtrr(r: &mut Registers, i: &Instruction) {
        if r[i.a] > r[i.b] {
            r[i.c] = 1;
        } else {
            r[i.c] = 0;
        }
    }
    fn fn_eqir(r: &mut Registers, i: &Instruction) {
        if i.a == r[i.b] {
            r[i.c] = 1;
        } else {
            r[i.c] = 0;
        }
    }
    fn fn_eqri(r: &mut Registers, i: &Instruction) {
        if r[i.a] == i.b {
            r[i.c] = 1;
        } else {
            r[i.c] = 0;
        }
    }
    fn fn_eqrr(r: &mut Registers, i: &Instruction) {
        if r[i.a] == r[i.b] {
            r[i.c] = 1;
        } else {
            r[i.c] = 0;
        }
    }
}
