pub mod lib;

fn main() {
    let result = lib::resolve_step(vec!(7,14,0,17,11,1,2), 2020);
    println!("At 2020, an elve says: {}", result);
}
