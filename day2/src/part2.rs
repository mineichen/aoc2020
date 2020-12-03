mod lib;

fn main() {    
    let rules = lib::load_rules();        
    let valid = rules
        .map(Result::unwrap)
        .filter(|(rule, text)| rule.is_valid(&text))
        .count();
    println!("Valid passwords: {}", valid);
}

impl lib::OccurenceRule {
    pub fn is_valid(&self, input: &str) -> bool {
        self.is_char_valid(input, self.first)
            ^ self.is_char_valid(input, self.second)

    }
    fn is_char_valid(&self, input: &str, pos: usize) -> bool {
        input.chars()
            .skip(pos - 1)
            .map(|c| c == self.char)
            .next()
            .unwrap_or(false)
    }
}