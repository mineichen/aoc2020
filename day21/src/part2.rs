pub mod lib;
use itertools::join;

fn main() {
    let foods = lib::load("day21/input.txt").collect::<Vec<_>>();
    let ingredient_to_alergen_map = lib::map_ingredients_to_alergen(foods.iter());
    let mut to_sort = ingredient_to_alergen_map.iter().collect::<Vec<_>>();
    to_sort.sort_by_key(|i| i.1);
    println!("Ingredients: {}", join(to_sort.iter().map(|(ingr, _alerg)| ingr), ","));
}
