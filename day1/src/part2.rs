use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("day1/input.txt")?;
    let f = BufReader::new(f);
    let numbers = f
        .lines()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let (low, medium, high) = eval(numbers)?;

    println!(
        "Solution: {} * {} * {} = {}",
        low,
        medium,
        high,
        low * medium * high
    );
    Ok(())
}

/// Puzzle result: 324 * 390 * 1306 = 165026160
fn eval(mut numbers: Vec<i32>) -> io::Result<(i32, i32, i32)> {
    numbers.sort();
    const DESIRED_SUM: i32 = 2020;

    let mut low_iter = numbers.iter();
    let mut low_offset = 0;
    while let Some(low) = low_iter.next() {
        low_offset += 1;
        let mut medium_iter = low_iter.clone().take_while(|y| low + *y < DESIRED_SUM);
        let mut medium_offset = low_offset;
        while let Some(medium) = medium_iter.next() {
            medium_offset += 1;
            let maybe_high = DESIRED_SUM - *low - *medium;
            if let Ok(_) = numbers[medium_offset..].binary_search(&maybe_high) {
                return Ok((*low, *medium, maybe_high));
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "No result"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_one() {
        assert_eq!((10, 20, 1990), eval(vec!(1990, 10, 20)).unwrap());
    }
    #[test]
    fn number_cannot_be_reused() {
        assert_eq!(true, eval(vec!(1)).is_err());
    }
}
