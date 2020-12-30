use std::fs::File;
use std::io::{BufRead, BufReader};


struct Policy {
    min: u32,
    max: u32,
    character: char,
    password: String
}


impl Policy {
    fn is_valid_part_one(&self) -> bool {
        let mut count = 0;

        for letter in self.password.chars() {
            if letter == self.character {
                count = count + 1;
            }
        }

        if count >= self.min && count <= self.max {
            return true;
        }
        false
    }

    fn is_valid_part_two(&self) -> bool {
        let first = self.password.chars().nth((self.min - 1) as usize).unwrap();
        let last = self.password.chars().nth((self.max - 1) as usize).unwrap();

        if (first == self.character && last != self.character) ||
           (first != self.character && last == self.character) {
               return true;
           }
        false
    }

    fn parse_line(line: String) -> Policy {
        let tokens: Vec<&str> = line.split(" ").collect();

        let password = tokens.last().expect("Empty vector");
        let character = tokens[1].chars().nth(0).expect("Unable to detect character");

        let range: Vec<&str> = tokens[0].split("-").collect();
        let min = range[0].parse::<u32>().unwrap();
        let max = range[1].parse::<u32>().unwrap();

        Policy { min: min, max: max, character: character, password: password.to_string()}
    }
}




pub fn run_day_two() {
    println!("Running Day two...");

    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("day_two_input.txt").expect("Unable to open file");
    let lines = BufReader::new(file).lines();

    let raw_data = lines.map(|l| l.unwrap());

    let mut valid_passwords = 0;

    for line in raw_data {
        //println!("{}", line);

        let policy = Policy::parse_line(line);

        if policy.is_valid_part_one() {
            valid_passwords = valid_passwords + 1;
        }
    }
    println!("Result: {}", valid_passwords);
}


fn part_two() {
    let file = File::open("day_two_input.txt").expect("Unable to open file");
    let lines = BufReader::new(file).lines();

    let raw_data = lines.map(|l| l.unwrap());

    let mut valid_passwords = 0;

    for line in raw_data {
        //println!("{}", line);

        let policy = Policy::parse_line(line);

        if policy.is_valid_part_two() {
            valid_passwords = valid_passwords + 1;
        }
    }
    println!("Result: {}", valid_passwords);
}