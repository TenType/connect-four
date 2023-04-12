use connect_four_engine::{Engine, Game};
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

const NUM_LINES: usize = 1000;

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
            t.lower_bound, t.median, t.upper_bound
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
    pub median: Duration,
    pub upper_bound: Duration,
}

fn bench_file(file_name: &str) -> BenchTimes {
    let path = format!("./test_data/{file_name}.csv");

    let mut times = [Duration::ZERO; NUM_LINES];

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut engine = Engine::new();

    // Benchmark each line, skipping the header
    for (i, line) in reader.lines().skip(1).enumerate() {
        times[i] = bench_line(line.unwrap(), &mut engine);

        let percent_complete: f64 = (i as f64 / NUM_LINES as f64 * 100.0).floor();
        print!("{EDIT}{PROGRESS_COLOR}{file_name:>15}{RESET} Running {i}/{NUM_LINES} {BOLD}({percent_complete}%){RESET}");
        stdout().flush().unwrap();
    }

    times.sort_unstable();

    BenchTimes {
        lower_bound: times[0],
        median: times[NUM_LINES / 2],
        upper_bound: times[NUM_LINES - 1],
    }
}

fn bench_line(line: String, engine: &mut Engine) -> Duration {
    let items: Vec<&str> = line.split(',').collect();

    let [moves, expected] = items[..] else {
        panic!("file line should have moves and score separated by a comma");
    };

    let expected: i8 = expected.parse().unwrap();

    let game = Game::from_str(moves).expect("move string should be valid");

    let now = Instant::now();
    let actual = engine.evaluate(game);
    let time = now.elapsed();

    assert_eq!(
        expected, actual,
        "input = {moves}, expected = {expected}, actual = {actual}"
    );

    time
}
