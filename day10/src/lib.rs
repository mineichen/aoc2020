pub fn load_sorted(path: &str) -> Vec<usize> {
    let mut result = utils::LineReaderIterator::from_file(path, move |line| Ok(line.parse()?))
        .map(Result::unwrap)
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();

    result.sort();
    result.push(result.last().unwrap() + 3);
    result
}

pub fn count_combinations(path: &str) -> usize {
    let data = load_sorted(path);
    let mut result = 1;
    let mut low = 0;
    for (i, window) in data.windows(2).enumerate() {
        let first = window[0];
        let second = window[1];
        if second - first == 3 {
            let to_skip = 1.max(i as i64 - low as i64) as usize;
            result *= TribonacciIter::new().skip(to_skip).next().unwrap();
            low = i + 1;
        }
    }
    result
}

struct TribonacciIter([usize; 3], u8);
impl TribonacciIter {
    fn new() -> Self {
        TribonacciIter([1, 1, 2], 253)
    }
}
impl Iterator for TribonacciIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let ctr = self.1;
        self.1 = self.1.overflowing_add(1).0;
        Some(match ctr {
            253 => 1,
            254 => 1,
            255 => 2,
            _ => {
                let result = self.0.iter().sum();
                *self.0.get_mut(ctr as usize).unwrap() = result;

                if self.1 == 3 {
                    self.1 = 0;
                }

                result
            }
        })
    }
}

pub fn count_joints(sorted_adapters: Vec<usize>) -> Vec<usize> {
    let mut diffs = Vec::new();

    for cur in sorted_adapters.windows(2) {
        let diff = cur[1] - cur[0];

        if diffs.len() <= diff {
            diffs.resize(diff + 1, 0);
        }
        diffs[diff] += 1;
    }
    diffs
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA_PATH: &str = "day10/testdata.txt";
    #[test]
    fn count_joints_test() {
        let result = count_joints(load_sorted(TEST_DATA_PATH));
        assert_eq!(220, result[1] * result[3])
    }

    #[test]
    fn count_combinations_test() {
        assert_eq!(19208, count_combinations(TEST_DATA_PATH));
    }

    #[test]
    fn test_tribonacci() {
        let test_sequence = [1, 1, 2, 4, 7, 13, 24, 44, 81].iter();

        for (expected, result) in test_sequence.zip(TribonacciIter::new()) {
            assert_eq!(*expected, result);
        }
    }
}
