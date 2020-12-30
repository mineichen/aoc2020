pub mod lib;

fn main() {
    let set = (0..100).fold(
        lib::load_flipped("day24/input.txt"), 
        |acc, _| lib::change_floor(acc)
    );
    println!("Number of flipped Tiles: {}", set.len());
}
