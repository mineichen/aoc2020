pub mod lib;

fn main() {
    let (x, y) = lib::calc_waypoint(lib::load("day12/input.txt"));
    println!("Waypoints X: {}, Y: {}, Sum: {}", x.round(), y.round(), (x.abs() + y.abs()).round());
}
