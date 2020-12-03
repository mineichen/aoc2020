mod lib;

fn main() {
    let mut row = 0;
    let mut tree_count = 0;
    const COLUMNS: usize = 31;

    for col in lib::parse_input() {
        let symbol = col.iter_symbols().skip(row % COLUMNS).next().unwrap();
        if symbol == lib::Symbol::Tree {
            tree_count += 1;
        }
        row += 3;
    }

    println!("Hit {} trees on my way!", tree_count);
}
