pub mod lib;

fn main() {
    let mut game = lib::Game::new(vec!(1,3,7,8,2,6,4,9,5));
    game.solve(100);
    println!("Game endstate: {}", game.collect_from_label_1());
}
