pub fn load(path: &str) -> impl Iterator<Item=Cmd> {
    utils::LineReaderIterator::from_file(
            path, 
            move |line| {
                let mut it = line.char_indices();
                let (_, code) = it.next().ok_or(utils::Error::Format("Line contains no chars"))?;
                let (pos, _) = it.next().ok_or(utils::Error::Format("Char expects to be followed by number"))?;
                let n = line[pos..].parse::<i64>()? as f64;
                Ok(match code {
                    'N' => Cmd::North(n),
                    'S' => Cmd::South(n),
                    'W' => Cmd::West(n),
                    'E' => Cmd::East(n),
                    'L' => Cmd::Left(n),
                    'R' => Cmd::Right(n),
                    'F' => Cmd::Forward(n),
                    _ => return Err(utils::Error::Format("Unknown Code"))
                })
            }
        )
        .map(Result::unwrap)
}

pub fn calc(commands: impl Iterator<Item=Cmd>) -> (f64, f64) {
    let mut angle = 0.0;
    let mut x = 0.;
    let mut y = 0.;
    for cmd in commands {
        match cmd {
            Cmd::North(a) => y -= a,
            Cmd::South(a) => y += a,
            Cmd::West(a) => x -= a,
            Cmd::East(a) => x += a,
            Cmd::Left(a) => angle -= a * std::f64::consts::PI / 180.,
            Cmd::Right(a) => angle += a * std::f64::consts::PI / 180.,
            Cmd::Forward(a) => {
                x += angle.cos() * a;
                y += angle.sin() * a;
            }
        }
    }
    (x, y)
}

pub enum Cmd {
    North(f64),
    South(f64),
    West(f64),
    East(f64),
    Forward(f64),
    Right(f64),
    Left(f64)
}