use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn load_rules() -> impl Iterator<Item=(OccurenceRule, String)> {
    let f = File::open("day2/input.txt").unwrap();
    let f = BufReader::new(f);
    f.lines()
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
            let rule = OccurenceRule { first: min, second: max, char };
            chars.next();
            chars.next();

            (rule, chars.collect::<String>())
        })
    //std::iter::empty()
    
}


#[derive(Debug)]
pub struct OccurenceRule {
    pub first: usize,
    pub second: usize,
    pub char: char
}