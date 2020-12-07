pub fn get_grouped_flags() -> impl Iterator<Item=Poll> {
    PollIterator(utils::LineReaderIterator::from_file("day6/input.txt", |line| {
        let flags = line.chars().fold(0, |acc, n| acc | 1 << (n as u32 - 'a' as u32));
        Ok(Poll { flags })
    }).map(Result::unwrap))
}

struct PollIterator<T: Iterator<Item=Poll>>(T);
impl<T: Iterator<Item=Poll>> Iterator for PollIterator<T> {
    type Item = Poll;

    fn next(&mut self) -> Option<Self::Item> {
        let poll = self.0
            .by_ref()
            .take_while(|p| !p.is_emtpy())
            .fold(Poll { flags: 0}, |acc, p| acc.combine(p));
        if poll.flags != 0 { 
            Some(poll)
        } else {
            None
        } 
    }
}

#[derive(Debug)]
pub struct Poll {
    flags: u32
}

impl Poll {
    pub fn count_unique(&self) -> u32 {
        self.flags.count_ones()
    } 
    fn is_emtpy(&self) -> bool {
        self.flags == 0
    }
    fn combine(&self, other: Poll) -> Poll {
        Poll { flags: self.flags | other.flags}
    }
}