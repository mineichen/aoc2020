use std::io::BufRead;
pub fn load(path: &str) -> Result<Infos, utils::Error> {
    let mut reader = std::io::BufReader::new(std::fs::File::open(path)?).lines().map(Result::unwrap);
    let rules = reader.by_ref()
        .take_while(|rule| !rule.is_empty())
        .map(|rule| {
            let (name, range_text) = utils::split_once(&rule, ':').unwrap();
            let mut it = range_text
                .split(&" or ")
                .map(|t| {
                    let (from, to) = utils::split_once(t.trim(), '-').unwrap();
                    from.parse().unwrap()..=to.parse().unwrap()
                });
            Rule::new(name.to_owned(), it.next().unwrap(), it.next().unwrap())
        })
        .collect::<Vec<_>>();
    let my_ticket = reader.by_ref()
        .skip(1).next().unwrap()
        .split(',')
        .map(|p| p.parse().unwrap())
        .collect();

    let other_tickets = reader.skip(2)
        .map(|l| l.split(',')   
            .map(|p| p.parse().unwrap()).collect()
        ).collect();
    
    Ok(Infos { rules, my_ticket, other_tickets })
}

pub struct Infos {
    pub rules: Vec<Rule>,
    pub my_ticket: Vec<usize>,
    pub other_tickets: Vec<Vec<usize>>
}

impl Infos {
    pub fn iter_valid<'a>(&'a self) -> impl Iterator<Item=&'a Vec<usize>> {
        let rules = &self.rules;
        std::iter::once(&self.my_ticket).chain(
            self.other_tickets.iter().filter(move |ticket| {
                ticket.iter().all(
                    |nr| rules.iter().any(|r| r.contains(nr))
                )
            })
        )
    }
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    first_range: std::ops::RangeInclusive<usize>,
    second_range: std::ops::RangeInclusive<usize>
}

impl Rule {
    fn new(name: String, first_range: std::ops::RangeInclusive<usize>, second_range: std::ops::RangeInclusive<usize>) -> Self {
        Rule { name, first_range, second_range }
    }
    pub fn contains(&self, nr: &usize) -> bool {
        self.first_range.contains(nr) 
            || self.second_range.contains(nr)
    }
}