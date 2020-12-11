pub mod lib;

fn main() {
    let mut puzzle = lib::load_puzzle_with_floor_surrounding("day11/input.txt");
    while puzzle.apply_once(lib::immediate_neighbour_strategy){
        print!(".");
    }
    let occupied_seats = puzzle.cells.iter()
        .filter(|c| c == &&lib::CellState::Occupied)
        .count();
    println!("Stable number of occupied seats: {}", occupied_seats);
}
