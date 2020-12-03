mod lib;

fn main() {

    println!(
        "Hit {} trees on my way!", 
        lib::count_trees_for_slope(3, 1)
    );
}
