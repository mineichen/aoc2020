use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("day2/input.txt")?;
    let f = BufReader::new(f);
    let mut rules = f.lines()
        .map(|s| {
            let line = s.unwrap();
            let mut chars = line.chars();
            let mut min = chars
                .by_ref()
                .take_while(|c| c != &'-')
                .collect::<String>()
                .parse()
                .unwrap();
            let mut max = chars
                .by_ref()
                .take_while(|c| c != &' ')
                .collect::<String>()
                .parse()
                .unwrap();
            let char = chars.next().unwrap();
            let rule = OccurenceRule { min, max, char };
            chars.next();
            chars.next();

            (rule, chars.collect::<String>())
        });

    
        let valid = rules.filter(|(rule, text)| rule.is_valid(&text)).count();
        println!("Valid passwords: {}", valid);
    Ok(())
}

#[derive(Debug)]
struct OccurenceRule {
    min: usize,
    max: usize,
    char: char
}

impl OccurenceRule {
    fn is_valid(&self, input: &str) -> bool {
        let cnt = input.chars()
            .filter(|c| c == &self.char)
            .count();
        self.min <= cnt 
            && self.max >= cnt
    }
}