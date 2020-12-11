use std::fmt;

pub fn load_puzzle_with_floor_surrounding<T: AsRef<std::path::Path>>(path: T) -> Puzzle {
    let mut cells = Vec::new();
    let cells_ref = &mut cells;
    let cols = utils::LineReaderIterator::from_file(
        path, 
        move |line| {
            let len_before = cells_ref.len();
            cells_ref.extend(
                std::iter::once(CellState::Floor).chain(
                    line.chars().map(|c| match c {
                        '.' => CellState::Floor,
                        'L' => CellState::Available,
                        '#' => CellState::Occupied,
                        _ => panic!("Unknown cell")
                    }))
                    .chain(std::iter::once(CellState::Floor))
            );
            Ok(cells_ref.len() - len_before)
        }
    ).map(Result::unwrap).last().unwrap();
    let cells_len = cells.len();
    cells.extend(std::iter::repeat(CellState::Floor).take(cols*2));
    cells.copy_within(0..cells_len, cols);
    cells.copy_within(cells_len + cols..cells_len + cols*2, 0);
    let rows = cells.len() / cols;
    Puzzle::new(
        cells,
        cols,
        rows
    )
}

pub struct Puzzle {
    cols: usize,
    rows: usize,
    pub cells: Vec<CellState>,
    #[allow(dead_code)]
    buffer: Vec<CellState>
}
impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({}, {})", self.cols, self.rows)?;
        for row in self.cells.chunks(self.cols) {
            for v in row {
                f.write_fmt(format_args!("{}", v.as_symbol()))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        self.cols.eq(&other.cols)
            && self.cells.eq(&other.cells)
    }
}

impl Puzzle {
    fn new(cells: Vec<CellState>, cols: usize, rows: usize) -> Self {
        let buffer = cells.clone();
        Puzzle {cells, buffer, cols, rows}
    }

    pub fn apply_once(&mut self) -> bool {
        let mut has_change = false;
        let cols = self.cols;
        let mut pos = cols;
        for y in 1..self.rows-1 {
            pos += 1;
            for x in 1..(cols-1) {
                debug_assert_eq!(pos, x + y*cols);
                let cur = self.cells[pos];
                if cur != CellState::Floor {                
                    let count_adjacent = (self.cells[pos-cols-1] == CellState::Occupied) as u8
                        + (self.cells[pos-cols] == CellState::Occupied) as u8
                        + (self.cells[pos-cols+1] == CellState::Occupied) as u8
                        + (self.cells[pos-1] == CellState::Occupied) as u8
                        + (self.cells[pos+1] == CellState::Occupied) as u8
                        + (self.cells[pos+cols-1] == CellState::Occupied) as u8
                        + (self.cells[pos+cols] == CellState::Occupied) as u8
                        + (self.cells[pos+cols+1] == CellState::Occupied) as u8;
                    
                    *self.buffer.get_mut(pos).unwrap() = if cur == CellState::Occupied {
                        if count_adjacent >= 4 { has_change = true; CellState::Available} else {CellState::Occupied }
                    } else {
                        debug_assert!(cur == CellState::Available);
                        if count_adjacent == 0 {has_change = true; CellState::Occupied} else { CellState::Available}                        
                    };
                }

                pos += 1;
            }
            pos += 1
        }
        std::mem::swap(&mut self.buffer, &mut self.cells);
        has_change
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellState {
    Floor,
    Available,
    Occupied
}

impl CellState {
    fn as_symbol(&self) -> char {
        match self {
            CellState::Floor => '.',
            CellState::Occupied => '#',
            CellState::Available => 'L'
        }
    }
}
#[cfg(test)]
mod tests {
    use {super::*, std::path::PathBuf};
    static TEST_DATA_PATH: &str = "day11/testdata";
    #[test]
    fn compare_example() {
        let mut paths = (0..usize::max_value()).map(|i| {
            let mut path = PathBuf::from(TEST_DATA_PATH);
            path.push(format!("{}.txt", i));
            path
        }).take_while(|p| p.exists());

        let path = paths.next().unwrap();
        let mut prev = load_puzzle_with_floor_surrounding(path);

        for path in paths {
            let puzzle = load_puzzle_with_floor_surrounding(path);
            prev.apply_once();
            assert_eq!(prev, puzzle);
        }
    }
}