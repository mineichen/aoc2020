use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("day1/input.txt")?;
    let f = BufReader::new(f);
    let mut numbers = f.lines()
        .map(|s| s.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    numbers.sort();
    let mut iterator = numbers.iter();
    let mut high = *iterator.next_back().unwrap();
    let mut low = *iterator.next().unwrap();
    const DESIRED_SUM: u32 = 2020;

    loop {
        let sum = high + low;
        if sum == DESIRED_SUM {
            println!("Solution: {} * {} = {}", low, high, high * low);
            return Ok(());
        } else if sum < DESIRED_SUM {
            low = *iterator.next().unwrap();
        } else {
            high = *iterator.next_back().unwrap();
        }
    }
}