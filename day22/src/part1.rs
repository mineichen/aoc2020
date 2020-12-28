pub mod lib;

fn main() {
    let mut game = lib::load("day22/input.txt");
    game.solve();
    println!("Winner_score: {}", game.calc_score())
}
