pub mod lib;

fn main() {
    let r: i64 = lib::line_results("day18/input.txt", lib::calculation_precedence).sum();
    println!("Sum of all lines advanced: {}", r);
}
