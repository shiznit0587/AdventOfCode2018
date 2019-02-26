use crate::utils;
use regex::Regex;

pub struct Emulator {
    pub ip: usize,
    pub registers: Registers,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            ip: 0,
            registers: [0; 6],
        }
    }

    pub fn run_program(&mut self, program: &Program) {
        while self.ip < program.instructions.len() {
            self.run_instruction(program);
        }
    }

    pub fn run_instruction(&mut self, program: &Program) {
        self.registers[program.ip_register] = self.ip;
        let i = &program.instructions[self.ip];
        i.op.op(&mut self.registers, &i.data);
        self.ip = self.registers[program.ip_register];
        self.ip += 1;
    }
}

pub struct Program {
    pub ip_register: usize,
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(lines: &Vec<String>) -> Self {
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
            data: InstructionData {
                a: utils::parse(&c[2]),
                b: utils::parse(&c[3]),
                c: utils::parse(&c[4]),
            },
        }
    }
}

pub type Registers = [usize; 6];

pub struct Instruction {
    pub op: Op,
    pub data: InstructionData,
}

pub struct InstructionData {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.op, self.data.a, self.data.b, self.data.c
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Op {
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

    pub fn op(&self, r: &mut Registers, i: &InstructionData) {
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
                if r[i.a] == i.b {
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
