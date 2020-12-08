mod lib;

fn main() {
    let mut program = lib::parse("day8/input.txt");
    let r = program.run();
    println!("Programm exited with: {:?}", r);
    println!("Accumulator after loop-detection: {}", program.accumulator);
}
