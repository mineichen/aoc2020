use std::{collections::HashSet, ops::Add};

pub fn load(path: &str) -> impl Iterator<Item=Point> {
    utils::LineReaderIterator::from_file(
            path, 
            move |line| {
                Ok(LineParser(line).fold(
                    Point {x: 0., y: 0.},
                    |acc, n| { acc + n}
                ))
            }
        )
        .map(Result::unwrap)
}

const RAD60: f64 = std::f64::consts::PI / 3.;
lazy_static::lazy_static! {
    static ref EAST: Point = Point::from_angle(0.);
    static ref NORTH_EAST: Point = Point::from_angle(RAD60);
    static ref NORTH_WEST: Point = Point::from_angle(2.*RAD60);
    static ref WEST: Point = Point::from_angle(3.*RAD60);
    static ref SOUTH_WEST: Point = Point::from_angle(4.*RAD60);
    static ref SOUTH_EAST: Point = Point::from_angle(5.*RAD60);
    static ref DIRECTIONS: [Point; 6] = [*EAST, *NORTH_EAST, *NORTH_WEST, *WEST, *SOUTH_WEST, *SOUTH_EAST];

}
pub fn load_flipped(path: &str) -> HashSet<Point> {
    let mut set = HashSet::new();
    for point in load(path) {
        if !set.insert(point) {
            set.remove(&point);
        }
    }
    set
}

pub fn change_floor(input: HashSet<Point>) -> HashSet<Point> {
    let mut result = HashSet::new();
    for point in input.iter() {
        let white_neighbours: smallvec::SmallVec<[_; 6]> = DIRECTIONS.iter()
            .map(|d| *point + *d)
            .filter(|d| !input.contains(&d))
            .collect();
        let count_black_neighbours = 6 - white_neighbours.len();
        if count_black_neighbours == 1 || count_black_neighbours == 2 {
            result.insert(*point);
        }
        for n in white_neighbours {
            if DIRECTIONS.iter().filter(|d| input.contains(&(n + **d))).count() == 2 {
                result.insert(n);
            }
        }
    }
    result
}


#[derive(Debug, Clone, Copy)]
pub struct Point { pub x: f64, pub y: f64 }
impl Point {
    fn from_angle(angle: f64) -> Self {
          Point {
            x: angle.cos(),
            y: angle.sin()
        }        
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y}
    }
}
impl Default for Point {
    fn default() -> Self {
        Point { x: 0., y: 0. }
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        ((self.x * 100.).round() as i32) == ((other.x * 100.).round() as i32)
            && ((self.y * 100.).round() as i32) == ((other.y * 100.).round() as i32)        
    }
}
impl Eq for Point {
    fn assert_receiver_is_total_eq(&self) {}
}
impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ((self.x * 100.).round() as i32).hash(state);
        ((self.y * 100.).round() as i32).hash(state);
    }
}


struct LineParser<'a>(&'a str);

impl<'a> Iterator for LineParser<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match nom::branch::alt::<_,_, nom::error::Error<_>,_>((
            nom::combinator::map(nom::bytes::complete::tag("ne"), |_| *NORTH_EAST),
            nom::combinator::map(nom::bytes::complete::tag("nw"), |_| *NORTH_WEST),
            nom::combinator::map(nom::bytes::complete::tag("se"), |_| *SOUTH_EAST),
            nom::combinator::map(nom::bytes::complete::tag("sw"), |_| *SOUTH_WEST),
            nom::combinator::map(nom::bytes::complete::tag("e"), |_| *EAST),
            nom::combinator::map(nom::bytes::complete::tag("w"), |_| *WEST)
        ))(self.0) {
            Ok((rest, next)) => { 
                self.0 = rest;
                Some(next)
            }
            Err(_) => { None }
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*};
       
    #[test]
    fn test_simple_daily_change() {
        let mut set = HashSet::new();
        set.insert(Point::default());
        set.insert(*EAST);

        assert_eq!(4, change_floor(set).len());
    }
    #[test]
    fn sum_to_zero() {        
        assert_eq!(
            Point::default(),
            LineParser("nenwwswsee").fold(Point::default(), |acc, n| acc + n)
        );
        assert_eq!(
            Point::default(),
            LineParser("nesw").fold(Point::default(), |acc, n| acc + n)
        );
        assert_eq!(
            Point::default(),
            LineParser("ew").fold(Point::default(), |acc, n| acc + n)
        );
        assert_eq!(
            Point::default(),
            LineParser("senw").fold(Point::default(), |acc, n| acc + n)
        );
        assert_eq!(
            Point::default(),
            LineParser("nwwswee").fold(Point::default(), |acc, n| acc + n)
        );
        assert_eq!(
            Point::default(),
            LineParser("nwesw").fold(Point::default(), |acc, n| acc + n)
        );
        assert_eq!(
            Point::default(),
            LineParser("swenw").fold(Point::default(), |acc, n| acc + n)
        );
        assert_eq!(
            Point::default(),
            LineParser("swenw").fold(Point::default(), |acc, n| acc + n)
        );
    }

    #[test]
    fn count_flipped_in_test_input() {
        assert_eq!(10, load_flipped("../day24/test_input.txt").len());
    }
    #[test]
    fn flip_neighbours() {
        assert_eq!(15, change_floor(load_flipped("../day24/test_input.txt")).len());
    }

    #[test]
    fn parse_line() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw";
        let mut iter = LineParser(input);

        assert_eq!(Some(*SOUTH_EAST), iter.next());
        assert_eq!(Some(*NORTH_WEST), iter.by_ref().skip(1).next());
        assert_eq!(Some(*NORTH_EAST), iter.next());
        assert_eq!(Some(*WEST), iter.by_ref().skip(2).next());
        assert_eq!(Some(*EAST), iter.by_ref().skip(1).next());
        assert_eq!(Some(*SOUTH_WEST), iter.next());

        assert_eq!(10, iter.count());
    }
}
