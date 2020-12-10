use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io")]
    Io(#[from] std::io::Error),
    #[error("parse")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Format exception")]
    Format(&'static str),
    #[error("Format exception {0}")]
    Split(char),
}

pub struct LineReaderIterator<T, TFn: FnMut(&str) -> Result<T, Error>, TRead: Read> {
    reader: BufReader<TRead>,
    buffer: String,
    mapper: TFn,
}

impl<T, TFn: FnMut(&str) -> Result<T, Error>> LineReaderIterator<T, TFn, File> {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P, mapper: TFn) -> Self {
        LineReaderIterator::from_reader(File::open(path).unwrap(), mapper)
    }
}

impl<T, TFn: FnMut(&str) -> Result<T, Error>> LineReaderIterator<T, TFn, std::io::Cursor<String>> {
    pub fn from_string(i: String, mapper: TFn) -> Self {
        LineReaderIterator::from_reader(std::io::Cursor::new(i), mapper)
    }
}

impl<T, TFn: FnMut(&str) -> Result<T, Error>, TRead: Read> LineReaderIterator<T, TFn, TRead> {
    pub fn from_reader(read: TRead, mapper: TFn) -> Self {
        LineReaderIterator {
            reader: BufReader::new(read),
            buffer: String::new(),
            mapper,
        }
    }
}

impl<T, TFn: FnMut(&str) -> Result<T, Error>, TRead: Read> Iterator
    for LineReaderIterator<T, TFn, TRead>
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer) {
            Ok(n) if n > 0 => Some((self.mapper)(&self.buffer.trim_end())),
            Ok(_) => None,
            Err(e) => Some(Err(e.into())),
        }
    }
}

pub fn split_once(input: &str, delimiter: char) -> Result<(&str, &str), Error> {
    let mut first = input.splitn(2, delimiter);
    return Ok((
        first.next().unwrap_or(""),
        first.next().ok_or(Error::Split(delimiter))?,
    ));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
