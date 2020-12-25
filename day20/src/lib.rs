use std::{collections::{HashMap, hash_map::Entry}, fmt::Debug, io::BufRead};

fn parse_header(i: &str) -> nom::IResult<&str, usize>{
    nom::combinator::map(
        nom::sequence::tuple((
            nom::bytes::complete::tag("Tile "),
            nom::bytes::complete::take_while1(|c: char| c.is_digit(10))
         )),
        |(_,s)| std::str::FromStr::from_str(s).unwrap()
    )(i)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TileId(pub usize);

#[derive(Clone, Copy, Debug)]
pub struct Corner {
    pub tile_id: TileId,
    rotations: u8
}
impl Default for Corner {
    fn default() -> Self {
        Self { tile_id: TileId(0), rotations: 0}
    }
}

fn binary_search_all_by_key<'a, T, TFn: FnMut(&T) -> TMapped, TMapped: Ord>(input: &'a [T], key: &TMapped, mut mapper: TFn) -> &'a[T]{
    let mapper_ref = &mut mapper;
    if let Ok(i) = &input.binary_search_by_key(key, |a| (mapper_ref)(a)) {
        let off_min = input[0..*i].iter().rev().take_while(|v|&(mapper_ref)(v) == key).count();
        let off_max = input[i+1..].iter().take_while(|v|&(mapper_ref)(v) == key).count();
        &input[i-off_min..=i+off_max]
    } else {
        &input[0..0]
    }
}

pub fn load(path: &str) -> Puzzle {
    let reader = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let mut lines = reader.lines();

    let mut r = HashMap::new();
    while let Some(header) = lines.next() {
        let header = header.unwrap();
        
        let (_, parsed_id) = parse_header(&header).unwrap();
        r.insert(TileId(parsed_id), load_tile(lines.by_ref(), 10));
        lines.next();
    }
    
    Puzzle::new(r) 
}

fn load_tile(inner: impl Iterator<Item=Result<String, std::io::Error>>, side: usize) -> Tile<bool> {
    let mut cells = vec![false; side * side];
    let mut ctr = 0;
    let mut inner = inner.take(side);
    while let Some(row) = inner.next() {
        for c in row.unwrap().chars() {
            cells[ctr] = match c {
                '.' => false,
                '#' => true,
                c => {
                    eprintln!("Char: '{}'", c);
                    panic!("Unknown char");
                }
            };
            ctr += 1;
        }
    }
    debug_assert_eq!(side*side, ctr);
    Tile { cells, side: side}
}

const SEAMONSTER_WIDTH: usize = 20;
const SEAMONSTER_HEIGHT: usize = 3;
//                   # 
// #    ##    ##    ###
//  #  #  #  #  #  #  
fn seamonster_offsets(line: usize) -> [usize;15] {
    let line2 = line * 2;
    [18, 
    0+line, 5+line, 6+line, 11+line, 12+line, 17+line, 18+line, 19+line, 
    1+line2, 4+line2, 7+line2, 10+line2, 13+line2, 16+line2]
}

pub struct Tile<T> {
    side: usize,
    cells: Vec<T>
}

