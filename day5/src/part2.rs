mod lib;

fn main() {
    let mut seats = lib::list_seats().collect::<Vec<_>>();
    seats.sort_by(|a, b| a.id.cmp(&b.id));

    for non_filled in seats.windows(2).filter(|p| p[0].id + 1 != p[1].id) {
        println!("Possibility: {:?}", non_filled[0].id + 1);
    }
}
