mod lib;

fn main() {
    let max_id_seat = lib::list_seats().map(|s| s.id()).max().unwrap();
    println!("Max seat id is: {}", max_id_seat);
}
