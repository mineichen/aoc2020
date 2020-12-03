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
            let rule = OccurenceRule { first_pos: min, second_pos: max, char };
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
    first_pos: usize,
    second_pos: usize,
    char: char
}

impl OccurenceRule {
    pub fn is_valid(&self, input: &str) -> bool {
        self.is_char_valid(input, self.first_pos)
            ^ self.is_char_valid(input, self.second_pos)

    }
    fn is_char_valid(&self, input: &str, pos: usize) -> bool {
        input.chars()
            .skip(pos - 1)
            .map(|c| c == self.char)
            .next()
            .unwrap_or(false)
    }
}