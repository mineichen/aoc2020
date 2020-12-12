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
            Cmd::North(a) => y += a,
            Cmd::South(a) => y -= a,
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

pub fn calc_waypoint(commands: impl Iterator<Item=Cmd>) -> (f64, f64) {
    let mut x = 0.;
    let mut y = 0.;
    let mut waypoint_x = 10.; // relative
    let mut waypoint_y = 1.;

    for cmd in commands {
        match cmd {
            Cmd::North(a) => waypoint_y += a,
            Cmd::South(a) => waypoint_y -= a,
            Cmd::West(a) => waypoint_x -= a,
            Cmd::East(a) => waypoint_x += a,
            Cmd::Left(a) => {
                let angle = a * std::f64::consts::PI / 180.;
                let (new_x, new_y) = (
                    angle.cos() * waypoint_x - angle.sin() * waypoint_y,
                    angle.sin() * waypoint_x + angle.cos() * waypoint_y
                );
                waypoint_x = new_x;
                waypoint_y = new_y;
            },
            Cmd::Right(a) => {
                let angle = -a * std::f64::consts::PI / 180.;
                let (new_x, new_y) = (
                    angle.cos() * waypoint_x - angle.sin() * waypoint_y,
                    angle.sin() * waypoint_x + angle.cos() * waypoint_y
                );
                waypoint_x = new_x;
                waypoint_y = new_y;
            },
            Cmd::Forward(a) => {
                x += waypoint_x * a;
                y += waypoint_y * a;
            }
        }
    }
    (x, y)
}

#[derive(Debug, Copy, Clone)]
pub enum Cmd {
    North(f64),
    South(f64),
    West(f64),
    East(f64),
    Forward(f64),
    Right(f64),
    Left(f64)
}

#[cfg(test)]
mod tests {
    use {super::*};
    #[test]
    fn calc_waypoint_test() {
        let cmd = [
            Cmd::Forward(10.),
            Cmd::North(3.),
            Cmd::Forward(7.),
            Cmd::Right(90.),
            Cmd::Forward(11.)
        ];
        let (x, y) = calc_waypoint(cmd.iter().map(|a| *a));
        assert_eq!(214., x);
        assert_eq!(-72., y);
    }
}
