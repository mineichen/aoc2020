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
    let mut iter = utils::LineReaderIterator::from_file("day4/input.txt", |line| {
        
        let mut result = Flags::empty();
        
        if line.is_empty() {
            return Ok(result);   
        }

        for pair in line.split(' ') {
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
            result |= flag;
            
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