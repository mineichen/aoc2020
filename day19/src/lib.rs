use std::{collections::HashMap, io::BufRead};

pub fn load(path: &str) -> (Evaluator, impl Iterator<Item=String>) {
    let reader = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let mut lines = reader.lines();
    let rules = lines.by_ref()
        .take_while(|l| if let Ok(r) = l { !r.is_empty() } else { false })
        .map::<Result<_, utils::Error>, _>(|line| {
        let lineref = &line.unwrap();
        let (n, rest) = utils::split_once(lineref, ':')?;
        let mut iter = rest.chars().skip(1);
        let c = iter.next().unwrap();
        Ok((n.trim().parse::<usize>().unwrap(), match c {
            '"' => { 
                Rule::Char(iter.next().unwrap())
            },
            _ => {
                Rule::Ref(rest.split('|')
                    .map(|group| group.trim()
                            .split(' ')
                            .map(|n| n.trim().parse().unwrap())
                            .collect())
                    .collect()
                )
            }
        }))       
    })
    .map(Result::unwrap)
    .collect::<HashMap<usize,Rule>>();
    (Evaluator {rules}, lines.map(Result::unwrap))
}

pub struct Evaluator {
    pub rules: HashMap<usize, Rule>
}

impl Evaluator {
    pub fn check<'a>(&'a self, rule: &usize, chars: &str) -> bool {
        self.match_len(rule, chars, vec!()) == chars.len()
    }
    pub fn replace_8_and_11(&mut self) -> Result<(),()>  {
        *self.rules.get_mut(&8).ok_or(())? = Rule::Ref(vec!(vec!(42), vec!(42, 8)));
        *self.rules.get_mut(&11).ok_or(())? = Rule::Ref(vec!(vec!(42, 31), vec!(42, 11, 31)));
        Ok(())
    }
    fn match_len<'a>(&'a self, rule: &usize, chars: &str, trace: Vec<usize>) -> usize {
        match &self.rules[&rule] {
            Rule::Char(x) => {
                //println!("Char check: {}=={}?, {:?}", x, chars, trace);
                (chars.chars().next() == Some(*x)) as usize
            },
            Rule::Ref(rule_refs) => {
                let mut count = 0;
                let any = rule_refs.iter().any(|ref_ids| {
                    count = 0;
                    ref_ids.iter().all(|id| {
                        let mut traceclone = trace.clone();
                        traceclone.push(*rule);
                        let offset = self.match_len(id, &chars[count..], traceclone);
                        count += offset;
                        offset > 0
                    })
                });
                any as usize * count
            }
        }
    }
    
    pub fn matches<TIter: Iterator<Item=usize> + Clone>(&self, mut rule_list: TIter, msg: &str) -> bool {
        use Rule::*;
        match rule_list.next() {
            Some(head) => {
                match self.rules.get(&head).unwrap() {
                    Char(c) => msg.starts_with(*c) && self.matches(rule_list, &msg[1..]),
                    Ref(r) => r.iter().any(|and_rules| self.matches(
                        and_rules.iter().copied().chain(rule_list.clone()).collect::<Vec<_>>().into_iter(),
                        msg
                    ))
                }
            }
            None => msg.is_empty()
        }
    }
}

#[derive(Debug)]
pub enum Rule {
    Char(char),
    Ref(Vec<Vec<usize>>)
}

#[cfg(test)]
mod tests {
    use {super::*};
    #[test]
    fn test_part_1() {
        let (rules, msgs) = load("../day19/test_input_part1.txt");
        let count_valid = msgs
            .filter(|m| { rules.check(&0, &m) })
            .count();
        assert_eq!(2, count_valid);
    }

    #[test]
    fn test_simple_recursive() {
        let mut rules = HashMap::new();
        rules.insert(0, Rule::Ref(vec!(vec!(1), vec!(1, 0))));
        rules.insert(1, Rule::Char('a'));
        rules.insert(2, Rule::Char('b'));
        rules.insert(3, Rule::Ref(vec!(vec!(2, 1), vec!(0, 1))));
        let eval = Evaluator { rules };
        assert!( eval.matches(std::iter::once(0), "aaa"));
    }

    #[test]
    fn test_part_2() {
        let (mut rules, msgs) = load("../day19/test_input_part2.txt");
        let msgs = msgs.collect::<Vec<_>>();
        //assert_eq!(3, msgs.iter().filter(|m| rules.check(&0, &m)).count());
        rules.replace_8_and_11().unwrap();
        assert!(rules.matches(std::iter::once(0), "aaaaabbaabaaaaababaa"));
        let count_valid = msgs
            .iter()
            .filter(|m| { rules.matches(std::iter::once(0), &m)})
            .count();
        assert_eq!(12, count_valid);
    }
}
