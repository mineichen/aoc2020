mod lib;

fn main() {

    println!(
        "Hit {} on all routes!", 
        lib::count_trees_for_slope(1, 1)
         * lib::count_trees_for_slope(3, 1)
         * lib::count_trees_for_slope(5, 1)
         * lib::count_trees_for_slope(7, 1)
         * lib::count_trees_for_slope(1, 2)
    );
}
