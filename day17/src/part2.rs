pub mod lib;

use std::collections::HashSet;

fn main() {
    let mut bag = Bag::new(lib::load("day17/input.txt"));
    for _ in 0..6 {
        bag.step();
    }
    println!("Active in bag: {}", bag.count_active());
}

pub struct Bag {
    active_cells: HashSet<Coordinate>,
    bounds_x: (i32, i32),
    bounds_y: (i32, i32),
    bounds_z: (i32, i32),
    bounds_w: (i32, i32),
}

impl Bag {
    pub fn new(active: impl Iterator<Item=(i32, i32)>) -> Self {
        let mut bounds_x = (i32::max_value(),i32::min_value());
        let mut bounds_y = (i32::max_value(),i32::min_value());
        
        let active_cells = active.map(|a| {
            bounds_x = ((a.0).min(bounds_x.0), (a.0).max(bounds_x.1));
            bounds_y = ((a.1).min(bounds_y.0), (a.1).max(bounds_y.1));
            Coordinate (a.0, a.1, 0, 0)
        }).collect();
        
        Bag { active_cells, bounds_x, bounds_y, bounds_z: (0,0), bounds_w: (0,0)}
    }
    pub fn print_slice(&self, z: i32, w: i32) {
        for y in self.bounds_y.0..=self.bounds_y.1 {
            for x in self.bounds_x.0..=self.bounds_x.1 {
                print!("{}", if self.active_cells.contains(&Coordinate(x, y, z, w)) { "#" } else { "." });
            }
            print!("\n");
        }
    }

    pub fn count_active(&self) -> usize {
        self.active_cells.len()
    }
    pub fn step(&mut self) {
        let mut clone = self.active_cells.clone();        
        let clone_ref = &mut clone;

        self.bounds_x = (self.bounds_x.0 - 1, self.bounds_x.1 + 1);
        self.bounds_y = (self.bounds_y.0 - 1, self.bounds_y.1 + 1);
        self.bounds_z = (self.bounds_z.0 - 1, self.bounds_z.1 + 1);
        self.bounds_w = (self.bounds_w.0 - 1, self.bounds_w.1 + 1);

        for c in self.iter_bounds() {
            let is_active = self.active_cells.contains(&c);
            let neighbours = c.neighbours().filter(|n| self.active_cells.contains(&n)).count();
            
            match (is_active, neighbours) {
                (true, 2) | (true, 3) => {},
                (true, _) => { assert!(clone_ref.remove(&c)); },
                (false, 3) => { assert!(clone_ref.insert(c)); },
                (false, _) => {}
            }
        }
        self.active_cells = clone;
    }
    fn iter_bounds(&self) -> impl Iterator<Item=Coordinate> + '_ {
        (self.bounds_w.0..=self.bounds_w.1).flat_map(
            move |w| (self.bounds_z.0..=self.bounds_z.1).flat_map(
                move |z| (self.bounds_y.0..=self.bounds_y.1).flat_map(
                    move |y| (self.bounds_x.0..=self.bounds_x.1).map(
                        move |x| Coordinate(x, y, z, w)
                    )
                )
            )
        )
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate(i32, i32, i32, i32);
impl Coordinate {
    fn neighbours(&self) -> impl Iterator<Item=Coordinate> + '_ {
        ((self.3-1)..=(self.3+1)).flat_map(
            move|w| ((self.2-1)..=(self.2+1)).flat_map(
                move |z| ((self.1-1)..=(self.1+1)).flat_map(
                    move |y| ((self.0-1)..=(self.0+1)).filter(move |x| x != &self.0 || y != self.1 || z != self.2 || w != self.3).map(
                        move |x| Coordinate(x, y, z, w)
                    )
                )
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use {super::*};

    #[test]
    fn coordinate_has_80_neighbours() {
        let neighbours = Coordinate(0,0,0,0).neighbours().collect::<Vec<_>>();
        assert_eq!(80, neighbours.len());
    }

    #[test]
    fn verify_testdata() {
        let mut bag = Bag::new(lib::load("../day17/test_input.txt"));
        for _ in 0..6 {
            bag.print_slice(0, 0);
            bag.step();
            bag.print_slice(0, 0);
        }
        assert_eq!(848, bag.count_active());
    }
}