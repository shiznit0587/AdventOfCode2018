use crate::emulator::{Emulator, Program};

pub fn day19(lines: &mut Vec<String>) {
    println!("Running Day 19 - a");

    let program = Program::new(lines);
    let mut emulator = Emulator::new();
    emulator.run_program(&program);

    println!("Value of Register 0 = {}", emulator.registers[0]);

    println!("Running Day 19 - b");

    emulator = Emulator::new();
    emulator.registers[0] = 1;
    run_program_b(&mut emulator, &program);

    println!("Value of Register 0 = {}", emulator.registers[0]);
}

fn run_program_b(emulator: &mut Emulator, program: &Program) {
    // Stop executing instructions once we enter the core calculation loop.
    while emulator.ip != 1 {
        emulator.run_instruction(program);
    }
    // The program is iterating all pairs of numbers less than or equal to a value stored
    // in register 4, using registers 3 and 5. For each pair whose product is the target,
    // it adds the value from register 5 to register 0.
    // In essence, it's summing the divisors.
    let mut sum = 0;
    for i in 1..emulator.registers[4] + 1 {
        if emulator.registers[4] % i == 0 {
            sum += i;
        }
    }
    emulator.registers[0] = sum;
}
