pub mod lib;

fn main() {
    let mut puzzle = lib::load("day20/input.txt");
    let tile = puzzle.assemble();
    println!("Roughness: {:?}", tile.determine_roughness());
}
