use itertools::Itertools;
pub struct Game {
    input: Vec<usize>,
    current_cup_index: usize,
}

pub trait GameTrait {
    fn new(input: Vec<usize>) -> Self;
    fn solve(&mut self, rounds: usize);
    fn collect_from_label_1(&self) -> usize;
    fn multiply_two_after_label_1(&self) -> usize;
}

pub struct FastGame {
    next_lut: Vec<usize>,
    next_lut_idx: usize
}

impl GameTrait for FastGame {
    fn new(input: Vec<usize>) -> Self {
        let mut next_lut = vec!(0;input.len());

        for (prev, next) in input.iter().circular_tuple_windows() {
            next_lut[*prev-1] = *next-1;
        }
        let next_lut_idx = *input.first().unwrap() - 1;

        Self { next_lut, next_lut_idx }
    }
    fn solve(&mut self, rounds: usize) {
        for _ in 0..rounds {
            let first = self.next_lut[self.next_lut_idx];
            let second = self.next_lut[first];
            let third = self.next_lut[second];
            let fourth = self.next_lut[third];
            let mut dst = self.next_lut_idx;
            //println!("{}, {}, {}, {}, {}", self.next_lut_idx+1, first+1, second+1, third+1, fourth+1);
            
            loop {
                dst = if dst == 0 { self.next_lut.len() - 1 } else { dst - 1};
                if dst != first && dst != second && dst != third {
                    break
                }
            }
            
            self.next_lut[self.next_lut_idx] = fourth;
            self.next_lut[third] = self.next_lut[dst];
            self.next_lut[dst] = first;            
            
            self.next_lut_idx = fourth;
        }        
    }

    fn collect_from_label_1(&self) -> usize {
        
        (0..8).fold(
            (0, 0), 
            |(sum, idx), _| { 
                let next = self.next_lut[idx];
                (sum * 10 + next + 1, next)
            }
        ).0
    }

    fn multiply_two_after_label_1(&self) -> usize {
        let first = self.next_lut[0];
        let second = self.next_lut[first];
        (first + 1) * (second + 1)
    }
}

impl GameTrait for Game {
    fn new(input: Vec<usize>) -> Self {
        Self { input, current_cup_index: 0 }
    }
    fn solve(&mut self, rounds: usize) {
        let now = std::time::SystemTime::now();
        for round in 0..rounds {
            /*
            if round % 512 == 511 {
                let micros = now.elapsed().unwrap().as_micros() as f64;
                let ratio = round as f64 / rounds as f64;
                println!("Estimated Rest: {}s", (micros / ratio) / 1_000_000.);
            } */
            let current_cup_label = self.input[self.current_cup_index];
           
            let mut source_idx = WrappingCounter::new(
                self.current_cup_index,
                self.input.len()
            );
            let mut destination_idx = source_idx.clone();

            let next_current_cup = source_idx.next();
            let following_labels = [ 
                self.input[next_current_cup], 
                self.input[source_idx.next()],
                self.input[source_idx.next()]
            ];
            let destination_label = self.decrease_label_until_none_match(current_cup_label, &following_labels);
            // println!("Round {}: {:?}, dest: {}, labels: {:?} ", round, self.input, destination_label, following_labels);
            loop {
                let item = self.input[source_idx.next()];
                self.input[destination_idx.next()] = item;
                if item == destination_label {
                    break;
                }
            }
            self.input[destination_idx.next()] = following_labels[0];
            self.input[destination_idx.next()] = following_labels[1];
            self.input[destination_idx.next()] = following_labels[2];
            self.current_cup_index = next_current_cup;
        }
    }
    fn collect_from_label_1(&self) -> usize {
        let mut ctr = WrappingCounter::new(0, self.input.len());
        while self.input[ctr.cur] != 1 {
            ctr.next();
        }
        (0..8).fold(0, |acc, _| acc * 10 + self.input[ctr.next()])
    }
    fn multiply_two_after_label_1(&self) -> usize {
        let mut ctr = WrappingCounter::new(0, self.input.len());
        while self.input[ctr.cur] != 1 {
            ctr.next();
        }
        self.input[ctr.next()] * self.input[ctr.next()]
    }
}
impl Game {
    
    
    fn decrease_label_until_none_match(&self, label: usize, next_labels: &[usize]) -> usize {
        let mut result = label;
        loop {
            result = if result == 1 { self.input.len() } else { result - 1 };
            if !next_labels.contains(&result) {
                return result;
            }
        }
    }    
}

#[derive(Clone)]
struct WrappingCounter {
    limit: usize,
    cur: usize
}
impl WrappingCounter {
    fn new(cur: usize, limit: usize) -> Self{
        Self { cur, limit}
    }
    fn next(&mut self) -> usize {
        self.cur = if self.cur < self.limit - 1 { self.cur + 1 } else { 0 };
        self.cur
    }
}

#[cfg(test)]
mod tests {
    use {super::*};
    
    #[test]
    fn test() {
        let mut game = Game::new(vec!(3, 8, 9, 1, 2, 5, 4, 6, 7));
        game.solve(10);
        assert_eq!(92658374, game.collect_from_label_1()); 
        assert_eq!(9*2, game.multiply_two_after_label_1());     
    }
    #[test]
    fn test_fast() {
        let mut game = FastGame::new(vec!(3, 8, 9, 1, 2, 5, 4, 6, 7));
        game.solve(10);
        assert_eq!(92658374, game.collect_from_label_1());  
        assert_eq!(9*2, game.multiply_two_after_label_1());      
    }
    #[test]
    fn test_part2() {
        let mut game = FastGame::new(vec!(3, 8, 9, 1, 2, 5, 4, 6, 7).into_iter().chain(10..=1_000_000).collect());
        game.solve(10_000_000);
        assert_eq!(149245887792, game.multiply_two_after_label_1());
        //assert_eq!(8)
    }
    #[test]
    fn test_decrease_label() {
        let game = Game::new(vec!(1,2,3,4,5,6));
        assert_eq!(6, game.decrease_label_until_none_match(1, &[2,3,4]));
        assert_eq!(5, game.decrease_label_until_none_match(6, &[2,3,4]));
        assert_eq!(1, game.decrease_label_until_none_match(3, &[2,3,4]));
    }
    #[test]
    fn test_wrapping_counter() {
        let mut ctr = WrappingCounter::new(10, 12);
        assert_eq!(ctr.next(), 11);
        assert_eq!(ctr.next(), 0);
    }
}
