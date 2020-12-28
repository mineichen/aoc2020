pub mod lib;

fn main() {
    let mut game = lib::load("day22/input.txt");
    game.solve_recursive();
    println!("Winner_score recursive: {}", game.calc_score())
}
