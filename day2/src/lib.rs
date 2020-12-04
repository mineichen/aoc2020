use utils::Error;

fn split_once(input: &str, delimiter: char) -> Result<(&str, &str), Error> {
    let mut first = input.splitn(2, delimiter);
    return Ok((
        first.next().unwrap_or(""), 
        first.next().ok_or(Error::Split(delimiter))?
    ));
}

fn parse_line(input: &str) -> Result<(OccurenceRule, String), Error> {
    let (min_text, rest) = split_once(input, '-')?;
    let (max_text, rest) = split_once(rest, ' ')?;                    

    let min = min_text.parse()?;
    let max = max_text.parse()?;
    let mut chars = rest.char_indices();
    let (_, char) = chars.next().ok_or(Error::Format("No char provided in role"))?;
    let rule = OccurenceRule { first: min, second: max, char };

    let (offset, _) = chars.skip(2).next().ok_or(Error::Format("No text provided in role"))?;
    let text = rest.split_at(offset).1;
    
    Ok((rule, text.to_owned()))
}

pub fn load_rules() -> impl Iterator<Item=Result<(OccurenceRule, String), Error>> {
    utils::LineReaderIterator::from_file("day2/input.txt", parse_line)
}

#[derive(Debug)]
pub struct OccurenceRule {
    pub first: usize,
    pub second: usize,
    pub char: char
}