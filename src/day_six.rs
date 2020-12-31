use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::Iterator;
use std::collections::HashMap;


pub fn run_day_six() {
    println!("Running Day six...");

    let file = File::open("day_six_input.txt").expect("Unable to open file");
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();


    // part one
    let groups = collect_groups_yes(&lines);

    let sum: u32 = groups.iter().sum();
    println!("Result: {}", sum);

    // part two
    let groups = collect_groups_all_yes(&lines);

    let sum: u32 = groups.iter().sum();
    println!("Result: {}", sum);

}

fn collect_groups_all_yes(lines: &Vec<String>) -> Vec<u32> {
    let mut groups_yes = Vec::<u32>::new();
    let mut data = String::new();
    let mut number_of_lines = 0;

    for line in lines {
        data.push_str(&line);
        number_of_lines = number_of_lines + 1;

        if line.is_empty() {
            let group_yes = parse_group_all_data(data, number_of_lines - 1);
            groups_yes.push(group_yes);

            data = String::new();
            number_of_lines = 0;
        }
    }
    groups_yes
}

fn parse_group_all_data(data: String, number_of_lines: u32) -> u32 {
    let mut unique_chars = HashMap::new();
    
    for c in data.chars() {
        if unique_chars.contains_key(&c) {
            // increase count
            let value = unique_chars.entry(c).or_insert(0);
            *value += 1;
        } else {
            unique_chars.insert(c, 1);
        }

    }

    //println!("{} - {:?}", number_of_lines, unique_chars);

    let mut yes_count = 0;

    for (_, i) in unique_chars {
        if i == number_of_lines {
            yes_count = yes_count + 1;
        }
    }
    yes_count
}



fn collect_groups_yes(lines: &Vec<String>) -> Vec<u32> {
    let mut groups_yes = Vec::<u32>::new();
    let mut data = String::new();

    for line in lines {
        data.push_str(&line);

        if line.is_empty() {
            let group_yes = parse_group_data(data);
            groups_yes.push(group_yes);

            data = String::new();
        }
    }
    groups_yes
}

fn parse_group_data(data: String) -> u32 {
    let mut unique_chars = Vec::<char>::new();
    
    for c in data.chars() {
        if !unique_chars.contains(&c) {
            unique_chars.push(c);
        }
    }
    unique_chars.len() as u32
}
