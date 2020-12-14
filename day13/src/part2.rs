fn main() -> Result<(), utils::Error>{
    let text = std::fs::read_to_string("day13/input.txt")?;
    let line2 = text.split('\n').into_iter().skip(1).next().unwrap();
    let ids = line2.split(',')
        .enumerate()
        .filter_map(|(offset, id_str)| id_str.parse::<u64>().map(|id| Item {
            id, offset: offset as u64
         }).ok())
        .collect::<Vec<_>>();

    let t = calc(ids);
    println!("T: {}", t);
    
    Ok(())
}

struct Item { id: u64, offset: u64}

// Adopt https://topaz.github.io/paste/#XQAAAQBTAwAAAAAAAAA4HUvIY+sakU2wdylCxjVGdiCHisKPHY7OuXrSFrUUUqj3cqd72jp2vgDg8cL1oNJwzB21hiZeCdNmFjqrU49XiZ8tnMUHqe9AyxFlf34WFT0InKF9YLRIcyUIgBMFdU3y7GM5V+CLyQ0Ztd0HeY/xtoqOsjrnUfKKkkIVnT18w0+sW3ZJyClMSCQTvIcvD8qu45CNblyniC9QjpsCfHS5asDQ5le4Dz9sfoQnxR4W1W5CRwx+JTs8RGUbuKt/nG0PhalNMAhh1rmB6f+dkzlI7JBnn1RPWQf2aMZPL6aRzArLmGnCgCmb7TCQ6USt+xcjoFnwabgqpI/Zpn/x8PiAUFuDESr0VzUjeF5yycWOwMYZNemJ6UIhHrKlBNFHSHLzFqju+V3NZou92ZU1OdN03dSrKDIq1mBEZOWeO41q+GUZRYc207G7n9uDvMwLsUY96as/kcaGr54c99v3+Xh36crJLg4bQoVIi/1ATk4=
// to rust. Don't fully understand it yet
fn calc(input: Vec<Item>) -> u64 {
    let mut it = input.into_iter();
    let first = it.next().unwrap();
    let mut final_time = first.id - first.offset % first.id;
    let mut delta = final_time;
    for bus in it {
        let starting_time = bus.id - (final_time % bus.id);
        let bus_time = bus.offset % bus.id;
        
        delta *= {
            let mut cycle = 0;
            let mut current_time = final_time;
            loop {
                if bus.id - (current_time % bus.id) == bus_time {
                    final_time = current_time;
                }
                cycle += 1;
                current_time += delta;
                if bus.id - (current_time % bus.id) == starting_time {
                    break
                }
            }
            cycle
        };
    }
    final_time
}
#[cfg(test)]
mod tests {
    use {super::*};
    #[test]
    fn calc_1() {
       assert_eq!(3417, calc(vec!(
           Item { id: 17, offset: 0},
           Item { id: 13, offset: 2},
           Item { id: 19, offset: 3}
        )));
    }
    #[test]
    fn calc_2() {
       assert_eq!(754018, calc(vec!(
           Item { id: 67, offset: 0},
           Item { id: 7, offset: 1},
           Item { id: 59, offset: 2},
           Item { id: 61, offset: 3}
        )));
    }
    #[test]
    fn calc_3() {
       assert_eq!(1261476, calc(vec!(
           Item { id: 67, offset: 0},
           Item { id: 7, offset: 1},
           Item { id: 59, offset: 3},
           Item { id: 61, offset: 4}
        )));
    }   

    #[test]
    fn calc_4() {
       assert_eq!(1202161486, calc(vec!(
           Item { id: 1789, offset: 0},
           Item { id: 37, offset: 1},
           Item { id: 47, offset: 2},
           Item { id: 1889, offset: 3}
        )));
    }   
}
