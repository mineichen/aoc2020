pub mod lib;

fn main() {    
    let memory = lib::load_v2("day14/input.txt");
    println!("Sum v2: {}", memory.values().sum::<u64>());
}
