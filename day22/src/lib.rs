use std::collections::{HashSet, VecDeque};

pub fn load(path: &str) -> Game {
    let mut numbers = utils::LineReaderIterator::from_file(
            path, 
            move |line| Ok(line.parse::<u8>().ok())
        )
        .map(Result::unwrap);
    let a = numbers.by_ref().skip(1).take_while(|i| i != &None).map(|a| a.unwrap()).collect();
    let b = numbers.by_ref().skip(1).take_while(|i| i != &None).map(|a| a.unwrap()).collect();

    Game(a, b)
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Game(VecDeque<u8>, VecDeque<u8>);
impl Game {
    pub fn solve(&mut self) {
        while let Some(a) = self.0.pop_front() {
            if let Some(b) = self.1.pop_front() {
                self.move_back(a > b, a, b);
            } else {
                self.0.push_front(a);
                break;
            }
        }
    }
    fn move_back(&mut self, is_a_winning: bool, a: u8, b: u8) {
        if is_a_winning {
            self.0.push_back(a);
            self.0.push_back(b);
        } else {
            self.1.push_back(b);
            self.1.push_back(a);
        }
    }
    // True means A wins
    pub fn solve_recursive(&mut self) -> bool {
        let previous_games = &mut HashSet::<Game>::new();
        while let Some(a) = self.0.pop_front() {
            if let Some(b) = self.1.pop_front() {
                //println!("cache: {}, a: {}[{:?}], b: {}[{:?}]", previous_games.len(), a, self.0, b, self.1);
               
                if !previous_games.insert(self.clone()) {
                    return true;
                } else if a as usize <= self.0.len() && b as usize <= self.1.len() {
                    let mut clone = Game (
                        self.0.iter().copied().take(a as usize).collect(),
                        self.1.iter().copied().take(b as usize).collect()
                    );
                    //println!("Start sub: {} <= {} && {} <= {}", a as usize, self.0.len(), b as usize, self.1.len());
                    self.move_back(clone.solve_recursive(), a, b);
                } else {
                    self.move_back(a > b, a, b);
                }
                
            } else {
                self.0.push_front(a);
                break;
            }
        }
        self.0.len() > self.1.len()
    }
    pub fn calc_score(&self) -> usize {
        let len = self.0.len() + self.1.len();
        self.0.iter()
            .chain(self.1.iter())
            .fold((len, 0), |(multiplier, sum), n| 
                (multiplier-1, sum + (*n as usize) * multiplier)
            ).1
    }
}

#[cfg(test)]
mod tests {
    use {super::*};
    #[test]
    fn test_demo_part_1() {
        let mut game = load("../day22/input_test.txt");
        assert_eq!(5, game.0.len());
        assert_eq!(5, game.1.len());
        game.solve();
        assert_eq!(306, game.calc_score())
    }
    #[test]
    fn test_demo_part2() {
        let mut game = load("../day22/input_test.txt");
        game.solve_recursive();
        assert_eq!(291, game.calc_score())
    }

    #[test]
    fn test_recursive_infinite() {
        let mut game = load("../day22/input_infinite.txt");
        game.solve_recursive();
        assert_eq!(129, game.calc_score())
    }

    #[test]
    fn test_two_deq_are_equal() {
        let mut a = VecDeque::new();
        a.push_front(1);
        let mut b = VecDeque::new();
        b.push_front(1);

        assert_eq!(a, b);

        let game = Game(a, b);
        let mut set = HashSet::new();
        assert!(set.insert(game.clone()));
        assert!(!set.insert(game.clone()));

        let mut clone = game.clone();
        clone.0.pop_back();
        let mut new_b = VecDeque::new();
        new_b.push_front(1);
        assert_eq!(clone, Game(VecDeque::new(), new_b));

        clone.1.push_back(1);
        assert_eq!(1, game.1.len());
        assert_eq!(2, clone.1.len());
    }
}
