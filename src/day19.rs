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
    emulator.run_program_b(&program);

    println!("Value of Register 0 = {}", emulator.registers[0]);
}

struct Emulator {
    ip: usize,
    registers: Registers,
}

impl Emulator {
    fn new() -> Self {
        Emulator {
            ip: 0,
            registers: [0; 6],
        }
    }

    fn run_program(&mut self, program: &Program) {
        while self.ip < program.instructions.len() {
            self.run_instruction(program);
        }
    }

    fn run_program_b(&mut self, program: &Program) {
        // Stop executing instructions once we enter the core calculation loop.
        while self.ip != 1 {
            self.run_instruction(program);
        }
        // The program is iterating all pairs of numbers less than or equal to a value stored
        // in register 4, using registers 3 and 5. For each pair whose product is the target,
        // it adds the value from register 3 to register 0.
        // In essence, it's summing the divisors.
        let mut sum = 0;
        for i in 1..self.registers[4] + 1 {
            if self.registers[4] % i == 0 {
                sum += i;
            }
        }
        self.registers[0] = sum;
    }

    fn run_instruction(&mut self, program: &Program) {
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

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.op, self.a, self.b, self.c)
    }
}

#[allow(non_camel_case_types)]
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

    fn to_string(&self) -> &str {
        match self {
            Op::addr => "addr",
            Op::addi => "addi",
            Op::mulr => "mulr",
            Op::muli => "muli",
            Op::banr => "banr",
            Op::bani => "bani",
            Op::borr => "borr",
            Op::bori => "bori",
            Op::setr => "setr",
            Op::seti => "seti",
            Op::gtir => "gtir",
            Op::gtri => "gtri",
            Op::gtrr => "gtrr",
            Op::eqir => "eqir",
            Op::eqri => "eqri",
            Op::eqrr => "eqrr",
        }
    }

    pub fn op(&self, r: &mut Registers, i: &Instruction) {
        r[i.c] = match self {
            Op::addr => r[i.a] + r[i.b],
            Op::addi => r[i.a] + i.b,
            Op::mulr => r[i.a] * r[i.b],
            Op::muli => r[i.a] * i.b,
            Op::banr => r[i.a] & r[i.b],
            Op::bani => r[i.a] & i.b,
            Op::borr => r[i.a] | r[i.b],
            Op::bori => r[i.a] | i.b,
            Op::setr => r[i.a],
            Op::seti => i.a,
            Op::gtir => {
                if i.a > r[i.b] {
                    1
                } else {
                    0
                }
            }
            Op::gtri => {
                if r[i.a] > i.b {
                    1
                } else {
                    0
                }
            }
            Op::gtrr => {
                if r[i.a] > r[i.b] {
                    1
                } else {
                    0
                }
            }
            Op::eqir => {
                if i.a == r[i.b] {
                    1
                } else {
                    0
                }
            }
            Op::eqri => {
                if r[i.a] == r[i.b] {
                    1
                } else {
                    0
                }
            }
            Op::eqrr => {
                if r[i.a] == r[i.b] {
                    1
                } else {
                    0
                }
            }
        };
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