impl Debug for Tile<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        for line in self.cells.chunks(self.side) {
            for cell in line {
                write!(f, "{}", if *cell {"#"} else {"."})?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Tile<bool> {
    fn edges_cw(&self) -> [u16;4] {
        let top = self.cells.iter().take(self.side).rev().fold(0, |acc, i| (acc << 1)+ *i as u16);
        let right = self.cells.iter().rev().step_by(self.side).fold(0, |acc, i| (acc << 1)+ *i as u16);
        let bottom = self.cells.iter().skip(self.side * (self.side - 1)).fold(0, |acc, i| (acc << 1)+ *i as u16);
        let left = self.cells.iter().step_by(self.side).fold(0, |acc, i| (acc << 1)+ *i as u16);
        [top, right, bottom, left]
    }
    fn edges_ccw(&self) -> [u16;4] {
        let top = self.cells.iter().take(self.side).fold(0, |acc, i| (acc << 1)+ *i as u16);
        let right = self.cells.iter().skip(self.side-1).step_by(self.side).fold(0, |acc, i| (acc << 1)+ *i as u16);
        let bottom = self.cells.iter().rev().take(self.side).fold(0, |acc, i| (acc << 1)+ *i as u16);
        let left = self.cells.iter().rev().skip(self.side-1).step_by(self.side).fold(0, |acc, i| (acc << 1)+ *i as u16);
        [top, right, bottom, left]
    }
}
impl<T> Tile<T> {
    fn flip(&mut self) {
        for line in self.cells.chunks_mut(self.side) {
            let mut iter = line.iter_mut();
            while let Some(left) = iter.next() {
                if let Some(right) = iter.next_back() {
                    std::mem::swap(left, right);
                }
            }
        }
    }
    
    fn rotate_90(&mut self) {
        for y in 0..self.side/2 {
            for x in 0..(self.side + 1) / 2 {
    
                let rot_270_x = y;
                let rot_270_y = self.side - x - 1;
        
                let rot_90_x = self.side - y - 1;
                let rot_90_y = x;
        
                let rot_180_x = self.side - x - 1;
                let rot_180_y = self.side - y - 1;
                self.cells.swap(x + y * self.side, rot_90_x + rot_90_y * self.side);
                self.cells.swap(x + y * self.side, rot_180_x + rot_180_y * self.side);
                self.cells.swap(x + y * self.side, rot_270_x + rot_270_y * self.side);
            }
        }
    }
}

pub struct Puzzle {
    side: usize,
    tiles: HashMap<TileId, Tile<bool>>,
    ccw_lut: Vec<(u16, TileId)>
}



impl Puzzle {
    fn new(mut tiles: HashMap<TileId, Tile<bool>>) -> Self {
        let mut ccw_lut = Vec::with_capacity(tiles.len() * 4);
        for (id, tile) in tiles.iter_mut() {
            for edge in tile.edges_ccw().iter() {
                ccw_lut.push((*edge, *id));
            }
        }    
        ccw_lut.sort_by_key(|f| f.0);
        let side = ((tiles.len() as f64).sqrt() + 0.5) as usize;
        assert_eq!(side * side, tiles.len(), "Expected square puzzle");
        Self { tiles, side, ccw_lut}
    }
    pub fn find_corners(&self) -> [Corner;4] {
        let mut unmatched_edges = HashMap::new();

        for (id, tile) in self.tiles.iter() {        
            for (i, edge) in tile.edges_cw().iter().enumerate().chain(tile.edges_ccw().iter().enumerate()) {
                match unmatched_edges.entry(*edge) {
                    Entry::Occupied(x) => { x.remove_entry(); }
                    Entry::Vacant(x) => { x.insert((*id, i as u8)); }
                } 
            }
        }
        
        let mut result = [Corner::default();4];
        let mut result_ctr = 0;
        let mut tile_id_ctr = HashMap::<_, (usize, u8, u8)>::new();
        for (tile_id, edge_idx) in unmatched_edges.values() {        
            match tile_id_ctr.entry(*tile_id) {
                Entry::Occupied(mut x) => { 
                    let value = x.get_mut(); 
                    *value = (value.0 + 1, *edge_idx.min(&value.1), *edge_idx.max(&value.2));
                    if value.0 == 4 {
                        let rotations = 3 - if value.2 - value.1 == 1 
                            { value.1} else { 3 };
                        println!("{:?} min: {}, max: {}, rot: {}", tile_id, value.1, value.2, rotations);
                        result[result_ctr] = Corner { 
                            tile_id: *tile_id, 
                            rotations
                        };
                        result_ctr += 1;
                    }
                }
                Entry::Vacant(y) => { y.insert((1usize, *edge_idx, *edge_idx)); }
            }
        }
        assert_eq!(4, result_ctr);

        result
    }
    fn find_and_rotate_till_corner_match<'a>(&'a mut self, source_id: TileId, orientation: u8) -> TileId {
        if let Entry::Occupied(mut e) = self.tiles.entry(source_id) {
            let source = e.get_mut();
            let source_edge = source.edges_cw()[orientation as usize];
            let mut search_all = binary_search_all_by_key(self.ccw_lut.as_slice(), &source_edge, |c| c.0)
                .iter()
                .filter(|i| i.1 != source_id);
            let (other_id, other)  = if let Some((_, other_id)) = search_all.next()  {
                (*other_id, self.tiles.get_mut(&other_id).unwrap())
            } else {
                let reversed_edge = source.edges_ccw()[orientation as usize];
                let mut it = binary_search_all_by_key(self.ccw_lut.as_slice(), &reversed_edge, |c| c.0)
                    .iter()
                    .filter(|i| i.1 != source_id);
                let result = it.next();
                if result == None {
                    eprintln!("Either {0}{0:10b} nor {1}{1:10b}found in ccw-lut (self-id: {2:?}):", source_edge, reversed_edge, source_id);
                    for (edge, id) in self.ccw_lut.iter() {
                        eprintln!("{0:10b}({0}) -> {1:?}", edge, id);
                    }
                    panic!("Error on edge {:b} on Tile {:?}", source_edge, source_id);
                }
                assert_eq!(it.next(), None);
                let other_id = result.unwrap().1;
                let other = self.tiles.get_mut(&other_id).unwrap();
                other.flip();
                (other_id, other)
            };
            let opposite_orientation = (orientation + 2) % 4;
            let mut ctr = 0;
            while other.edges_ccw()[opposite_orientation as usize] != source_edge {
                other.rotate_90();
                ctr += 1;
                if ctr == 5 {
                    panic!(
                        "ccw: {:?}, cw: {:?}, requested: {}", 
                        other.edges_ccw(), 
                        other.edges_cw(),
                        source_edge
                    );
                }
            }
            
            return other_id;
        } 
        panic!("Unkwnown tile {:?}", source_id);        
    }
    pub fn assemble(&mut self) -> AssembledTile {
        let top_left_corner = self.find_corners()[0];
        let top_left_tile = self.tiles.get_mut(&top_left_corner.tile_id).unwrap();
        let cell_side = top_left_tile.side;
        
        for _ in 0..top_left_corner.rotations {
            top_left_tile.rotate_90();
        }

        println!("Top_left ({:?}): \n{:?}", top_left_corner.tile_id, top_left_tile);
        let mut matrix = vec!(top_left_corner.tile_id; self.side * self.side);
        // First column, left to right
        matrix.iter_mut().skip(1).take(self.side - 1).fold(top_left_corner.tile_id, |prev, cur| {
            let id = self.find_and_rotate_till_corner_match(prev, 1);
            *cur = id;
            println!("prev: {:?}, next: {:?}", prev, id);
            id
        });
        let mut cell = self.side;
        // Rest columns
        for _ in 0..self.side*(self.side-1) {
            let top = cell - self.side;
            matrix[cell] = self.find_and_rotate_till_corner_match(matrix[top], 2);
            cell+=1;
        }
        println!("Matrix: {:?}", matrix);

        let inner_side = cell_side - 2;
        let total_side = inner_side*self.side;
        let cells = total_side * total_side;
        let mut assembly_tile = Tile { cells: vec![false; cells], side: total_side };
        for (i, target) in assembly_tile.cells.iter_mut().enumerate() {
            let x = i % total_side;
            let y = i / total_side;
            let x_big = x / inner_side;
            let x_inner = x % inner_side;
            let y_big = y / inner_side;
            let y_inner = y % inner_side;
            let outer = x_big + y_big * self.side;
            let inner = x_inner+1 + (y_inner+1) * cell_side;
            
            *target = self.tiles[&matrix[outer]].cells[inner];
            /*if i < 64*self.side + 10 {
                eprintln!("Outer: {}, Inner: {}", outer, inner);
            }*/
        }
         
        AssembledTile(assembly_tile)
    }
}

pub struct AssembledTile(Tile<bool>);
impl AssembledTile {
    
