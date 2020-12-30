use std::fs::File;
use std::io::{BufRead, BufReader};



pub fn run_day_three() {
    println!("Running Day three...");

    part_one();
    part_two();
}


fn part_one() {
    println!("Part one...");
    make_a_ride(3, 1);
}

fn part_two() {
    println!("Part two...");
    let trees_1_1 = make_a_ride(1, 1);
    let trees_1_3 = make_a_ride(3, 1);
    let trees_1_5 = make_a_ride(5, 1);
    let trees_1_7 = make_a_ride(7, 1);
    let trees_2_1 = make_a_ride(1, 2);

    let result = trees_1_1 * trees_1_3 * trees_1_5 * trees_1_7 * trees_2_1;
    println!("Total Result: {}", result);
}


fn make_a_ride(step_size: usize, step_size_y: usize) -> u32 {
    let file = File::open("day_three_input.txt").expect("Unable to open file");
    let lines = BufReader::new(file).lines();

    let raw_data = lines.map(|l| l.unwrap());

    let mut tree_count = 0;
    let mut x = 0;
    let mut line_number = 0;

    for line in raw_data {
        line_number = line_number + 1;
        if step_size_y > 1 && line_number % step_size_y == 0 {
            continue;
        }

        let char_count = line.chars().count();

        if x >= char_count {
            x = x - char_count;
        }

        let character = line.chars().nth(x).unwrap();

        if character == '#' {
            tree_count = tree_count + 1;
        }
        x = x + step_size;
    }
    println!("Result: {}", tree_count);
    tree_count
}