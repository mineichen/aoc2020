use std::{
    collections::{HashMap,HashSet, hash_map::Entry}, 
    str::FromStr
};

pub fn load(path: &str) -> impl Iterator<Item=Food> {
    utils::LineReaderIterator::from_file(path, core::str::FromStr::from_str)
        .map(Result::unwrap)
}

pub struct Food {
    ingredients: HashSet<String>,
    alergens: Vec<String>
}

impl FromStr for Food {
    type Err = utils::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut split = line.split(" (contains ");
        let ingredients = split.next().unwrap()
            .split(' ')
            .map(std::borrow::ToOwned::to_owned)
            .collect();
        let alergens = split.next().unwrap()
            .trim_end_matches(')')
            .split(", ")
            .map(std::borrow::ToOwned::to_owned)
            .collect();

        Ok(Food { ingredients, alergens})
    }
}

pub fn count_harmless_ingredient_occurences(path: &str) -> usize {
    let foods = load(path).collect::<Vec<_>>();
    let ingredient_to_alergen_map = map_ingredients_to_alergen(foods.iter());
    foods.iter().map(|f| f.ingredients.iter()
        .filter(|i| !ingredient_to_alergen_map.contains_key(*i))
        .count()
    ).sum()
}

pub fn map_ingredients_to_alergen<'a>(foods: impl Iterator<Item=&'a Food>) -> HashMap<&'a String, &'a String> {
    let mut alergens = HashMap::<_, Vec<_>>::new();
    for food in foods {
        for alergen in food.alergens.iter() {
            match alergens.entry(alergen) {
                Entry::Occupied(mut e) => {
                    e.get_mut().retain(|i| food.ingredients.contains(*i));
                }
                Entry::Vacant(e) => { 
                    e.insert(
                        food.ingredients.iter().collect::<Vec<_>>()
                    ); 
                }
            }
        }
    }
    let mut ingredient_to_alergen_map = HashMap::<_,_>::new();
    let mut possible_ingredients_per_alergen = alergens.into_iter().collect::<Vec<_>>();
    possible_ingredients_per_alergen.sort_by_key(|i| i.1.len());
    
    loop {
        let len_before = possible_ingredients_per_alergen.len();
        possible_ingredients_per_alergen.retain(|(alergen, ingredients)| {
            let mut iter = ingredients.iter().filter(|a| !ingredient_to_alergen_map.contains_key(*a));
            let first = iter.next().expect("Cannot find matching ingredient to alergen");
            if iter.next() == None {
                ingredient_to_alergen_map.insert(first.clone(), alergen.clone());
                false
            } else {
                true
            }
        });
        if len_before == possible_ingredients_per_alergen.len() {
            break;
        }
    }
    assert_eq!(0, possible_ingredients_per_alergen.len(), "Expected possible_ingredients_per_alergen to be empty");

    ingredient_to_alergen_map
}

#[cfg(test)]
mod tests {
    use {super::*};
    #[test]
    fn test_parse() {
        let line = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)";
        let food = Food::from_str(line).unwrap();
        assert!(food.alergens.contains(&"fish".to_owned()));
        assert!(food.alergens.contains(&"dairy".to_owned()));
        assert!(food.ingredients.contains(&"mxmxvkd".to_owned()));
        assert!(food.ingredients.contains(&"nhms".to_owned()));

        assert!(!food.alergens.contains(&"".to_owned()));
        assert!(!food.ingredients.contains(&"".to_owned()));
    }

    #[test]
    fn process_part1_example_input() {
        assert_eq!(5, count_harmless_ingredient_occurences("../day21/part1_test_input.txt"));
    }
}

