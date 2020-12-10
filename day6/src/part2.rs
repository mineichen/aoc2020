mod lib;

fn main() {
    let summed: u32 = lib::AllPollIterator(lib::read_flags())
        .map(|p| p.count_unique())
        .sum();

    println!("Sum group agreement: {:?}", summed);
}
