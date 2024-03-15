use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::time::{Duration, Instant};

fn main() {
    let perf_01_start: Instant = Instant::now();
    let part_01_result: u32 = run_part_01().expect("Should have worked");
    println!("1st PART RESULT: {}", part_01_result);
    let perf_01_duration: Duration = perf_01_start.elapsed();
    println!("1st part execution time: {:.2?}", perf_01_duration);

    println!(" ");

    // let perf_02_start: Instant = Instant::now();
    // let part_02_result: i32 = run_part_02().expect("Should have worked");
    // println!("2nd PART RESULT: {}", part_02_result);
    // let perf_02_duration: Duration = perf_02_start.elapsed();
    // println!("2nd part execution time: {:.2?}", perf_02_duration);
}

#[derive(Debug)]
struct Number {
    digits: Vec<u32>,
    is_part_num: bool,
}

trait NumberTrait {
    fn add_digit(&mut self, d: u32);
    fn set_as_part_num(&mut self);
    fn to_string(&self) -> String;
    fn digits_to_int(&self) -> u32;
}

impl NumberTrait for Number {
    fn add_digit(&mut self, d: u32) {
        self.digits.push(d);
    }

    fn set_as_part_num(&mut self) {
        self.is_part_num = true;
    }

    fn digits_to_int(&self) -> u32 {
        let mut result: u32 = 0;
        for (digit_index, digit) in self.digits.iter().rev().enumerate() {
            result += digit * 10_u32.pow(digit_index as u32);
        }
        result
    }

    fn to_string(&self) -> String {
        self.digits.iter().map(|d| d.to_string()).join("") + "|" + &self.is_part_num.to_string()
    }
}

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

fn run_part_01() -> io::Result<u32> {
    let mut all_chars: Vec<Vec<char>> = Vec::new();
    let file = File::open("input.txt").expect("Should have been able to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_to_check: String = line.expect("Should have gotten line");
        let line_chars: Vec<char> = line_to_check.chars().collect();
        all_chars.push(line_chars);
    }

    let all_numbers = get_all_numbers(&all_chars);
    // println!("{:?}", all_numbers);
    Ok(all_numbers
        .iter()
        .filter(|n| n.is_part_num)
        .map(|n| n.digits_to_int())
        .sum())
}

/**
 * Please note: minimum size of all_chars is 2x2
 */
fn get_all_numbers(all_chars: &Vec<Vec<char>>) -> Vec<Number> {
    let mut all_numbers: Vec<Number> = Vec::new();
    for (row_index, row) in all_chars.iter().enumerate() {
        let mut is_prev_char_a_digit = false;
        for (col_index, ch) in row.iter().enumerate() {
            if *ch == '.' {
                is_prev_char_a_digit = false;
                continue;
            }
            match ch.to_digit(10) {
                Some(num) => {
                    let curr_position = Position {
                        row: row_index,
                        col: col_index,
                    };
                    if is_prev_char_a_digit {
                        let last_num = all_numbers.last_mut().unwrap();
                        last_num.add_digit(num);
                        if is_adjacent_to_symbol_num(&curr_position, &all_chars) {
                            last_num.set_as_part_num();
                        }
                    } else {
                        let is_part_num = is_adjacent_to_symbol_num(&curr_position, &all_chars);
                        all_numbers.push(Number {
                            digits: vec![num],
                            is_part_num,
                        })
                    }
                    is_prev_char_a_digit = true;
                }
                None => {
                    is_prev_char_a_digit = false;
                }
            }
        }
    }
    all_numbers
}

