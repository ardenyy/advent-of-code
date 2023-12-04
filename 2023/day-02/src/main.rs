use std::fs;
use regex::Regex;

static INPUT_1: &str = "day-02/src/input1.txt";
static INPUT_2: &str = "day-02/src/input2.txt";


const RED_MAX: i32 = 12;
const GREEN_MAX: i32 = 13;
const BLUE_MAX: i32 = 14;

fn main() {
    let result1 = part_one();
    let result2 = part_two();

    println!("Result 1: {result1}\nResult 2: {result2}");
}

fn part_one() -> i32 {
    let input = fs::read_to_string(INPUT_1).expect("");

    let mut total = 0;
    for line in input.lines() {
        let split = line.split_once(':').unwrap();
        let game_id = extract_number(split.0).unwrap();

        let subsets = split.1.split(';');

        let mut error = false;
        for subset in subsets {
            for color in subset.split(',') {
                let number = extract_number(color).unwrap();
                let max = get_limit(color).unwrap();
                if number > max {
                    error = true;
                    break;
                }
            }
            if error {
                break;
            }
        }
        if !error {
            total += game_id;
        }
    }
    
    total
}

fn get_limit(input: &str) -> Option<i32> {
    if input.contains("red") {
        Some(RED_MAX)
    }
    else if input.contains("green") {
        Some(GREEN_MAX)
    }
    else if input.contains("blue") {
        Some(BLUE_MAX)
    }
    else {
        None
    }
}

fn extract_number(input: &str) -> Option<i32> {
    let re = Regex::new(r"\d+").unwrap();

    if let Some(m) = re.find(input) {
        Some(m.as_str().parse::<i32>().unwrap())
    } else {
        None
    }
}

fn part_two() -> i32 {
    let input = fs::read_to_string(INPUT_2).expect("");

    
    
    0
}
