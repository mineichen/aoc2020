use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use core::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io")]
    Io(#[from] std::io::Error),
    #[error("parse")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Format exception")]
    Format(&'static str),
    #[error("Format exception {0}")]
    Split(char)
}

fn split_once(input: &str, delimiter: char) -> Result<(&str, &str), Error> {
    let mut first = input.splitn(2, delimiter);
    return Ok((
        first.next().unwrap_or(""), 
        first.next().ok_or(Error::Split(delimiter))?
    ));
}

pub struct FileReaderIterator {
    lines: std::io::Lines<BufReader<File>>
}

impl Iterator for FileReaderIterator {
    type Item = Result<(OccurenceRule, String), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|s| {
            match s {
                Ok(line) =>  {
                    let (min_text, rest) = split_once(&line, '-')?;
                    let (max_text, rest) = split_once(rest, ' ')?;                    

                    let min = min_text.parse()?;
                    let max = max_text.parse()?;
                    let mut chars = rest.char_indices();
                    let (_, char) = chars.next().ok_or(Error::Format("No char provided in role"))?;
                    let rule = OccurenceRule { first: min, second: max, char };

                    let (offset, _) = chars.skip(2).next().ok_or(Error::Format("No text provided in role"))?;
                    let text = rest.split_at(offset).1;
                    // Try to get rid of more
                    Ok((rule, text.to_string()))
                },
                Err(e) => Err(e.into())          
            }    
        })  
    }
}

pub fn load_rules() -> impl Iterator<Item=Result<(OccurenceRule, String), Error>> {
    let f = File::open("day2/input.txt").unwrap();
    let f = BufReader::new(f);
    FileReaderIterator { lines: f.lines() }
}

#[derive(Debug)]
pub struct OccurenceRule {
    pub first: usize,
    pub second: usize,
    pub char: char
}