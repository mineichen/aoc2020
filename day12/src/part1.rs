pub mod lib;

fn main() {
    let (x, y) = lib::calc(lib::load("day12/input.txt"));
    println!("X: {}, Y: {}, Sum: {}", x.round(), y.round(), (x.abs() + y.abs()).round());
}
