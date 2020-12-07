bitflags::bitflags! {
    pub struct Flags: u32 {
        const BYR = 1; // (Birth Year)
        const IYR = 2; // (Issue Year)
        const EYR = 4; // (Expiration Year)
        const HGT = 8; // (Height)
        const HCL = 16; // (Hair Color)
        const ECL = 32; // (Eye Color)
        const PID = 64; // (Passport ID)
        const CID = 128; // (Country ID)

        const REQUIRED = Self::BYR.bits
                    | Self::IYR.bits
                    | Self::EYR.bits
                    | Self::HGT.bits
                    | Self::HCL.bits
                    | Self::ECL.bits
                    | Self::PID.bits;
    }
}

pub fn count_valid_passports<T: Fn(Flags, &str) -> Result<bool, utils::Error>>(validator: T) -> u32 {
    count_valid_passports_on_iter(std::fs::File::open("day4/input.txt").unwrap(), validator)
}

fn count_valid_passports_on_iter<T: Fn(Flags, &str) -> Result<bool, utils::Error>,>(reader: impl std::io::Read, validator: T) -> u32 {
    let mut iter = utils::LineReaderIterator::from_reader(reader, |line| {
        
        let mut result = Flags::empty();
        if line.is_empty() {
            return Ok(result);   
        }

        for pair in line.trim().split(' ') {
            let (key, value) = utils::split_once(pair, ':')?;
            
            let flag = match key {
                "byr" => Flags::BYR,
                "iyr" => Flags::IYR,
                "eyr" => Flags::EYR,
                "hgt" => Flags::HGT,
                "hcl" => Flags::HCL,
                "ecl" => Flags::ECL,
                "pid" => Flags::PID,
                "cid" => Flags::CID,
                _ => panic!("Unknown flag")
            };
            if flag == Flags::CID || (validator)(flag, value).unwrap_or(false) {
                result |= flag;
            }            
        }
        Ok(result)
    }).map(Result::unwrap);

    let mut counter = 0;
    while let Some(item) = iter.next() {
        let mut flags = item;
        while let Some(next) = iter.by_ref().take_while(|l| l != &Flags::empty()).next() {
            flags |= next;
        }
        if flags.contains(Flags::REQUIRED) {
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {

    #[test]
    fn cid_is_not_a_splitter() {
        let input = "pid:479898570 hgt:165cm eyr:2024 byr:1932
        iyr:2010 ecl:grn
        cid:88
        hcl:#c0a76e";
        let c = super::count_valid_passports_on_iter(
            std::io::Cursor::new(input), 
            |flag, _| Ok(flag != super::Flags::CID)
        );
        assert_eq!(1, c);
    }
}