    pub fn determine_roughness(mut self) -> Result<usize, &'static str> {
        let offsets = seamonster_offsets(self.0.side);
        let mut found_monsters = Tile { cells: vec![false;self.0.side*self.0.side], side: self.0.side };
        
        let mut found_any = false;
        for _ in 0..2 {
            for _ in 0..4 {
                let mut pos = 0;
                while pos < self.0.side*(self.0.side - SEAMONSTER_HEIGHT + 1){
                    for _ in 0..self.0.side - SEAMONSTER_WIDTH {
                        let found = offsets.iter().all(|i| self.0.cells[i+pos]);
                        if found {
                            found_any = true;
                            for i in offsets.iter() {
                                found_monsters.cells[i + pos] = true;
                            }                            
                        }
                        pos += 1;
                    }
                    pos += SEAMONSTER_WIDTH;
                }
                
                if found_any {
                    return Ok(self.0.cells.iter().filter(|i| **i).count()
                        - found_monsters.cells.iter().filter(|i| **i).count());
                }
                self.0.rotate_90();
            }
            self.0.flip();
        }
        Err("No seamonsters found")
    }
}
#[cfg(test)]
mod tests {
    
    use {super::*};
    #[test]
    fn test_input_assembly() {
        let mut puzzle = load("../day20/test_input.txt");
        let mut tile = puzzle.assemble();
        let expected = load_tile(std::io::BufReader::new(std::fs::File::open("../day20/test_assembled.txt").unwrap()).lines(), 24);
        for _ in 0..2 {
            for _ in 0..4 {
                if tile.0.cells == expected.cells {
                    return;
                }
                // eprintln!("Try: \n{:?}", tile.0);
                tile.0.rotate_90();

            }
            tile.0.flip();
        }
        panic!("No variation of assembled tile matches expected result");
    }
    
