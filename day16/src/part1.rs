pub mod lib;

fn main() {
    let infos = lib::load("day16/input.txt").unwrap();
    let rules = &infos.rules;
    let sum : usize = infos.other_tickets.iter().map(|ticket| {
        ticket.iter().filter(
            |nr| !rules.iter().any(|r| r.contains(*nr))
        ).sum::<usize>()
    }).sum();
    println!("Sum of all outliers: {}", sum);
}
