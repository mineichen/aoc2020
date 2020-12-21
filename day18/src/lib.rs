extern crate nom;

use {
    nom::{
        IResult, 
        bytes::complete::{tag, take_while1}, 
        combinator::{map_res},
        character::complete::char,
        sequence::{delimited},
        branch::alt,
        multi::{fold_many1}
    },
    std::str::FromStr
};

pub fn line_results(path: &str) -> impl Iterator<Item=i64> {
    utils::LineReaderIterator::from_file(
            path, 
            move |line| Ok(calculation_no_precedence(line).unwrap().1)
        )
        .map(Result::unwrap)
}

fn calculation_no_precedence(i: &str) -> nom::IResult<&str, i64>  {
    let (mut i, mut result) = expression(i)?;
    while let Ok((irest, r)) = operation(i, result) {
        i = irest;
        result = r;
    }
    
    Ok((i, result))
}
fn expression(i: &str) -> nom::IResult<&str, i64> {
    let (i, inner) = alt((
         (delimited(char('('), calculation_no_precedence, char(')'))),
         number
    ))(i.trim_start())?;
    Ok((i, inner))
}

pub fn operation(i: &str, before: i64) -> nom::IResult<&str, i64> {
    
    let (i, op) = alt((char('+'), char('*')))(i.trim_start())?;
    let (i, second) = expression(i)?;
    
    Ok((i, match op {
        '+' => before + second,
        '*' => before * second,
        _ => panic!("Foo")
    }))
}

fn number(i: &str) -> nom::IResult<&str, i64> {
    let (i, n) = map_res(
        take_while1(|c: char| c.is_digit(10)),
        FromStr::from_str
    )(i.trim_start())?;
    Ok((i, n))
}

#[cfg(test)]
mod tests {
    use {super::*};
    #[test]
    fn parse_simple() {
        let (rest, n) = number("11+2").unwrap();
        assert_eq!(11, n);
        assert_eq!(rest, "+2");
    }
    #[test]
    fn parse_sum() {
        let (rest, n) = calculation_no_precedence("11+2+1").unwrap();
        assert_eq!(14, n);
        assert_eq!(rest, "");
    }
    #[test]
    fn parse_product() {
        let (rest, n) = calculation_no_precedence("11*2+1").unwrap();
        assert_eq!(23, n);
        assert_eq!(rest, "");
    }
    #[test]
    fn parse_product_parantesis() {
        let (rest, n) = expression("(11*2)+1").unwrap();
        assert_eq!(22, n);
        assert_eq!(rest, "+1");
    }  
    #[test]
    fn parse_product_parantesis_first() {
        let (rest, n) = calculation_no_precedence("2 * 3 + (4 * 5)").unwrap();
        assert_eq!(rest, "");
        assert_eq!(26, n);
    } 
    #[test]   
    fn parse_example_2() {
        let (rest, n) = calculation_no_precedence("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap();
        assert_eq!(rest, "");
        assert_eq!(437, n);
    }
    #[test]   
    fn parse_example_4() {
        let (rest, n) = calculation_no_precedence("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap();
        assert_eq!(rest, "");
        assert_eq!(13632, n);
    }
}
