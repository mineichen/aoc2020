pub mod lib;

fn main() {
    let memory = lib::load("day14/input.txt");
    println!("Sum: {}", memory.iter().sum::<u64>());
}