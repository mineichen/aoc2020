pub fn resolve_step(game: Vec<usize>, nth: usize) -> usize {
    let init_len = game.len();
    let mut last_spoken = *game.last().unwrap();
    let mut steps: Vec<Cell> = vec!(Cell::new(); game.iter().max().map(|i| i+1).unwrap_or(0));
    for (i, n) in game.into_iter().enumerate() {
        steps.get_mut(n).unwrap().push(i);
    }
    

    for i in init_len..nth {
        let stats = steps.get_mut(last_spoken).unwrap();
        last_spoken = match stats.before_last {
            Some(before_last) => { stats.last.unwrap() - before_last },
            None => { 0 }
        };
        if last_spoken >= steps.len() {
            steps.resize(last_spoken+1, Cell::new());
        }
        steps.get_mut(last_spoken).unwrap().push(i);        
    }
    last_spoken
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    last: Option<usize>,
    before_last: Option<usize>
}

impl Cell {
    fn new() -> Self {
        Cell { last: None, before_last: None }
    }
    fn push(&mut self, index: usize) {
        self.before_last = self.last;
        self.last = Some(index);
    }
}

#[cfg(test)]
mod tests {
    use {super::*};
    #[test]
    fn game_0_3_6_works_correctly() {
        assert_eq!(436, resolve_step(vec!(0,3,6), 2020));
    }  
    #[test]
    fn game_1_3_2_works_correctly() {
        assert_eq!(1, resolve_step(vec!(1,3,2), 2020));
    }
    #[test]
    fn game_2_1_3_works_correctly() {
        assert_eq!(10, resolve_step(vec!(2,1,3), 2020));
    }    
}
