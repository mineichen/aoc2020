pub struct Game {
    input: Vec<usize>
}
impl Game {
    pub const fn new(input: Vec<usize>) -> Self {
        Self { input }
    }
    pub fn solve(&mut self, rounds: usize) -> usize {
        let mut current_cup_index = 0;
        for _round in 0..rounds {
            let current_cup_label = self.input[current_cup_index];
            let mut source_idx = WrappingCounter::new(
                current_cup_index,
                self.input.len()
            );
            let mut destination_idx = source_idx.clone();

            let next_current_cup = source_idx.next();
            let following_labels = [ 
                self.input[next_current_cup as usize], 
                self.input[source_idx.next() as usize],
                self.input[source_idx.next() as usize]
            ];
            let destination_label = self.decrease_label_until_none_match(current_cup_label, &following_labels);
            // println!("Round {}: {:?}, dest: {}, labels: {:?} ", round, self.input, destination_label, following_labels);
            loop {
                let item = self.input[source_idx.next() as usize];
                self.input[destination_idx.next() as usize] = item;
                if item == destination_label {
                    break;
                }
            }
            self.input[destination_idx.next() as usize] = following_labels[0];
            self.input[destination_idx.next() as usize] = following_labels[1];
            self.input[destination_idx.next() as usize] = following_labels[2];
            current_cup_index = next_current_cup as usize;
        }
        let mut ctr = WrappingCounter::new(0, self.input.len());
        while self.input[ctr.cur as usize] != 1 {
            ctr.next();
        }
        (0..8).fold(0, |acc, _| acc * 10 + self.input[ctr.next() as usize] as usize)
    }
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
        let mut game = Game {input: vec!(3, 8, 9, 1, 2, 5, 4, 6, 7)};
        assert_eq!(92658374, game.solve(10));        
    }
    #[test]
    fn test_decrease_label() {
        let game = Game { input: vec!(1,2,3,4,5,6) };
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
