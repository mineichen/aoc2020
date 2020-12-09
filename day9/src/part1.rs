mod lib;

fn main() {
    if let Err(e) = lib::validate(&lib::read_cypher()) {
        println!("Error for number {}", e);
    } else {
        println!("Everything is valid");
    }
}
