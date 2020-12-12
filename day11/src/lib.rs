use std::{fmt, fs::File};

pub fn load_puzzle_with_floor_surrounding<T: std::io::Read>(reader: T) -> Puzzle {
    let mut cells = Vec::new();
    let cells_ref = &mut cells;
    let cols = utils::LineReaderIterator::from_reader(
        reader, 
        move |line| {
            let len_before = cells_ref.len();
            cells_ref.extend(
                std::iter::once(CellState::Floor).chain(
                    line.trim().chars().map(|c| match c {
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

pub fn output_number_of_occupied_seats<T: Fn(&Puzzle, CellState, usize) -> CellState>(strategy: T) {
    let mut puzzle = load_puzzle_with_floor_surrounding(File::open("day11/input.txt").unwrap());
    println!("Input: {:?}", puzzle);
    while puzzle.apply_once(&strategy){
        print!(".");
    }
    print!("\n");
    let occupied_seats = puzzle.cells.iter()
        .filter(|c| c == &&CellState::Occupied)
        .count();
    println!("Stable number of occupied seats: {}", occupied_seats);
}

pub fn all_direction_neighbour_strategy(puzzle: &Puzzle, cur: CellState, pos: usize) -> CellState {
    let count_adjacent = puzzle.count_occupied_directional(pos);
    if cur == CellState::Occupied {
        if count_adjacent >= 5 { CellState::Available } else { CellState::Occupied }
    } else {
        debug_assert!(cur == CellState::Available);
        if count_adjacent == 0 { CellState::Occupied } else { CellState::Available }
    }
}

pub fn immediate_neighbour_strategy(puzzle: &Puzzle, cur: CellState, pos: usize) -> CellState {
    let cols = puzzle.cols;
    let count_adjacent = (puzzle.cells[pos-cols-1] == CellState::Occupied) as u8
        + (puzzle.cells[pos-cols] == CellState::Occupied) as u8
        + (puzzle.cells[pos-cols+1] == CellState::Occupied) as u8
        + (puzzle.cells[pos-1] == CellState::Occupied) as u8
        + (puzzle.cells[pos+1] == CellState::Occupied) as u8
        + (puzzle.cells[pos+cols-1] == CellState::Occupied) as u8
        + (puzzle.cells[pos+cols] == CellState::Occupied) as u8
        + (puzzle.cells[pos+cols+1] == CellState::Occupied) as u8;

    if cur == CellState::Occupied {
        if count_adjacent >= 4 { CellState::Available } else { CellState::Occupied }
    } else {
        debug_assert!(cur == CellState::Available);
        if count_adjacent == 0 { CellState::Occupied } else { CellState::Available }
    }
}

impl Puzzle {
    fn new(cells: Vec<CellState>, cols: usize, rows: usize) -> Self {
        let buffer = cells.clone();
        Puzzle {cells, buffer, cols, rows}
    }

    fn count_occupied_directional(&self, pos: usize) -> u8 {
        let x = pos % self.cols;
        let y = pos / self.cols;
        let cols = self.cols; 
        fn check_directional(mut pos: usize, cells: &[CellState], steps: usize, dir: i32) -> bool {
            (0..steps).map(move|_| {
                    pos = (pos as i32 + dir) as usize;
                    cells[pos]
                })
                .filter(|f| f != &CellState::Floor)
                .map(|f| f == CellState::Occupied)
                .next().unwrap_or(false)
        }
        if cols < y + 1 {
            println!("Error");
        }
        let steps_right = cols - x - 1;
        let steps_bottom = self.rows - y - 1;
    
        
        (check_directional(pos, &self.cells, x.min(y), -1-cols as i32)) as u8
            + (check_directional(pos, &self.cells, y, -(cols as i32))) as u8
            + (check_directional(pos, &self.cells, steps_right.min(y), 1-cols as i32)) as u8
            + (check_directional(pos, &self.cells, x, -1)) as u8
            + (check_directional(pos, &self.cells, steps_right, 1)) as u8
            + (check_directional(pos, &self.cells, x.min(steps_bottom), -1+cols as i32)) as u8
            + (check_directional(pos, &self.cells, steps_bottom, cols as i32)) as u8
            + (check_directional(pos, &self.cells, steps_right.min(steps_bottom), 1+cols as i32)) as u8
    }
    pub fn apply_once<T: Fn(&Puzzle, CellState, usize) -> CellState>(&mut self, strategy: &T) -> bool {
        let mut has_change = false;
        let mut pos = self.cols;
        for y in 1..self.rows-1 {
            pos += 1;
            for x in 1..(self.cols-1) {
                debug_assert_eq!(pos, x + y*self.cols);
                let cur = self.cells[pos];
                if cur != CellState::Floor {
                    let new_state = (strategy)(&self, cur, pos);
                    has_change |= cur != new_state;
                    *self.buffer.get_mut(pos).unwrap() = new_state;                    
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
    #[test]
    fn compare_part_1() {
        compare_strategy("../day11/testdata", &immediate_neighbour_strategy)
    }
    #[test]
    fn compare_part_2() {
        compare_strategy("../day11/part2_testdata", &all_direction_neighbour_strategy)
    }
    fn compare_strategy<T: Fn(&Puzzle, CellState, usize) -> CellState>(path: &str,strategy: &T) {
        let mut paths = (0..usize::max_value()).map(|i| {
            let mut path = PathBuf::from(path);
            path.push(format!("{}.txt", i));
            path
        }).take_while(|p| p.exists());

        let path = paths.next().unwrap();
        let mut prev = load_puzzle_with_floor_surrounding(File::open(path).unwrap());

        for path in paths {
            let puzzle = load_puzzle_with_floor_surrounding(File::open(path).unwrap());
            prev.apply_once(strategy);
            assert_eq!(prev, puzzle);
        }
    }

    #[test]
    fn count_directional_8_test() {
        let data = ".......#.
        ...#.....
        .#.......
        .........
        ..#L....#
        ....#....
        .........
        #........
        .........
        ...#.....";
        count_around_available(8, 5*11+4, data);
    } 
    #[test]
    fn count_directional_0_test() {
        let data = ".##.##.
        #.#.#.#
        ##...##
        ...L...
        ##...##
        #.#.#.#
        .......
        .##.##.";
        count_around_available(0, 4*9+4, data);
    } 
    fn count_around_available(count: u8, pos: usize, data: &str) {
        let puzzle = load_puzzle_with_floor_surrounding(std::io::Cursor::new(data));
        let value_at_pos = puzzle.cells[pos];
        assert_eq!(value_at_pos, CellState::Available);
        assert_eq!(count, puzzle.count_occupied_directional(pos));
    }
}