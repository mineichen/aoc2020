pub mod lib;

fn main() -> Result<(), utils::Error>{
    let text = std::fs::read_to_string("day13/input.txt")?;
    let mut lines = text.split('\n').into_iter();
    let num = lines.next().unwrap().parse::<u64>()?;

    let res = lines.next().unwrap().split(',')
        .filter_map(|i| i.parse::<u64>().ok())
        .fold((0, u64::max_value()), |acc, n| {
            let modulo = n - (num % n);
            if modulo < acc.1 {
                (n, modulo)
            } else {
                acc
            }
        });

    println!("Result close to {}: {}*{}={}", num, res.0, res.1, res.0 * res.1);

    Ok(())
}
