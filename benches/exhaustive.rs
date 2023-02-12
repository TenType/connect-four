use connect_four_engine::{Game, Solver};
use std::cmp::{max, min};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

const TEST_DATA: &[&str] = &[
    "end_easy",
    "middle_easy",
    "middle_medium",
    "begin_easy",
    "begin_medium",
    "begin_hard",
];

const EDIT: &str = "\r";
const PROGRESS_COLOR: &str = "\x1b[1;34m";
const DONE_COLOR: &str = "\x1b[1;32m";
const BOLD: &str = "\x1b[1;37m";
const RESET: &str = "\x1b[0m";

fn main() {
    for file_name in TEST_DATA {
        progress_prefix(file_name);
        let t = bench_file(file_name);
        done_prefix(file_name);
        println!(
            "{:<9.3?} < {BOLD}{:^9.3?}{RESET} < {:>9.3?}",
            t.lower_bound, t.average, t.upper_bound
        );
    }

    // progress_prefix("full_search");
    // let time = bench_vec(vec![], 1);
    // done_prefix("full_search");
    // println!(
    //     "{NA:<9} < {BOLD}{:^9.3?}{RESET} < {NA:>9}",
    //     time,
    //     NA = "N/A",
    // );
}

fn progress_prefix(name: &str) {
    print!("{PROGRESS_COLOR}{name:>15}{RESET} Starting...");
    stdout().flush().unwrap();
}

fn done_prefix(name: &str) {
    print!("{EDIT}{DONE_COLOR}{name:>15}{RESET} ");
}

struct BenchTimes {
    pub lower_bound: Duration,
    pub average: Duration,
    pub upper_bound: Duration,
}

fn bench_file(file_name: &str) -> BenchTimes {
    let path = format!("./test_data/{file_name}.txt");

    let file = File::open(path.clone()).unwrap();
    let reader = BufReader::new(file);
    let line_count = reader.lines().count();

    let mut lower_bound = Duration::MAX;
    let mut upper_bound = Duration::ZERO;
    let mut time_sum = Duration::ZERO;

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let time = bench_line(line.unwrap());

        lower_bound = min(lower_bound, time);
        upper_bound = max(upper_bound, time);

        time_sum += time;

        let percent_complete: f64 = (i as f64 / line_count as f64 * 100.0).floor();
        print!("{EDIT}{PROGRESS_COLOR}{file_name:>15}{RESET} Running {i}/{line_count} {BOLD}({percent_complete}%){RESET}");
        stdout().flush().unwrap();
    }

    let average = time_sum / line_count.try_into().unwrap();

    BenchTimes {
        lower_bound,
        average,
        upper_bound,
    }
}

fn bench_line(line: String) -> Duration {
    let items: Vec<&str> = line.split(' ').take(2).collect();

    let [moves, expected] = items[..] else {
        panic!("file line should have two strings separated by a space");
    };

    let expected: i8 = expected.parse().unwrap();

    let mut game = Game::new();
    game.play_str(moves).expect("invalid move string");

    let now = Instant::now();
    let actual = Solver::solve(game);
    let time = now.elapsed();

    assert_eq!(
        expected, actual,
        "input = {moves}, expected = {expected}, actual = {actual}"
    );

    time
}
