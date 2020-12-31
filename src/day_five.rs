use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn run_day_five() {
    println!("Running Day five...");

    let file = File::open("day_five_input.txt").expect("Unable to open file");
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    assert_eq!(to_row_number("BFFFBBF"), 70);
    assert_eq!(to_row_number("FFFBBBF"), 14);
    assert_eq!(to_row_number("BBFFBBF"), 102);

    assert_eq!(to_column_number("RRR"), 7);
    assert_eq!(to_column_number("RRR"), 7);
    assert_eq!(to_column_number("RLL"), 4);

    assert_eq!(get_seat(70, 7), 567);
    assert_eq!(get_seat(14, 7), 119);
    assert_eq!(get_seat(102, 4), 820);
    
    //BFFFBBFRRR: row 70, column 7, seat ID 567.
    //FFFBBBFRRR: row 14, column 7, seat ID 119.
    //BBFFBBFRLL: row 102, column 4, seat ID 820.


    let mut seat_numbers = collect_seat_numbers(lines);
    seat_numbers.sort();

    let lowest_seat_number = seat_numbers.iter().min().unwrap();
    let highest_seat_number = seat_numbers.iter().max().unwrap();

    println!("Result: Lowest Seat {}", lowest_seat_number);
    println!("Result: Highest Seat {}", highest_seat_number);


    for n in *lowest_seat_number..*highest_seat_number {
        if !seat_numbers.contains(&n) {
            println!("Result Seat number: {}", n);
        }
    }
}

fn collect_seat_numbers(lines: Vec<String>) -> Vec<i32> {
    lines.into_iter()
         .map(|line| get_seat_from_line(line))
         .collect()        
}

fn get_seat_from_line(line: String) -> i32 {
    let (_, _, seat) = parse_boarding_pass(&line);
    seat
}


fn parse_boarding_pass(boarding_pass: &str) -> (i8, i8, i32) {

    let (row, column) = boarding_pass.split_at(7);

    let row_number = to_row_number(row);
    let column_number = to_column_number(column);
    let seat = get_seat(row_number, column_number);

    (row_number, column_number, seat)
}


fn get_seat(row: i8, column: i8) -> i32 {
    return (row as i32 * 8) + column as i32;
}

fn to_column_number(column: &str) -> i8 {
    let mut raw_binary = column.replace('L', "0");
    raw_binary = raw_binary.replace('R', "1");

    let number = i8::from_str_radix(&raw_binary, 2);
    return number.unwrap();
}


fn to_row_number(row: &str) -> i8 {
    let mut raw_binary = row.replace('F', "0");
    raw_binary = raw_binary.replace('B', "1");

    let number = i8::from_str_radix(&raw_binary, 2);
    return number.unwrap();
}