use itertools::Itertools;

pub fn read_cypher() -> Vec<u64> {
    utils::LineReaderIterator::from_file("day9/input.txt", |line| Ok(line.parse()?))
        .map(Result::unwrap)
        .collect()
}

pub fn validate(data: &Vec<u64>) -> Result<(), u64> {
    for window in data.windows(25 + 1) {
        let previous = &window[0..25];
        let current = window[25];

        previous
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| *a + *b == current)
            .next()
            .ok_or(current)?;
    }
    Ok(())
}
