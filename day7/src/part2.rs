pub mod lib;

fn main() {
    let mut hashmap = std::collections::HashMap::new();
    let mut parser = lib::RuleParser::new();

    for rule in lib::read_rules(&mut parser) {
        hashmap.insert(rule.id, rule);
    }

    let shiny_id = parser.parse_material(&mut "shiny gold".split(' '));
    let cnt = hashmap[&shiny_id].count_with_sub_bags(&hashmap);
    println!("Count recursive: {}", cnt - 1);
}
