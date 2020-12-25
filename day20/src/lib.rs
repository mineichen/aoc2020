use std::{
    collections::{HashMap, hash_map::Entry},
    io::BufRead
};

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
pub fn load(path: &str) -> Puzzle {
    let reader = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let mut lines = reader.lines();

    let mut r = HashMap::new();
    while let Some(header) = lines.next() {
        let header = header.unwrap();
        let mut cells = vec![false; 100];
        let mut ctr = 0;
        let mut inner = lines.by_ref().take(10);
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
        debug_assert_eq!(100, ctr);
        let (_, parsed_id) = parse_header(&header).unwrap();
        
        r.insert(TileId(parsed_id), Tile { cells, width: 10, height: 10 });
        lines.next();
    }
    Puzzle { tiles: r } 
}

pub struct Tile {
    width: usize,
    height: usize,
    cells: Vec<bool>
}

impl Tile {
    fn edges_cw(&self) -> [u16;4] {
        let top = self.cells.iter().take(self.width).rev().fold(0, |acc, i| (acc << 1)+ *i as u16);
        let right = self.cells.iter().rev().step_by(self.width).fold(0, |acc, i| (acc << 1)+ *i as u16);
        let bottom = self.cells.iter().skip(self.width * (self.height - 1)).fold(0, |acc, i| (acc << 1)+ *i as u16);
        let left = self.cells.iter().step_by(self.width).fold(0, |acc, i| (acc << 1)+ *i as u16);
        [top, right, bottom, left]
    }
    fn edges_ccw(&self) -> [u16;4] {
        let top = self.cells.iter().take(self.width).fold(0, |acc, i| (acc << 1)+ *i as u16);
        let right = self.cells.iter().skip(self.width-1).step_by(self.width).fold(0, |acc, i| (acc << 1)+ *i as u16);
        let bottom = self.cells.iter().rev().take(self.width).fold(0, |acc, i| (acc << 1)+ *i as u16);
        let left = self.cells.iter().rev().skip(self.width-1).step_by(self.width).fold(0, |acc, i| (acc << 1)+ *i as u16);
        [top, right, bottom, left]
    }
}

pub struct Puzzle {
    tiles: HashMap<TileId, Tile>
}

impl Puzzle {
    pub fn find_corners(&self) -> [TileId;4] {
        let mut unmatched_edges = HashMap::new();

        for (id, tile) in self.tiles.iter() {        
            for edge in tile.edges_cw().iter().chain(tile.edges_ccw().iter()) {
                match unmatched_edges.entry(*edge) {
                    Entry::Occupied(x) => { x.remove_entry(); }
                    Entry::Vacant(x) => { x.insert(*id); }
                } 
            }
        }
        
        let mut result = [TileId(0),TileId(0),TileId(0),TileId(0)];
        let mut result_ctr = 0;
        let mut tile_id_ctr = HashMap::new();
        for tile_id in unmatched_edges.values() {        
            match tile_id_ctr.entry(*tile_id) {
                Entry::Occupied(mut x) => { 
                    let value = x.get_mut(); 
                    *value += 1;
                    if value == &4 {
                        result[result_ctr] = *tile_id;
                        result_ctr += 1;
                    }
                }
                Entry::Vacant(y) => { y.insert(1); }
            }
        }
        assert_eq!(4, result_ctr);

        result
    }
    fn combine(&mut self) -> usize {
        let mut lut = Vec::new();
        for (id, tile) in self.tiles.iter() {
            let ccw = tile.edges_ccw();
            for edge in ccw.iter() {
                lut.push((*id, *edge))
            }
        }
        lut.sort_by_key(|a| a.1);
        for (id, edge) in lut.iter() {
            println!("Edge: {}, TileId: {:?}", edge, id);
        }
        
/*
        let mut count = 0;
        let mut count_unique = 0;
        for (tile_id, tile) in self.tiles.iter_mut() {
            let cw = tile.edges_cw();
            let ccw = tile.edges_ccw();
            for (edge_index, (edge_cw, edge_ccw)) in cw.iter().zip(ccw.iter()).enumerate() {
                if let Ok(i) = lut.binary_search_by_key(edge_cw, |l| l.1) {
                    let range = full_range_by_key(&lut, i, |i| &i.1);
                    if range.clone().count() == 1 {
                        let neighbour = tile.neighbours.get_mut(edge_index).unwrap();
                        let new_neighbour = Some(lut[i].0);
                        assert_eq!(&None, neighbour, "{:?}", new_neighbour);
                        *neighbour = new_neighbour;
                        count += 1;
                    } else {
                        panic!("Expected no duplicates");
                    }
                } else {
                    count_unique += 1;
                    println!(
                        "{:10b}({:6}) {:?} (id: {:?})", 
                        edge_ccw,
                        edge_ccw,
                        full_range_by_key(
                            &lut, 
                            lut.binary_search_by_key(edge_ccw, |l| l.1).unwrap(),
                            |l| &l.1
                        ).count(),
                        tile_id
                    );
                    println!("{:10b}({:6})", edge_cw, edge_cw);
                }
            }
        }
        println!(
            "Connections: {}, Lut: {}, Tiles: {}, TwoEmpty: {:?}, unique: {}",
            count, 
            lut.len(),
            self.tiles.len(),
            self.tiles.values().filter(
                 |v| v.neighbours.iter().filter(|n| n == &&None).count() == 2
            ).count(),
            count_unique
        ); */
        1
    }
}

#[cfg(test)]
mod tests {
    
    use {super::*};
    #[test]
    fn test_input_demo() {
        let puzzle = load("../day20/test_input.txt");
        assert_eq!(
            20899048083289,
            puzzle.find_corners().iter().map(|i| i.0).product::<usize>()
        );
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
    fn reverse_edges() {
        let mut tile = Tile {cells: vec![false; 100], width: 10, height: 10};
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
}


