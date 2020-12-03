mod lib;


fn main() {
    let rules = lib::load_rules();    
    let valid = rules.filter(|(rule, text)| rule.is_valid(&text)).count();
    println!("Valid passwords: {}", valid);
}


impl lib::OccurenceRule {
    fn is_valid(&self, input: &str) -> bool {
        let cnt = input.chars()
            .filter(|c| c == &self.char)
            .count();
        self.first <= cnt 
            && self.second >= cnt
    }
}