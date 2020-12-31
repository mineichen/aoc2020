pub mod lib;

fn main() {
    let (mut rules, msgs) = lib::load("day19/input.txt");
    rules.replace_8_and_11().unwrap();
    let count_valid = msgs.filter(|m| rules.matches(std::iter::once(0), &m)).count();
    println!("Valid msgs: {}", count_valid);        
}
