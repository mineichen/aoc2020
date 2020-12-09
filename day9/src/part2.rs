mod lib;

// Wrong: 4294633
fn main() {
    let data = lib::read_cypher();
    if let Err(invalid_sum) = lib::validate(&data) {
        let mut sum = 0;
        let mut lower_end = data.iter().map(|i| *i).enumerate();
        let mut upper_end = data.iter().map(|i| *i).enumerate();
        let mut last_upper = None;
        let mut last_lower = None;

        loop {
            let op = if sum < invalid_sum {
                last_upper = upper_end.next();
                last_upper
            } else if sum > invalid_sum {
                last_lower = lower_end.next();
                last_lower.map(|i| (i.0, u64::max_value() - i.1 + 1))
            } else {
                let lower_index = last_lower.unwrap().0;
                let upper_index = last_upper.unwrap().0;
                let range = &data[lower_index+1..=upper_index];
                let checksum: u64 = range.iter().sum();
                let checksum_diff = invalid_sum as i64 - checksum as i64;
                assert!(checksum_diff == 0);
                let (min, max) = range.iter()
                    .fold(
                        (u64::max_value(), u64::min_value()), 
                        |(low, high), n| (low.min(*n), high.max(*n))
                    );
                
                println!("{}+{}={}", min, max, min + max);
                return;
            };
            if let Some((_, u)) = op {
                sum = sum.overflowing_add(u).0;
            } else {
                return; // no more elements to check
            }
        }
    } else {
        println!("Everything is valid");
    }
}
