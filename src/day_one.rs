use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;


pub fn run_day_one() {
    println!("Running Day one...");

    let file = File::open("day_one_input.txt").expect("Unable to open file");

    let lines = BufReader::new(file).lines();
    let map = lines.map(|l| l.unwrap().parse::<u32>().unwrap());

    let mut numbers = Vec::from_iter(map);
    numbers.sort();

    day_one_two_entries(&numbers);    
    day_one_three_entries(&numbers);
}

fn day_one_two_entries(numbers: &Vec<u32>) {
    for i in 0..numbers.len() {
        // println!("{}", numbers[i]);

        for j in i+1..numbers.len() {
            let result = numbers[i] + numbers[j];

            if result == 2020 {
                println!("{} - {}", numbers[i], numbers[j]);
                println!("Answer: {}", numbers[i] * numbers[j]);
            }
        }
    }
}

fn day_one_three_entries(numbers: &Vec<u32>) {
    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            for k in j+1..numbers.len() {
                let result = numbers[i] + numbers[j] + numbers[k];

                if result == 2020 {
                    println!("{} - {} - {}", numbers[i], numbers[j],  numbers[k]);
                    println!("Answer: {}", numbers[i] * numbers[j] * numbers[k]);
                }
            }
        }
    }
}