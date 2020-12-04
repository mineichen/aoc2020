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
    lines: BufReader<File>,
    buffer: String,
    mapper: TFn
}

impl<T, TFn: Fn(&str) -> Result<T, Error>> LineReaderIterator<T, TFn> {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P, mapper: TFn) -> Self {
        let f = File::open(path).unwrap();
        let f = BufReader::new(f);
        LineReaderIterator { lines: f, buffer: String::new(), mapper: mapper }
    }
}

impl<T, TFn: Fn(&str) -> Result<T, Error>> Iterator for LineReaderIterator<T, TFn> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        match self.lines.read_line(&mut self.buffer) {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
