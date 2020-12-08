mod lib;

fn main() {
    let mut program = lib::parse("day8/input.txt");

    for (i, op) in program.instructions.iter().enumerate() {
        let mut changed_program = match op {
            lib::Instruction::Acc(_) => continue,
            lib::Instruction::Jmp(x) => {
                let mut p = program.clone();
                *p.instructions.get_mut(i).unwrap() = lib::Instruction::Nop(*x);
                p
            },
            lib::Instruction::Nop(x) => {
                let mut p = program.clone();
                *p.instructions.get_mut(i).unwrap() = lib::Instruction::Jmp(*x);
                p
            }
        };
        if let Ok(_) = changed_program.run() {
            println!("Programm terminated with acc={} after changing instruction at index {}", changed_program.accumulator, i);
            return;
        }
    }
    let r = program.run();
    println!("Programm exited with: {:?}", r);
    println!("Accumulator after loop-detection: {}", program.accumulator);    
}
