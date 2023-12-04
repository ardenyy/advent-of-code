use std::{fs, str::FromStr};

static INPUT_1: &str = "day-03/src/input1.txt";
static INPUT_2: &str = "day-03/src/input2.txt";

fn main() {
    let input1 = fs::read_to_string(INPUT_1).expect("");
    let input2 = fs::read_to_string(INPUT_2).expect("");
    let result1 = part_one(input1);
    let result2 = part_two(input2);

    println!("Result 1: {result1}\nResult 2: {result2}");
}

fn part_one(input: String) -> i32 {
    let mut array_2d: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut total = 0;
    let mut used_cells:Vec<(usize, usize)> = Vec::new();
    for row in 0..140 {
        for col in 0..140 {
            if !array_2d[row][col].is_digit(10) && array_2d[row][col] != '.' {
                
                for y in 0..3 {
                    for x in 0..3 {
                        if array_2d[row + y -1][col + x -1].is_digit(10) {
                            if used_cells.contains(&(row + y, col + x)) {
                                continue;
                            }
                            used_cells.push((row + y -1, col + x -1));
                            let mut number = array_2d[row + y -1][col + x -1].to_string();
                            let mut i = 10; // plus 10
                            while array_2d[row + y -1 + i - 10][col + x -1 + i - 10].is_digit(10) {
                                if used_cells.contains(&(row + y -1+ i - 10, col + x -1 + i - 10)) {
                                    i -= 1;
                                    continue;
                                }
                                used_cells.push((row + y -1 + i - 10, col + x -1 + i - 10));
                                let digit = array_2d[row + y -1 + i - 10][col + x -1 + i - 10];
                                number.insert(0, digit);
                                i -= 1;
                            }
                            while array_2d[row + y + i - 10][col + x -1 + i - 10].is_digit(10) {
                                if used_cells.contains(&(row + y -1 + i - 10, col + x -1+ i - 10)) {
                                    i -= 1;
                                    continue;
                                }
                                used_cells.push((row + y -1 + i - 10, col + x -1 + i - 10));
                                let digit = array_2d[row + y -1 + i - 10][col + x -1 + i - 10];
                                number.insert(number.len()-1, digit);
                                i += 1;
                            }
                            total += number.parse::<i32>().unwrap();
                        }
                    }
                }
            }
        }
    }

    total
}

fn part_two(input: String) -> i32 {
    0
}
