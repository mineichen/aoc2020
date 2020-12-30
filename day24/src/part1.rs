pub mod lib;

fn main() {
    let set = lib::load_flipped("day24/input.txt");
    println!("Number of flipped Tiles: {}", set.len());
}