    #[test]
    fn test_input_demo() {
        let puzzle = load("../day20/test_input.txt");
        let corners = puzzle.find_corners();
        
        assert_eq!(
            20899048083289,
            corners.iter().map(|i| i.tile_id.0).product::<usize>()
        );
        let lut = corners.iter().map(|i| (i.tile_id, i)).collect::<HashMap<_,_>>();
        assert_eq!(1, lut[&TileId(1951)].rotations);
        assert_eq!(0, lut[&TileId(2971)].rotations);
    }
    #[test]
    fn test_first_of_test_input() {
        let puzzle = load("../day20/test_input.txt");
        let first_tile = &puzzle.tiles[&TileId(2311)];
        let ccw = first_tile.edges_ccw();
        let cw = first_tile.edges_cw();
        assert_eq!(ccw[0], 0b0011010010, "{:b}", ccw[0]);
        assert_eq!(cw[0], 0b0100101100, "{:b}", cw[0]);
        assert_eq!(ccw[1], 0b0001011001, "{:b}", ccw[1]);
        assert_eq!(cw[1], 0b1001101000, "{:b}", cw[1]);
        assert_eq!(ccw[2], 0b1110011100, "{:b}", ccw[2]);
        assert_eq!(cw[2], 0b0011100111, "{:b}", cw[2]);
        assert_eq!(ccw[3], 0b0100111110, "{:b}", ccw[3]);
        assert_eq!(cw[3], 0b0111110010, "{:b}", cw[3]);
    }

    #[test]
    fn flip_tile_3x3() {
        let mut tile = Tile { cells: (0..9).collect(), side: 3};
        let flipped = [2, 1, 0, 5, 4, 3, 8, 7, 6];
        tile.flip();
        for (a, b) in tile.cells.iter().zip(flipped.iter()) {
            assert_eq!(a,b);
        }
    }
    #[test]
    fn flip_tile_4x4() {
        let mut tile = Tile { cells: (0..16).collect(), side: 4};
        let flipped = [03, 02, 01, 00,
                       07, 06, 05, 04,
                       11, 10, 09, 08,
                       15, 14, 13, 12];
        tile.flip();
        for (a, b) in tile.cells.iter().zip(flipped.iter()) {
            assert_eq!(a,b);
        }
    }
    #[test]
    fn rotate_tile_3x3() {
        let mut tile = Tile { cells: (0..9).collect(), side: 3};
        let flipped = [6, 3, 0, 7, 4, 1, 8, 5, 2];
        tile.rotate_90();
        for (a, b) in tile.cells.iter().zip(flipped.iter()) {
            assert_eq!(a,b);
        }
    }
    #[test]
    fn rotate_tile_4x4() {
        let mut tile = Tile { cells: (0..16).collect(), side: 4};
        let flipped: [i32;16]  = [12, 08, 4, 0, 
                                  13, 09, 5, 1, 
                                  14, 10, 6, 2, 
                                  15, 11, 7, 3];
        tile.rotate_90();
        for (a, b) in tile.cells.iter().zip(flipped.iter()) {
            assert_eq!(a,b);
        }
    }

    #[test]
    fn reverse_edges() {
        let mut tile = Tile {cells: vec![false; 100], side: 10};
        tile.cells[1] = true; // top
        tile.cells[19] = true; // right
        tile.cells[98] = true; // bottom
        tile.cells[80] = true; // left
        let n = tile.edges_ccw();
        for neighbour in n.iter() {
            assert_eq!(256, *neighbour);
        }
        
        for neighbour in tile.edges_cw().iter() {
            assert_eq!(2, *neighbour);
        }
    }

    #[test]
    fn binary_search_all_test() {
        let n = [0, 2, 2, 3, 4, 4];
        assert_eq!(1, binary_search_all_by_key(&n[..], &0, |a| *a).len());
        assert_eq!(0, binary_search_all_by_key(&n[..], &1, |a| *a).len());
        assert_eq!(2, binary_search_all_by_key(&n[..], &2, |a| *a).len());
        assert_eq!(1, binary_search_all_by_key(&n[..], &3, |a| *a).len());
        assert_eq!(2, binary_search_all_by_key(&n[..], &4, |a| *a).len());
    }
}


