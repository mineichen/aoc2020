pub mod lib;

fn main() {
    println!(
        "Count non alergen ingredients: {}", 
        lib::count_harmless_ingredient_occurences("day21/input.txt")
    );
}
