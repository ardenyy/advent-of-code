use std::fs;
use regex::Regex;

static INPUT_1: &str = "day-02/src/input1.txt";
static INPUT_2: &str = "day-02/src/input2.txt";


fn main() {
    let result1 = part_one();
    let result2 = part_two();

    println!("Result 1: {result1}\nResult 2: {result2}");
}

fn part_one() -> i32 {
    const RED_MAX: i32 = 12;
    const GREEN_MAX: i32 = 13;
    const BLUE_MAX: i32 = 14;

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
                let max = get_limit(color, RED_MAX, GREEN_MAX, BLUE_MAX).unwrap();
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

fn get_limit(input: &str, rmax: i32, gmax: i32, bmax: i32) -> Option<i32> {
    if input.contains("red") {
        Some(rmax)
    }
    else if input.contains("green") {
        Some(gmax)
    }
    else if input.contains("blue") {
        Some(bmax)
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
    
    let mut total = 0;
    for line in input.lines() {
        let split = line.split_once(':').unwrap();

        let mut rmax = 0;
        let mut gmax = 0;
        let mut bmax = 0;

        let subsets = split.1.split(';');
        for subset in subsets {
            for color_str in subset.split(',') {
                let number = extract_number(color_str).unwrap();
                let color = get_color(color_str).unwrap();

                if matches!(color, Color::Red) && number > rmax {
                    rmax = number;
                }
                if matches!(color, Color::Green) && number > gmax {
                    gmax = number;
                }
                if matches!(color, Color::Blue) && number > bmax {
                    bmax = number;
                }
            }
        }
        total += rmax * gmax * bmax;
    }
    
    total
}

fn get_color(input: &str) -> Option<Color> {
    if input.contains("red") {
        Some(Color::Red)
    }
    else if input.contains("green") {
        Some(Color::Green)
    }
    else if input.contains("blue") {
        Some(Color::Blue)
    }
    else {
        None
    }
}

enum Color {
    Red, Green, Blue,
}
