use std::collections::HashMap;
pub fn load(path: &str) -> Vec<u64> {
    let mut memory = vec![0; std::u16::MAX as usize];
    let memory_mut = &mut memory;
    let mut mask = Mask::new(); 

    utils::LineReaderIterator::from_file(
            path, 
            move |line| { 
                Ok(if line.starts_with("mask = ") {
                    mask = line[7..].chars().fold(Mask::new(), |acc, n| {
                        let mut shift_acc = Mask { and: acc.and << 1, or: acc.or << 1};
                        match n {
                            '0' => {},
                            '1' => {shift_acc.or += 1; shift_acc.and += 1},
                            'X' => {shift_acc.and += 1},
                            _ => panic!("Unknown letter")
                        }
                        shift_acc
                    });
                    1
                } else {
                    let (location_str, value_str) = utils::split_once(line, '=')?;
                    let location = &location_str[4..location_str.len() -2];
                    let value = (value_str.trim().parse::<u64>()? & mask.and) | mask.or;
                    *memory_mut.get_mut(location.parse::<usize>()?).unwrap() = value;
                    1
                })
            }
        )
        .map(Result::unwrap)
        .count();
    memory
}

pub fn load_v2(path: &str) -> HashMap::<usize, u64> {
    let mut memory = HashMap::<usize, u64>::new();
    let memory_mut = &mut memory;
    let mut mask = Mask2::new(); 

    utils::LineReaderIterator::from_file(
            path, 
            move |line| { 
                Ok(if line.starts_with("mask = ") {
                    mask.or = 0;
                    mask.xor.clear();
                    
                    for (i, c) in line[7..].chars().enumerate() {
                        mask.or <<= 1;
                        match c {
                            '0' => {},
                            '1' => {mask.or += 1;},
                            'X' => {
                                let new_mask = 1 << (35 - i);
                                for i in 0..mask.xor.len() {
                                    mask.xor.push(mask.xor[i] ^new_mask)
                                }
                                mask.xor.push(new_mask)},
                            _ => panic!("Unknown letter")
                        };
                    }
                    1
                } else {
                    let (addr_str, value_str) = utils::split_once(line, '=')?;
                    let location = &addr_str[4..addr_str.len() -2];
                    let value = value_str.trim().parse::<u64>()?;
                    let addr_base = location.parse::<usize>()? | mask.or;
                    for xor_mask in std::iter::once(&0).chain(mask.xor.iter()) {
                        let addr = addr_base ^ xor_mask;
                        memory_mut.insert(addr, value);
                    }
                    
                    1
                })
            }
        )
        .map(Result::unwrap)
        .count();
    memory
}

struct Mask {
    and: u64,
    or: u64
}

impl Mask {
    const fn new() -> Self {
        Self { and: 0, or: 0}
    }
}
struct Mask2 {
    xor: Vec<usize>,
    or: usize
}

impl Mask2 {
    const fn new() -> Self {
        Self { xor: Vec::new(), or: 0}
    }
}

#[cfg(test)]
mod tests {
    use {super::*};
    #[test]
    fn init_memory_works() {
        let result = load("day14/test_input.txt");
        assert_eq!(165, result.iter().sum::<u64>());
    }
    #[test]
    fn init_memory_v2_works() {
        let result = load_v2("day14/test_input_part2.txt");
        assert_eq!(208, result.values().sum::<u64>());
    }
}
