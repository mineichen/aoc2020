mod lib;

use lib::Flags;

fn main() {
    println!("Valid passports with rules: {}", lib::count_valid_passports(validate_rule));
}

fn validate_rule(flag: lib::Flags, data: &str) -> Result<bool, utils::Error> {
    Ok(match flag {
        Flags::BYR => data.parse::<u32>()
            .map(|n| n >= 1920 && n <= 2002)?,
        Flags::IYR => data.parse::<u32>()
            .map(|n| n >= 2010 && n <= 2020)?,
        Flags::EYR => data.parse::<u32>()
            .map(|n| n >= 2020 && n <= 2030)?,
        Flags::HGT => {
                let (num_str, unit) = data.split_at(data.len() - 2);
                let num = num_str.parse::<u32>()?;
                match unit {
                    "in" => return Ok(num >= 59 && num <= 76),
                    "cm" => return Ok(num >= 150 && num <= 193),
                    _ => Err(utils::Error::Format("Unknown unit"))?
                }
            },
        Flags::HCL => {
            let (first, rest) = data.split_at(1);
            match first.chars().next() {
                Some(first_char) => first_char == '#' 
                    && u32::from_str_radix(rest, 16).is_ok() 
                    && rest.len() == 6,
                None => false
            } 
        },
        Flags::ECL => {
            match data {
                "amb"|"blu"|"brn"|"gry"|"grn"|"hzl"|"oth" => true,
                _ => false
            }
        },
        Flags::PID => data.len() == 9 && data.parse::<u64>().is_ok(),
        _ => return Err(utils::Error::Format("Unknown Flag detected"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byr() {
        assert_eq!(true, validate_rule(Flags::BYR, "2002").unwrap());
        assert_eq!(false, validate_rule(Flags::BYR, "2003").unwrap());
    }


    #[test]
    fn test_iyr() {
        assert_eq!(true, validate_rule(Flags::IYR, "2020").unwrap());
        assert_eq!(false, validate_rule(Flags::IYR, "2021").unwrap());
    }

    #[test]
    fn test_eyr() {
        assert_eq!(true, validate_rule(Flags::EYR, "2020").unwrap());
        assert_eq!(true, validate_rule(Flags::EYR, "2030").unwrap());
        assert_eq!(false, validate_rule(Flags::EYR, "2031").unwrap());
    }

    #[test]
    fn test_hgt() {
        assert_eq!(true, validate_rule(Flags::HGT, "60in").unwrap());
        assert_eq!(true, validate_rule(Flags::HGT, "190cm").unwrap());
        assert_eq!(false, validate_rule(Flags::HGT, "190in").unwrap());
        assert_eq!(true, validate_rule(Flags::HGT, "190").is_err());
    }

    #[test]
    fn test_hcl() {
        assert_eq!(true, validate_rule(Flags::HCL, "#123abc").unwrap());
        assert_eq!(false, validate_rule(Flags::HCL, "#123abz").unwrap());
        assert_eq!(false, validate_rule(Flags::HCL, "#123abcd").unwrap());
        assert_eq!(false, validate_rule(Flags::HCL, "123abc").unwrap());
    }

    #[test]
    fn test_ecl() {
        for valid in "amb blu brn gry grn hzl oth".split(' ') {
            assert_eq!(true, validate_rule(Flags::ECL, valid).unwrap());
        }
        assert_eq!(false, validate_rule(Flags::ECL, "other").unwrap());
    }

    #[test]
    fn test_pid() {
        assert_eq!(true, validate_rule(Flags::PID,  "000000001").unwrap());
        assert_eq!(false, validate_rule(Flags::PID, "0123456789").unwrap());
    }
}