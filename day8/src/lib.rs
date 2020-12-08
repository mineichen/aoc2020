pub fn parse(path: &str) -> CodeEmulator {
    CodeEmulator::new(
        utils::LineReaderIterator::from_file(
            path, 
            move|line| {
                let mut it = line.split(' ');
                Ok (match it.next().unwrap() {
                    "acc" => Instruction::Acc(it.next().unwrap().parse()?),
                    "nop" => Instruction::Nop(it.next().unwrap().parse()?),
                    "jmp" => Instruction::Jmp(it.next().unwrap().parse()?),
                    _ => return Err(utils::Error::Format("Unknown instruction"))
                })                
            }
        ).map(Result::unwrap).collect()
    )
}

#[derive(Clone)]
pub struct CodeEmulator {
    pub accumulator: i64,
    pc: usize,
    pub instructions: Vec<Instruction>
}

impl CodeEmulator {
    fn new(program: Vec<Instruction>) -> Self {
        Self { instructions: program, accumulator: 0, pc: 0 }
    }
    pub fn run(&mut self) -> Result<(), Error> {
        let mut visited = BoolMap::new();
        let mut trace = Vec::new();
        loop {
            if visited.is_set(self.pc) {
                return Err(Error::InfiniteLoop(self.pc));
            }
            let instruction = match self.instructions.get(self.pc) {
                Some(x) => x,
                None => return Ok(())
            };
                
            trace.push(instruction.clone());

            visited.set(self.pc);
            match instruction {
                Instruction::Acc(x) => {
                    self.accumulator += x;
                },
                Instruction::Jmp(x) => {
                    self.pc = (self.pc as i64 + x - 1) as usize;
                },
                Instruction::Nop(_) => {
                }
            }
            self.pc += 1;
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
       
    #[error("Program runs infinite at position {0}")]
    InfiniteLoop(usize)
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64)
}

struct BoolMap(Vec<usize>);

impl BoolMap {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn set(&mut self, pos: usize) {
        let (group, mask) = Self::split_pos(pos);

        if self.0.len() <= group + 1 {
            self.0.resize(group + 1, 0);
        }
        let r = self.0.get_mut(group).unwrap();
        *r |= mask;
    }
    pub fn is_set(&self, pos: usize) -> bool {
        let (group, mask) = Self::split_pos(pos);
        group < self.0.len() && self.0[group] & mask > 0
    }
    fn split_pos(pos: usize) -> (usize, usize) {
        (pos >> 6, 1 << (0b111111 & pos))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_bool_map() {
        let mut map = BoolMap::new();
        assert_eq!(false, map.is_set(0));
        assert_eq!(map.0.len(), 0);
        map.set(0);
        assert_eq!(map.0.len(), 1);
        assert_eq!(true, map.is_set(0));
        assert_eq!(false, map.is_set(1));

        assert_eq!(false, map.is_set(256));
        map.set(64);
        assert_eq!(map.0.len(), 2);
        assert_eq!(true, map.is_set(64));
    }
}