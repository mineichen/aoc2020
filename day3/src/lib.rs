use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

#[derive(PartialEq)]
pub enum Symbol {
    Tree,
    OpenSquare
}

pub struct Col(String);

impl Col {
    pub fn iter_symbols(&self) -> impl Iterator<Item=Symbol> + '_ {
        self.0.chars().map(|c| {
            match c {
                '#' => Symbol::Tree,
                '.' => Symbol::OpenSquare,
                _ => panic!("Error")
            }
        })
    }
}

pub fn parse_input() -> impl Iterator<Item=Col> {
    let f = File::open("day3/input.txt").unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|line_result| {
            Col(line_result.unwrap())            
        })
}
