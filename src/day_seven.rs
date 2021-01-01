extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::Iterator;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Rule {
    bag: String,
    contents: HashMap<String, u32>
}

pub fn run_day_seven() {
    println!("Running Day seven...");

    let test_lines = get_test_lines();
    let test_rules = get_rules(test_lines);

    let test_result = search_count(String::from("shiny gold"), &test_rules);
    assert_eq!(4, test_result.len());

    let lines = get_lines();
    let rules = get_rules(lines);
    let result = search_count(String::from("shiny gold"), &rules);

    println!("{}", result.len());
}

fn search_count(color: String, rules: &Vec<Rule>) -> Vec<String> {
    let mut bag_colors = Vec::<String>::new();


    for rule in rules.iter() {

        if check_rule(&color, &rule) {
            if !bag_colors.contains(&rule.bag) {
                bag_colors.push(rule.bag.clone());
            }

            let additional_bag_colors = search_count(rule.bag.clone(), rules);

            for additional_bag_color in additional_bag_colors {
                if !bag_colors.contains(&additional_bag_color) {
                    bag_colors.push(additional_bag_color);
                }
            }
        }
    }
    bag_colors
}

fn check_rule(color: &String, rule: &Rule) -> bool {
    for (c, n) in rule.contents.iter() {
        if c == color {
            println!("{:?}", rule);
            return true;
        }
    }
    false
}

fn get_rules(lines: Vec<String>) -> Vec<Rule> {
    let mut rules = Vec::new();

    for line in lines {
        let rule = parse_rule(line);
        rules.push(rule);
    }
    rules
}

fn get_lines() -> Vec<String> {
    let file = File::open("day_seven_input.txt").expect("Unable to open file");
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    lines
}


fn get_test_lines() -> Vec<String> {
    let lines = vec![
        String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
        String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
        String::from("bright white bags contain 1 shiny gold bag."),
        String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
        String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
        String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
        String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
        String::from("faded blue bags contain no other bags."),
        String::from("dotted black bags contain no other bags.") ];
    lines
}


fn parse_rule(input: String) -> Rule {
    //println!("{}", input);
    let outer_re = Regex::new(r"([0-9]* ?\w* \w* bags?)").unwrap();
    let inner_re = Regex::new(r"([0-9]*) ?(\w* \w*) bag.*").unwrap();

    let mut bag = String::new();
    let mut contents = HashMap::<String, u32>::new();

    for outer_capture in outer_re.captures_iter(&input) {
        let group = outer_capture.get(0).map_or("", |m| m.as_str());

        for inner_capture in inner_re.captures_iter(&group) {
            //println!("{:?}", inner_capture);
            let raw_number = inner_capture.get(1).map_or("", |m| m.as_str());
            let color = inner_capture.get(2).map_or("", |m| m.as_str());

            if raw_number.is_empty() {
                bag = color.to_string();
                continue;
            }

            let number = raw_number.parse::<u32>().unwrap();
            contents.insert(color.to_string(), number);
        }       
    }
    Rule { bag: bag, contents: contents }
}