fn is_adjacent_to_symbol_num(position: &Position, all_chars: &Vec<Vec<char>>) -> bool {
    let last_row: usize = all_chars.len() - 1;
    let last_col: usize = all_chars[0].len() - 1;
    let curr_row: usize = position.row;
    let curr_col: usize = position.col;
    let mut positions_to_check: Vec<Position> = Vec::new();
    if curr_row == 0 {
        if curr_col == 0 {
            positions_to_check.push(Position { row: 0, col: 1 });
            positions_to_check.push(Position { row: 1, col: 0 });
            positions_to_check.push(Position { row: 1, col: 1 });
        } else if curr_col == last_col {
            positions_to_check.push(Position {
                row: 0,
                col: last_col - 1,
            });
            positions_to_check.push(Position {
                row: 1,
                col: last_col,
            });
            positions_to_check.push(Position {
                row: 1,
                col: last_col - 1,
            });
        } else {
            positions_to_check.push(Position {
                row: 0,
                col: curr_col - 1,
            });
            positions_to_check.push(Position {
                row: 0,
                col: curr_col + 1,
            });
            positions_to_check.push(Position {
                row: 1,
                col: curr_col - 1,
            });
            positions_to_check.push(Position {
                row: 1,
                col: curr_col,
            });
            positions_to_check.push(Position {
                row: 1,
                col: curr_col + 1,
            });
        }
    } else if curr_row == last_row {
        if curr_col == 0 {
            positions_to_check.push(Position {
                row: last_row - 1,
                col: 0,
            });
            positions_to_check.push(Position {
                row: last_row - 1,
                col: 1,
            });
            positions_to_check.push(Position {
                row: last_row,
                col: 1,
            });
        } else if curr_col == last_col {
            positions_to_check.push(Position {
                row: last_row - 1,
                col: last_col,
            });
            positions_to_check.push(Position {
                row: last_row - 1,
                col: last_col - 1,
            });
            positions_to_check.push(Position {
                row: last_row,
                col: last_col - 1,
            });
        } else {
            positions_to_check.push(Position {
                row: last_row,
                col: curr_col - 1,
            });
            positions_to_check.push(Position {
                row: last_row,
                col: curr_col + 1,
            });
            positions_to_check.push(Position {
                row: last_row - 1,
                col: curr_col - 1,
            });
            positions_to_check.push(Position {
                row: last_row - 1,
                col: curr_col,
            });
            positions_to_check.push(Position {
                row: last_row - 1,
                col: curr_col + 1,
            });
        }
    } else {
        #[allow(clippy::collapsible_else_if)]
        if curr_col == 0 {
            positions_to_check.push(Position {
                row: curr_row - 1,
                col: 0,
            });
            positions_to_check.push(Position {
                row: curr_row - 1,
                col: 1,
            });
            positions_to_check.push(Position {
                row: curr_row,
                col: 1,
            });
            positions_to_check.push(Position {
                row: curr_row + 1,
                col: 0,
            });
            positions_to_check.push(Position {
                row: curr_row + 1,
                col: 1,
            });
        } else if curr_col == last_col {
            positions_to_check.push(Position {
                row: curr_row - 1,
                col: last_col - 1,
            });
            positions_to_check.push(Position {
                row: curr_row - 1,
                col: last_col,
            });
            positions_to_check.push(Position {
                row: curr_row,
                col: last_col - 1,
            });
            positions_to_check.push(Position {
                row: curr_row + 1,
                col: last_col - 1,
            });
            positions_to_check.push(Position {
                row: curr_row + 1,
                col: last_col,
            });
        } else {
            positions_to_check.push(Position {
                row: curr_row - 1,
                col: curr_col - 1,
            });
            positions_to_check.push(Position {
                row: curr_row - 1,
                col: curr_col,
            });
            positions_to_check.push(Position {
                row: curr_row - 1,
                col: curr_col + 1,
            });
            positions_to_check.push(Position {
                row: curr_row,
                col: curr_col - 1,
            });
            positions_to_check.push(Position {
                row: curr_row,
                col: curr_col + 1,
            });
            positions_to_check.push(Position {
                row: curr_row + 1,
                col: curr_col - 1,
            });
            positions_to_check.push(Position {
                row: curr_row + 1,
                col: curr_col,
            });
            positions_to_check.push(Position {
                row: curr_row + 1,
                col: curr_col + 1,
            });
        }
    }
    positions_to_check
        .iter()
        .any(|p: &Position| is_symbol(all_chars[p.row][p.col]))
}

fn is_symbol(ch: char) -> bool {
    // We are compiling the Regex on every call...
    let symbol_regex = Regex::new(r"[^\d\.\s]").unwrap();
    symbol_regex.is_match(&ch.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_digits_to_int_works() {
        let num = Number {
            digits: vec![0, 1, 0, 2, 3, 4, 0],
            is_part_num: false,
        };
        assert_eq!(num.digits_to_int(), 102340);
    }

    #[test]
    fn it_works_2x2() {
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![vec!['.', '.'], vec!['.', '.']])),
            ""
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![vec!['1', '.'], vec!['.', '.']])),
            "1|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![vec!['1', '.'], vec!['3', '.']])),
            "1|false,3|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![vec!['1', '.'], vec!['.', '4']])),
            "1|false,4|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![vec!['1', '2'], vec!['3', '4']])),
            "12|false,34|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![vec!['1', '2'], vec!['#', '4']])),
            "12|true,4|true"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![vec!['!', '2'], vec!['3', '4']])),
            "2|true,34|true"
        );
    }

    #[test]
    fn it_works_3x3() {
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
            ])),
            ""
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['1', '.', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
            ])),
            "1|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['.', '1', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
            ])),
            "1|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['.', '.', '1'],
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
            ])),
            "1|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['.', '.', '.'],
                vec!['1', '.', '.'],
                vec!['.', '.', '.'],
            ])),
            "1|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['.', '.', '.'],
                vec!['.', '1', '.'],
                vec!['.', '.', '.'],
            ])),
            "1|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['.', '.', '.'],
                vec!['.', '.', '1'],
                vec!['.', '.', '.'],
            ])),
            "1|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
                vec!['1', '.', '.'],
            ])),
            "1|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
                vec!['.', '1', '.'],
            ])),
            "1|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '1'],
            ])),
            "1|false"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['1', '2', '.'],
                vec!['.', '.', '#'],
                vec!['.', '.', '.'],
            ])),
            "12|true"
        );
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['7', '.', '.'],
                vec!['.', '8', '?'],
                vec!['1', '2', '3'],
            ])),
            "7|false,8|true,123|true"
        );
    }

    #[test]
    fn it_works_example() {
        assert_eq!(
            nums_to_string(&get_all_numbers(&vec![
                vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
                vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
                vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
                vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
                vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
                vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
                vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
            ])),
            "467|true,114|false,35|true,633|true,617|true,58|false,592|true,755|true,664|true,598|true"
        );
    }

    fn nums_to_string(all_numbers: &Vec<Number>) -> String {
        all_numbers.iter().map(|n| n.to_string()).join(",")
    }
}
