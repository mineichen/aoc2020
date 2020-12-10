use std::collections::{hash_map::Entry, HashMap};

pub fn read_rules<'a>(parser: &'a mut RuleParser) -> impl Iterator<Item = Rule> + 'a {
    utils::LineReaderIterator::from_file("day7/input.txt", move |i| parser.parse_line(i))
        .map(Result::unwrap)
}

pub struct RuleParser {
    pub color_generator: IdGenerator,
    pub material_generator: IdGenerator,
}
impl RuleParser {
    pub fn new() -> Self {
        RuleParser {
            color_generator: IdGenerator::new(),
            material_generator: IdGenerator::new(),
        }
    }

    pub fn parse_material<'a>(&mut self, chars: &mut impl Iterator<Item = &'a str>) -> u64 {
        let material = self.material_generator.get_id(chars.next().unwrap()) as u64;
        let color = self.color_generator.get_id(chars.next().unwrap()) as u64;
        material << 32 | color
    }

    fn parse_line(&mut self, line: &str) -> Result<Rule, utils::Error> {
        let mut words = line.split(' ');
        let id = self.parse_material(&mut words);
        let mut words = words.skip(2);
        let mut sub = Vec::new();
        while let Some(nr_str) = words.next() {
            let count = match nr_str.parse::<u32>() {
                Ok(n) => n,
                Err(_) => break,
            };

            let id = self.parse_material(&mut words);
            words.next().unwrap();

            sub.push(RuleRequirement { count, id })
        }

        Ok(Rule { id, sub })
    }
}

pub struct IdGenerator {
    set: HashMap<String, u32>,
    ctr: u32,
}

impl IdGenerator {
    fn new() -> Self {
        Self {
            set: std::collections::HashMap::new(),
            ctr: 1,
        }
    }
    fn get_id(&mut self, input: &str) -> u32 {
        match self.set.entry(input.to_string()) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let result = self.ctr;
                self.ctr += 1;
                e.insert(result);
                result
            }
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub id: u64,
    pub sub: Vec<RuleRequirement>,
}

#[derive(Debug)]
pub struct RuleRequirement {
    pub id: u64,
    pub count: u32,
}

impl Rule {
    pub fn contains_recursive(&self, id: u64, store: &HashMap<u64, Rule>) -> bool {
        self.sub.iter().any(|r| {
            r.count > 0
                && (r.id == id
                    || store
                        .get(&r.id)
                        .map(|rule| rule.contains_recursive(id, store))
                        .unwrap_or(false))
        })
    }

    pub fn count_with_sub_bags(&self, store: &HashMap<u64, Rule>) -> u32 {
        1 + self.sub.iter().fold(0, |acc, sub| {
            acc + sub.count * store[&sub.id].count_with_sub_bags(store)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_line() {
        let mut rule_parser = RuleParser::new();
        let line1 = "pale turquoise bags contain 3 muted cyan bags, 5 striped teal bags.";
        let line2 = "light beige bags contain 5 pale yellow bags, 3 light bronze bags, 5 pale turquoise bags.";
        let rule1 = rule_parser.parse_line(line1).unwrap();
        let rule2 = rule_parser.parse_line(line2).unwrap();

        assert_ne!(rule1.id, rule2.id);
    }

    #[test]
    fn contains_recursive_test() {
        let mut map = std::collections::HashMap::new();
        map.insert(
            1,
            Rule {
                id: 1,
                sub: vec![RuleRequirement { count: 1, id: 2 }],
            },
        );
        map.insert(
            2,
            Rule {
                id: 2,
                sub: vec![RuleRequirement { count: 1, id: 3 }],
            },
        );
        map.insert(3, Rule { id: 2, sub: vec![] });
        map.insert(
            4,
            Rule {
                id: 4,
                sub: vec![RuleRequirement { count: 0, id: 3 }],
            },
        );

        assert_eq!(true, map[&1].contains_recursive(3, &map));
        assert_eq!(false, map[&4].contains_recursive(3, &map));
    }

    #[test]
    fn parse_no_other_line() {
        let mut rule_parser = RuleParser::new();
        let line = "dotted violet bags contain no other bags.";
        let rule = rule_parser.parse_line(line).unwrap();

        assert_eq!(0, rule.sub.len());
    }
}
