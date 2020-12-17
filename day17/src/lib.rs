pub fn load(path: &str) -> impl Iterator<Item=(i32,i32)> {
    let mut occupied = Vec::new();
    let occupied_ref = &mut occupied;
    let mut line_counter = 0;
    utils::LineReaderIterator::from_file(
            path, 
            move |line| {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        occupied_ref.push((i as i32, line_counter))
                    }
                }
                line_counter += 1;
                Ok(())
            }
        ).count();
    occupied.into_iter()
}

