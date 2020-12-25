pub mod lib;

fn main() {
    let puzzle = lib::load("day20/input.txt");
    let corners = puzzle.find_corners();
    println!(
        "Corners: {:?}, product: {}",
        &corners,
        corners.iter().map(|t| t.0).product::<usize>()
    );
}
