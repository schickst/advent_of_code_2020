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

    let mut valid_passports = 0;

    for passport in passports {
        if passport.is_valid() {
            valid_passports = valid_passports + 1;
        }
        else {
            println!("{:?}", passport);
        }
    }
    println!("Result: {}", valid_passports);
}

