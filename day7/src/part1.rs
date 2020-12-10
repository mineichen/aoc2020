pub mod lib;

fn main() {
    let mut hashmap = std::collections::HashMap::new();
    let mut parser = lib::RuleParser::new();

    for (i, rule) in lib::read_rules(&mut parser).enumerate() {
        println!("Rule {}: {:?}", i, rule);
        hashmap.insert(rule.id, rule);
    }
    let shiny_id = parser.parse_material(&mut "shiny gold".split(' '));
    println!("Hashmap len: {}", hashmap.len());
    println!("'shiny gold'-id: {}", shiny_id);

    let numtypes = hashmap
        .values()
        .filter(|rule| rule.contains_recursive(shiny_id, &hashmap))
        .count();
    println!(
        "Number of Bags containing at least one shiny gold: {}",
        numtypes
    );
}
