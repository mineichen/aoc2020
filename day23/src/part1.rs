pub mod lib;

fn main() {
    let mut game = lib::Game::new(vec!(1,3,7,8,2,6,4,9,5));
    println!("Game endstate: {}", game.solve(100));
}
