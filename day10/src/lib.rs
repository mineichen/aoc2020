pub fn load_sorted(path: &str) -> Vec<usize> {

    let mut result = utils::LineReaderIterator::from_file(
            path, 
            move|line| Ok(line.parse()?)
        )
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    result.sort();
    result
}

pub fn count_joints(sorted_adapters: Vec<usize>) -> Vec<usize> {
    
    let mut it = sorted_adapters;
    let mut last = 0;
    let mut diffs = Vec::new();

    for cur in it {
        let diff = cur - last;

        if diffs.len() <= diff {
            diffs.resize(diff + 1, 0);
        }
        diffs[diff] += 1;
        last = cur;
    }
    diffs[3] += 1;
    diffs
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_joints_test() {
        let result = count_joints(load_sorted("../day10/testdata.txt"));
        assert_eq!(220, result[1] * result[3])
    }
}

