pub fn list_seats() -> impl Iterator<Item=Seat> {
    utils::LineReaderIterator::from_file("day5/input.txt", decode_seat).map(Result::unwrap)
}

fn decode_seat(code: &str) -> Result<Seat, utils::Error> {
    let mut iter = code.chars();
    let mut row = 0;
    let mut shift = 128;
    for code in iter.by_ref().take(7) {
        shift = shift >> 1;
        if code == 'B' {
            row += shift;
        }
    }

    let mut col = 0;
    let mut shift = 8;
    for code in iter {
        shift = shift >> 1;
        if code == 'R' {
            col += shift;
        }
    }
    Ok(Seat::new(col, row))
}

#[derive(Debug)]
pub struct Seat {
    pub row: u8,
    pub col: u8,
    pub id: u16
}

impl Seat {
    fn new(col: u8, row: u8) -> Self {
        Seat { col, row, id: row as u16 * 8 + col as u16 } 
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn decode_id() {
        let seat = super::decode_seat("BFFFBBFRRR").unwrap();
        assert_eq!(70, seat.row);
        assert_eq!(7, seat.col);
        assert_eq!(567, seat.id);
    }
}