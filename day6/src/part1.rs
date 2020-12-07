mod lib;

fn main() {
    let summed : u32 = lib::get_grouped_flags()
        .map(|p| p.count_unique())
        .sum();
    
    println!("Sum: {:?}", summed);    
}
