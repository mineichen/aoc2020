use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq)]
pub enum Symbol {
    Tree,
    OpenSquare,
}

pub struct Col(String);

impl Col {
    pub fn iter_symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.0.chars().map(|c| match c {
            '#' => Symbol::Tree,
            '.' => Symbol::OpenSquare,
            _ => panic!("Error"),
        })
    }
}

pub fn count_trees_for_slope(col_slope: usize, row_slope: usize) -> usize {
    let mut row = 0;
    let mut tree_count = 0;
    const COLUMNS: usize = 31;
    let mut col_iter = parse_input();

    while let Some(col) = col_iter.next() {
        let symbol = col.iter_symbols().skip(row % COLUMNS).next().unwrap();
        if symbol == Symbol::Tree {
            tree_count += 1;
        }
        row += col_slope;
        for _ in 1..row_slope {
            col_iter.next();
        }
    }
    tree_count
}

pub fn parse_input() -> impl Iterator<Item = Col> {
    let f = File::open("day3/input.txt").unwrap();
    let f = BufReader::new(f);
    f.lines().map(|line_result| Col(line_result.unwrap()))
}
