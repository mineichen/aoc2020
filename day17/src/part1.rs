pub mod lib;

fn main() {
    let mut bag = lib::load("day17/input.txt");
    for _ in 0..6 {
        bag.step();
    }
    println!("Active in bag: {}", bag.count_active());
}
