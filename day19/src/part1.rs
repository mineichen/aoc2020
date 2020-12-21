pub mod lib;

fn main() {
    let (rules, msgs) = lib::load("day19/input.txt");
    let count_valid = msgs.filter(|m| rules.check(&0, &m)).count();
    println!("Valid msgs: {}", count_valid);    
}
