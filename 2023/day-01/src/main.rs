use std::fs;

static INPUT_1: &str = "day-01/src/input1.txt";
static INPUT_2: &str = "day-01/src/input2.txt";

fn main() {
    let result1 = part_one();
    let result2 = part_two();

    println!("Result 1: {result1}\nResult 2: {result2}");
}

fn part_one() -> i32 {
    let input = fs::read_to_string(INPUT_1).expect("");
    
    let mut first = '0';
    let mut last = '0';
    let mut total = 0;
    for c in input.chars() {
        if c.is_digit(10) {
            last = c;
            if first == '0' {
                first = last;
            }
        }
        else if c == 0xA as char {
            let number = format!("{}{}", first, last);
            total += number.parse::<i32>().expect("");
            first = '0';
            last = '0';
        }
    }
    let number = format!("{}{}", first, last);
    total += number.parse::<i32>().expect("");

    total
}

fn part_two() -> i32 {
    let input = fs::read_to_string(INPUT_2).expect("");

    let numbers = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
    ];

    let mut first = '0';
    let mut last = '0';
    let mut total = 0;

    let mut log: String = "".to_string();

    for line in input.split("\n") {
        if line.contains("123456") {
            break;
        }
        let mut left_index = 0;
        for i in 1..line.len()  {
            let sub_str = &line[left_index..i];
            for tuple in numbers {
                if sub_str.contains(tuple.0) {
                    last = tuple.1;
                    if first == '0' {
                        first = last;
                    }
                    let below_i = i - 1;
                    if left_index < below_i {
                        left_index = below_i;
                    }
                    else {
                        left_index = i;
                    }
                    break;
                }
            }
            if i+1 == line.len() && left_index < i {
                while left_index  < i {
                    let sub_str = &line[left_index..i];
                    for tuple in numbers {
                        if sub_str.contains(tuple.0) {
                            last = tuple.1;
                            if first == '0' {
                                first = last;
                            }
                            break;
                        }
                    }
                    left_index += 1;
                }
            }
        }
        let number = format!("{}{}", first, last);
        total += number.parse::<i32>().expect("");
        first = '0';
        last = '0';
    }
    
    total
}
