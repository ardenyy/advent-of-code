use std::fs;

static INPUT_1: &str = "day-01/src/input1.txt";

fn main() {
    let input1 = fs::read_to_string(INPUT_1).expect("");
    
    let mut first = '0';
    let mut last = '0';
    let mut total = 0;
    for c in input1.chars() {
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
    println!("{total}");
}
