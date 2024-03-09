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

    println!(" ");

    let perf_02_start: Instant = Instant::now();
    let part_02_result: i32 = run_part_02().expect("Should have worked");
    println!("2nd PART RESULT: {}", part_02_result);
    let perf_02_duration: Duration = perf_02_start.elapsed();
    println!("2nd part execution time: {:.2?}", perf_02_duration);
}

trait IsValid {
    fn is_valid(&self) -> bool;
}

#[derive(Debug)]
struct Draw {
    game_id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl IsValid for Draw {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn run_part_01() -> io::Result<i32> {
    // https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
    let file = File::open("input.txt").expect("Should have been able to open file");
    let reader = BufReader::new(file);
    let mut total_sum: i32 = 0;

    for (current_index, line) in reader.lines().enumerate() {
        let line_to_check: String = line.expect("Should have gotten line");
        let draws_vec: Vec<Draw> = draws_from_string(&line_to_check, current_index as i32 + 1);
        if draws_vec.iter().all(|d| d.is_valid()) {
            total_sum += draws_vec.first().unwrap().game_id;
        }
    }

    Ok(total_sum)
}

struct Game {
    draws: Vec<Draw>,
}

trait CubeCount {
    fn req_blue(&self) -> i32;
    fn req_red(&self) -> i32;
    fn req_green(&self) -> i32;
    fn power(&self) -> i32;
}

impl CubeCount for Game {
    // This is quite inefficient because we are going to loop over the list of draws
    // 3 times instead of just 1. But the code is simple.
    fn req_blue(&self) -> i32 {
        self.draws.iter().map(|d| d.blue).max().unwrap_or(0)
    }

    fn req_red(&self) -> i32 {
        self.draws.iter().map(|d| d.red).max().unwrap_or(0)
    }

    fn req_green(&self) -> i32 {
        self.draws.iter().map(|d| d.green).max().unwrap_or(0)
    }

    fn power(&self) -> i32 {
        self.req_blue() * self.req_red() * self.req_green()
    }
}

fn run_part_02() -> io::Result<i32> {
    // https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
    let file = File::open("input.txt").expect("Should have been able to open file");
    let reader = BufReader::new(file);

    let mut games: Vec<Game> = Vec::new();
    for (current_index, line) in reader.lines().enumerate() {
        let line_to_check: String = line.expect("Should have gotten line");

        let game_id = current_index as i32 + 1;
        let game = Game {
            draws: draws_from_string(&line_to_check, game_id),
        };
        games.push(game);
    }
    Ok(games.iter().map(|g| g.power()).sum())
}

fn draws_from_string(draws_str: &str, game_id: i32) -> Vec<Draw> {
    let index_regex: Regex = Regex::new(r#"Game \d+:"#).unwrap();
    let blue_regex = Regex::new(r#"(?<blue>\d+) blue"#).unwrap();
    let red_regex = Regex::new(r#"(?<red>\d+) red"#).unwrap();
    let green_regex = Regex::new(r#"(?<green>\d+) green"#).unwrap();
    // We could have used the row index that comes from the file,
    // but we are using the loop index instead
    let str = index_regex.replace_all(draws_str, "");
    let draws_str = str.split(';');
    let mut draws_vec: Vec<Draw> = Vec::new();

    for g in draws_str {
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
        let draw = Draw {
            game_id,
            blue,
            red,
            green,
        };
        draws_vec.push(draw);
    }
    draws_vec
}
