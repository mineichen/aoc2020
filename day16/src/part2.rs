use std::collections::HashMap;

pub mod lib;

fn main() {
    let map = parse_my_ticket("day16/input.txt");
    let product: usize = map.iter()
        .filter(|(key, _)| key.starts_with("departure"))
        .map(|v| v.1)
        .product();
    println!("Product of all departure-fields: {}", product);
    
}
fn parse_my_ticket(path: &str) -> HashMap<String, usize> {
    let infos = lib::load(path).unwrap();
    let mut flagged_rules: Vec<_>  = infos.rules.iter()
        .map(|r| (RuleStats::new(infos.rules.len()), r))
        .collect();
    // println!("{:?}", infos.iter_valid().flat_map(|i| i.iter().enumerate()).collect::<Vec<_>>());
    for (index, value) in infos.iter_valid().flat_map(|i| i.iter().enumerate()) {    
        let invalids = flagged_rules.iter_mut().filter(|(_,i)| !i.contains(value));
        for (stats, _rule) in invalids {
            stats.register_pos(index);
        }
    }

    let mut to_process = flagged_rules.len();
    let mut continue_processing = true;
    while continue_processing {
        continue_processing = false;
        let mut i = 0;
        while i < to_process {
            if let Some(x) = flagged_rules[i].0.unique() {
                to_process -= 1;
                flagged_rules.swap(i, to_process);
                for j in 0..to_process {
                    flagged_rules.get_mut(j).unwrap().0.register_pos(x);
                }
                continue_processing = true;
            }
            i += 1;
        }
    }
    println!("Resolved the following: {:?}", flagged_rules.iter().map(|i| i.1.name.clone()).collect::<Vec<_>>());
    assert_eq!(0, to_process, "Expected all rules to eventually be unique");
    flagged_rules.iter().map(|i| (
        i.1.name.clone(),
        infos.my_ticket[i.0.unique().unwrap()]
    )).collect()
}

#[derive(Debug)]
struct RuleStats {
    missing: usize,
    mask: usize
} 

impl RuleStats {
    fn new(missing: usize) -> Self {
        Self { missing, mask: (1 << missing)-1 }
    }
    fn register_pos(&mut self, index: usize) {
        if (self.mask & 1<<index) > 0 {
            self.mask ^= 1 << index;
            self.missing -= 1;
        }
    }
    fn unique(&self) -> Option<usize> {
        if self.missing == 1 {
            Some(self.mask.trailing_zeros() as usize)
        } else { 
            None
        }
    } 
}


#[cfg(test)]
mod tests {
    use {super::*};

    #[test]
    fn identify_for_test_input() {
        let r = parse_my_ticket("../day16/test_input.txt");
        
        assert_eq!(r["row"], 11);
        assert_eq!(r["class"], 12);
        assert_eq!(r["seat"], 13);
    }

    #[test]
    fn rule_stats() {
        let mut stat = RuleStats::new(3);
        assert_eq!(None, stat.unique());
        stat.register_pos(2);
        assert_eq!(None, stat.unique());
        stat.register_pos(0);
        assert_eq!(Some(1), stat.unique());
    }  
    #[test]
    fn rule_stats_first() {
        let mut stat = RuleStats::new(3);
        stat.register_pos(2);
        stat.register_pos(1);
        assert_eq!(Some(0), stat.unique());
    }  
    #[test]
    fn rule_stats_last() {
        let mut stat = RuleStats::new(4);
        stat.register_pos(0);
        stat.register_pos(1);
        stat.register_pos(2);
        assert_eq!(Some(3), stat.unique());
    }  
}
