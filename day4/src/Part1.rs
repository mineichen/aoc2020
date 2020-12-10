mod lib;

fn main() {
    println!(
        "Valid passports: {}",
        lib::count_valid_passports(|_, _| Ok(true))
    );
}
