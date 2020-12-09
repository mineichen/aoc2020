mod lib;

fn main() {
    if let Err(e) = lib::validate() {
        println!("Error for number {}", e);
    } else {
        println!("Everything is valid");
    }
}
