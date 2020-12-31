pub fn count_required_loops(base: usize, key: usize) -> usize {
    let mut nr = SubjectNumber::new(base);
    let mut ctr = 0;
    while nr.value != key {
        nr.transform();
        ctr += 1;
    }
    ctr
}

pub struct SubjectNumber {
    pub value: usize,
    base_nr: usize
}
impl SubjectNumber {
    pub fn new(base_nr: usize) -> Self{
        Self { value: 1, base_nr }
    }
    fn transform(&mut self) {
        self.value = (self.value * self.base_nr) % 20201227;
    }
    pub fn transform_many(&mut self, cnt: usize) {
        for _ in 0..cnt {
            self.transform();
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*};
    #[test]
    fn test_example() {
       assert_eq!(8, count_required_loops(7, 5764801));
    }
}
