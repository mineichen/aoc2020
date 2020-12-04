use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

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

pub struct LineReaderIterator<T, TFn: Fn(&str) -> Result<T, Error>> {
    reader: BufReader<File>,
    buffer: String,
    mapper: TFn
}

impl<T, TFn: Fn(&str) -> Result<T, Error>> LineReaderIterator<T, TFn> {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P, mapper: TFn) -> Self {
        LineReaderIterator { 
            reader: BufReader::new(File::open(path).unwrap()), 
            buffer: String::new(), 
            mapper
         }
    }
}

impl<T, TFn: Fn(&str) -> Result<T, Error>> Iterator for LineReaderIterator<T, TFn> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer) {
            Ok(n) if n > 0 =>  {
                Some((self.mapper)(&self.buffer.trim_end()))                
            },
            Ok(_) => {
                None
            }
            Err(e) => {
                Some(Err(e.into()))
            }
        }
    }
}

pub fn split_once(input: &str, delimiter: char) -> Result<(&str, &str), Error> {
    let mut first = input.splitn(2, delimiter);
    return Ok((
        first.next().unwrap_or(""), 
        first.next().ok_or(Error::Split(delimiter))?
    ));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
