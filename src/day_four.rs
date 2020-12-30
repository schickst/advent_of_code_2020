use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Passport {  
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}


impl Passport {
    fn from_string(data: String) -> Passport {
        let tokens = data.split_ascii_whitespace();

        let mut byr: Option<String> = None;
        let mut iyr: Option<String> = None;
        let mut eyr: Option<String> = None;
        let mut hgt: Option<String> = None;
        let mut hcl: Option<String> = None;
        let mut ecl: Option<String> = None;
        let mut pid: Option<String> = None;
        let mut cid: Option<String> = None;

        for token in tokens {
            let key_value: Vec<&str> = token.split(":").collect();

            match key_value[0] {
                "byr" => byr = Some(key_value[1].to_string()),
                "iyr" => iyr = Some(key_value[1].to_string()),
                "eyr" => eyr = Some(key_value[1].to_string()),
                "hgt" => hgt = Some(key_value[1].to_string()),
                "hcl" => hcl = Some(key_value[1].to_string()),
                "ecl" => ecl = Some(key_value[1].to_string()),
                "pid" => pid = Some(key_value[1].to_string()),
                "cid" => cid = Some(key_value[1].to_string()),
                &_ => println!("Unknown sequence: {}", key_value[1])
            };
        }

        let passport = Passport {
            byr: byr, 
            iyr: iyr,
            eyr: eyr,
            hgt: hgt,
            hcl: hcl,
            ecl: ecl,
            pid: pid,
            cid: cid
        };
        passport
    }

    fn is_valid(&self) -> bool {
        if self.byr == None {
            return false;
        }
        if self.iyr == None {
            return false;
        }
        if self.eyr == None {
            return false;
        }
        if self.hgt == None {
            return false;
        }
        if self.hcl == None {
            return false;
        }
        if self.ecl == None {
            return false;
        }
        if self.pid == None {
            return false;
        }
        true
    }

    fn verify_birth_year(byr: String) -> bool {
        let birth_year = byr.parse::<u32>().unwrap();
        if birth_year < 1920 || birth_year > 2002 {
            println!("Birth year invalid");
            return false;
        }
        true
    }

    fn verify_height(hgt: String) -> bool {  
        let mut height = hgt.clone();
        if !height.ends_with("cm") && !height.ends_with("in") {
            println!("Height unit invalid");
            return false;
        }

        if height.ends_with("cm") {
            height = height.trim_end_matches("cm").to_string();
            let cms = height.parse::<u32>().unwrap();

            if cms < 150 || cms > 193 {
                println!("{}", cms);
                println!("Height cm invalid");
                return false;
            }
        }

        if height.ends_with("in") {
            height = height.trim_end_matches("in").to_string();
            let ins = height.parse::<u32>().unwrap();

            if ins < 59 || ins > 76 {
                println!("{}", ins);
                println!("Height in invalid");
                return false;
            }
        }
        true
    }

    fn verify_hair_color(hcl: String) -> bool {
        let hair_color = hcl.clone();
        if hair_color.len() != 7 || !hair_color.starts_with("#") {
            println!("Hair color invalid");
            return false;
        }
        let valid_characters = "0123456789abcdef";
        for c in 1..hair_color.len() {
            if !valid_characters.contains(hair_color.chars().nth(c).unwrap()) {
                println!("Hair color invalid");
                return false;
            }
        }
        true
    }

    fn verify_eye_color(ecl: String) -> bool {
        let eye_color = ecl.clone();
        let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !valid_eye_colors.iter().any(|&e| e == &eye_color) {
            println!("Eye color invalid");
            return false;
        }
        true
    }

    fn verify_passport_id(pid: String) -> bool {
        if pid.len() != 9 {
            return false;
        }

        for c in pid.chars() {
            if !c.is_digit(10) {
                println!("Passport ID invalid");
                return false;
            }
        }
        true
    }

    fn verify(&self) -> bool {
        if !Passport::verify_birth_year(self.byr.clone().unwrap()) {
            return false;
        }
        
        let issuer_year = self.iyr.clone().unwrap().parse::<u32>().unwrap();
        if issuer_year < 2010 || issuer_year > 2020 {
            println!("Issuer year invalid");
            return false;
        }

        let expiration_year = self.eyr.clone().unwrap().parse::<u32>().unwrap();
        if expiration_year < 2020 || expiration_year > 2030 {
            println!("Expiration year invalid");
            return false;
        }

        if !Passport::verify_height(self.hgt.clone().unwrap()) {
            return false;
        }

        if !Passport::verify_hair_color(self.hcl.clone().unwrap()) {
            return false;
        }

        if !Passport::verify_eye_color(self.ecl.clone().unwrap()) {
            return false;
        }

        if !Passport::verify_passport_id(self.pid.clone().unwrap()) {
            return false;
        }
        true
    }
}




pub fn run_day_four() {
    println!("Running Day four...");

    let file = File::open("day_four_input.txt").expect("Unable to open file");
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());

    let mut passports: Vec<Passport> = Vec::new();
    let mut raw_data = String::new();

    for line in lines {
        raw_data.push_str(&line);
        raw_data.push_str(" ");

        // Next passport
        if line.is_empty() {
            //println!("{}", raw_data);
            let passport = Passport::from_string(raw_data.clone());
            passports.push(passport);

            raw_data = String::new();
        }
    }

    part_one(&passports);
    part_two(&passports);
}

fn part_one(passports: &Vec<Passport>) {
    let mut valid_passports = 0;

    for passport in passports {
        if passport.is_valid() {
            valid_passports = valid_passports + 1;
        }
    }
    println!("Result: {}", valid_passports);
}


fn part_two(passports: &Vec<Passport>) {
    assert_eq!(Passport::verify_birth_year("2002".to_string()), true);
    assert_eq!(Passport::verify_birth_year("2003".to_string()), false);

    assert_eq!(Passport::verify_height("60in".to_string()), true);
    assert_eq!(Passport::verify_height("190cm".to_string()), true);
    assert_eq!(Passport::verify_height("190in".to_string()), false);
    assert_eq!(Passport::verify_height("190".to_string()), false);

    assert_eq!(Passport::verify_hair_color("#123abc".to_string()), true);
    assert_eq!(Passport::verify_hair_color("#123abz".to_string()), false);
    assert_eq!(Passport::verify_hair_color("123abc".to_string()), false);

    assert_eq!(Passport::verify_eye_color("brn".to_string()), true);
    assert_eq!(Passport::verify_eye_color("wat".to_string()), false);

    assert_eq!(Passport::verify_passport_id("000000001".to_string()), true);
    assert_eq!(Passport::verify_passport_id("0123456789".to_string()), false);


    let mut valid_passports = 0;

    for passport in passports {
        if passport.is_valid() && passport.verify() {
            valid_passports = valid_passports + 1;
        }
    }
    println!("Result: {}", valid_passports);
}

