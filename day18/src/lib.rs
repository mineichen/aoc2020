use nom::sequence::tuple;


use {
    nom::{
        IResult, 
        bytes::complete::{take_while1, take_while}, 
        combinator::{map_res},
        character::complete,
        sequence::{delimited, preceded},
        branch::alt,
        multi::fold_many0,
        error::ParseError
    },
    std::str::FromStr
};

pub fn line_results<TFn: Fn(&str) -> IResult<&str, i64>>(path: &str, mapper: TFn) -> impl Iterator<Item=i64> {
    utils::LineReaderIterator::from_file(
        path, 
        move |line| Ok((mapper)(line).unwrap().1)
    )
    .map(Result::unwrap)
}

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
  
    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c))(i)
}

pub fn calculation_no_precedence(i: &str) -> nom::IResult<&str, i64>  {
    let (i, result) = expression(i)?;
    fold_many0(tuple((
        preceded(sp, alt((complete::char('+'), complete::char('*')))), 
        preceded(sp, expression)
    )), result, |acc, (op, next)| match op {
        '+' => acc + next,
        '*' => acc * next,
        _ => panic!("Foo")
    })(i)
}

fn expression(i: &str) -> nom::IResult<&str, i64> {
    alt((
         (delimited(complete::char('('), calculation_no_precedence, complete::char(')'))),
         number
    ))(i)
}

fn number(i: &str) -> nom::IResult<&str, i64> {
    map_res(
        take_while1(|c: char| c.is_digit(10)),
        FromStr::from_str
    )(i)
}

fn expression_precedence(i: &str) -> nom::IResult<&str, i64> {
    alt((
         (delimited(complete::char('('), calculation_precedence, complete::char(')'))),
         number
    ))(i)
}
pub fn calculation_precedence(i: &str) -> nom::IResult<&str, i64>  {
    let (i, hold) = expression_precedence(i)?;
    let mut pending_multiplication: Option<i64> = None;
    let res = fold_many0(tuple((
        preceded(sp, alt((complete::char('+'), complete::char('*')))), 
        preceded(sp, expression_precedence)
    )), hold, |acc, (op, next)| match op {
        '+' => { 
            match pending_multiplication {
                Some(x) => { pending_multiplication = Some(x + next); acc },
                None => acc + next
            }
        },
        '*' => { 
            match pending_multiplication {
                Some(x) => { let result = acc * x; pending_multiplication = Some(next); result},
                None => { pending_multiplication = Some(next); acc }
            }
        },
        _ => panic!("Foo")
    })(i)?;
    Ok((res.0, res.1 * pending_multiplication.unwrap_or(1)))
}

#[cfg(test)]
mod tests {
    use {super::*};
    #[test]
    fn parse_precedence() {
        let (rest, n) = calculation_precedence("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap();
        
        assert_eq!(rest, "");
        assert_eq!(23340, n);
    }
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
