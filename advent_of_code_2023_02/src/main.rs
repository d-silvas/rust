use core::panic;
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::time::{Duration, Instant};

fn main() {
    let perf_01_start: Instant = Instant::now();
    let part_01_result: i32 = run_part_01().expect("Should have worked");
    println!("1st PART RESULT: {}", part_01_result);
    let perf_01_duration: Duration = perf_01_start.elapsed();
    println!("1st part execution time: {:.2?}", perf_01_duration);

    // println!("");

    // let perf_02_start: Instant = Instant::now();
    // let part_02_result: i64 = run_part_02().expect("Should have worked");
    // println!("2nd PART RESULT: {}", part_02_result);
    // let perf_02_duration: Duration = perf_02_start.elapsed();
    // println!("2nd part execution time: {:.2?}", perf_02_duration);

    // println!("");

    // let perf_02_good_start: Instant = Instant::now();
    // let part_02_good_result: i64 = run_part_02_good().expect("Should have worked");
    // println!("2nd PART GOOD RESULT: {}", part_02_good_result);
    // let perf_02_good_duration: Duration = perf_02_good_start.elapsed();
    // println!(
    //     "2nd part good execution time: {:.2?}",
    //     perf_02_good_duration
    // );
}

trait IsValid {
    fn is_valid(&self) -> bool;
}

#[derive(Debug)]
struct Game {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl IsValid for Game {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn run_part_01() -> io::Result<i32> {
    // https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
    let file = File::open("input.txt").expect("Should have been able to open file");
    let reader = BufReader::new(file);
    let index_regex: Regex = Regex::new(r#"Game \d+:"#).unwrap();
    let blue_regex = Regex::new(r#"(?<blue>\d+) blue"#).unwrap();
    let red_regex = Regex::new(r#"(?<red>\d+) red"#).unwrap();
    let green_regex = Regex::new(r#"(?<green>\d+) green"#).unwrap();
    let mut total_sum: i32 = 0;

    for (current_index, line) in reader.lines().enumerate() {
        let line_to_check: String = line.expect("Should have gotten line");
        let line_to_check = index_regex.replace_all(&line_to_check, "");
        let games_str = line_to_check.split(';');
        let mut games_vec: Vec<Game> = Vec::new();
        for g in games_str {
            let blue = match blue_regex.captures(g) {
                Some(caps) => caps["blue"].parse::<i32>().unwrap(),
                None => 0_i32,
            };
            let red = match red_regex.captures(g) {
                Some(caps) => caps["red"].parse::<i32>().unwrap(),
                None => 0_i32,
            };
            let green = match green_regex.captures(g) {
                Some(caps) => caps["green"].parse::<i32>().unwrap(),
                None => 0_i32,
            };
            let game = Game {
                id: current_index as i32 + 1,
                blue,
                red,
                green,
            };
            games_vec.push(game);
        }
        if games_vec.iter().all(|g| g.is_valid()) {
            total_sum += games_vec.first().unwrap().id;
        }
    }

    Ok(total_sum)
}

/**
 * Finds single digits in a string
 *
 * See https://stackoverflow.com/questions/58010114/capture-all-regex-matches-into-a-vector
 */
fn find_numbers(re: &Regex, s: &str) -> Vec<i64> {
    // Iterate over all matches
    re.find_iter(s)
        // Try to parse the string matches as i64 (inferred from fn type signature)
        // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
        .filter_map(|digits| digits.as_str().parse().ok())
        // Collect the results in to a Vec<i64> (inferred from fn type signature)
        .collect()
}

/**
 * THIS DOES NOT WORK
 *
 * See https://stackoverflow.com/questions/77587365/overlapping-matches-in-regex-rust-regex-engine
 */
fn run_part_02() -> io::Result<i64> {
    // https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
    let file = File::open("input.txt").expect("Should have been able to open file");
    let reader = BufReader::new(file);
    let re = Regex::new(r#"(\d|one|two|three|four|five|six|seven|eight|nine){1}"#).unwrap();
    let mut total_sum: i64 = 0;

    for line in reader.lines() {
        let line_to_check: String = line.expect("Should have gotten line");
        let matches: Vec<String> = find_strings(&re, &line_to_check);
        let first_calibration_digit: &str = match_calibration_values(&matches.first().unwrap());
        let last_calibration_digit: &str = match_calibration_values(&matches.last().unwrap());
        // See https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
        let calibration_val_str: String =
            first_calibration_digit.to_owned() + last_calibration_digit;
        total_sum += calibration_val_str.parse::<i64>().unwrap();
    }

    Ok(total_sum)
}

fn find_strings(re: &Regex, s: &str) -> Vec<String> {
    re.find_iter(s)
        .filter_map(|matches| matches.as_str().parse().ok())
        .collect()
}

fn match_calibration_values(str_to_match: &String) -> &str {
    match str_to_match.as_str() {
        "1" | "one" => "1",
        "2" | "two" => "2",
        "3" | "three" => "3",
        "4" | "four" => "4",
        "5" | "five" => "5",
        "6" | "six" => "6",
        "7" | "seven" => "7",
        "8" | "eight" => "8",
        "9" | "nine" => "9",
        _ => panic!("Should have matched"),
    }
}

fn run_part_02_good() -> io::Result<i64> {
    // See https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
    // for another idea of implementation
    let file = File::open("input.txt").expect("Should have been able to open file");
    let reader = BufReader::new(file);
    let mut total_sum: i64 = 0;

    for line in reader.lines() {
        let line_to_check: String = line.expect("Should have gotten line");
        let (first_match, last_match): (Option<&str>, Option<&str>) = match_nums(&line_to_check);
        let fm = first_match.unwrap().to_owned();
        let lm: String;
        if last_match.is_none() {
            lm = fm.clone();
        } else {
            lm = last_match.unwrap().to_owned();
        }
        let first_calibration_digit = match_calibration_values(&fm);
        let last_calibration_digit = match_calibration_values(&lm);
        let calibration_val_str: String =
            first_calibration_digit.to_owned() + last_calibration_digit;
        total_sum += calibration_val_str.parse::<i64>().unwrap();
    }

    Ok(total_sum)
}

/**
 * Inspiration: https://www.reddit.com/r/adventofcode/comments/1883ibu/comment/kfl143d/?utm_source=share&utm_medium=web2x&context=3
 */
fn match_nums(line: &str) -> (Option<&str>, Option<&str>) {
    use std::mem::swap;
    const NUMBERS: &str = "|one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9|";
    const N: usize = NUMBERS.len();
    let bnumbers = NUMBERS.as_bytes();
    let mut dp1 = [usize::MAX; N];
    let mut dp2 = [usize::MAX; N];
    let mut first = None;
    let mut last = None;

    for b1 in line.bytes().chain([b'#']) {
        for (j, b2) in (1..).zip(NUMBERS.bytes()) {
            if b2 == b'|' && dp1[j - 1] != usize::MAX {
                let k = dp1[j - 1];
                if first.is_none() {
                    first = Some(&NUMBERS[k..j - 1]);
                } else {
                    last = Some(&NUMBERS[k..j - 1]);
                }
            } else if b1 == b2 {
                if bnumbers[j - 2] == b'|' {
                    dp2[j] = j - 1;
                } else {
                    dp2[j] = dp1[j - 1];
                }
            }
        }
        swap(&mut dp1, &mut dp2);
        dp2.fill(usize::MAX);
    }
    (first, last)
}
