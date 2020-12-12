pub mod lib;

fn main() {
    lib::output_number_of_occupied_seats(
        lib::all_direction_neighbour_strategy
    );
}
