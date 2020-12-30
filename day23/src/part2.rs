pub mod lib;

use lib::GameTrait;

fn main() {
    let mut game = lib::FastGame::new(vec!(1,3,7,8,2,6,4,9,5).into_iter().chain(10..=1_000_000).collect());
    game.solve(10_000_000);
    println!("Fast: {}", game.multiply_two_after_label_1());
}
