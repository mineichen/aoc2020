#[macro_use]
extern crate bitflags;

mod lib;

fn main() {
    println!("Valid passports: {}", lib::count_valid_passports(|_, _| Ok(true)));
}
