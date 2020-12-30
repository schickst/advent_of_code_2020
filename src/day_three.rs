use std::fs::File;
use std::io::{BufRead, BufReader};



pub fn run_day_three() {
    println!("Running Day three...");

    part_one();
    part_two();
}


fn part_one() {
    let file = File::open("day_three_input.txt").expect("Unable to open file");
    let lines = BufReader::new(file).lines();

    let raw_data = lines.map(|l| l.unwrap());


}

fn part_two() {

}