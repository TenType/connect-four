use connect_four_engine::{Game, Score, Solver};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn test_file(file_name: &str) {
    let path = format!("./tests/data/{file_name}.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let items: Vec<&str> = line.split(' ').take(2).collect();

        if let [move_str, expected_str] = items[..] {
            let moves: Vec<usize> = move_str
                .chars()
                .map(|c| c.to_digit(10).expect("Not a digit") as usize)
                .collect();

            let mut game = Game::new();
            game.play_moves(&moves).expect("Invalid moves");

            let actual = Solver::solve(game);
            let expected: Score = expected_str.parse().unwrap();

            assert_eq!(
                expected, actual,
                "input = {moves:?}, expected = {expected}, actual = {actual}"
            );
        } else {
            panic!("File line should have 2 items");
        }
    }
}

#[test]
fn begin_easy() {
    test_file("begin_easy");
}

#[test]
fn begin_medium() {
    test_file("begin_medium");
}

#[test]
fn begin_hard() {
    test_file("begin_hard");
}

#[test]
fn middle_easy() {
    test_file("middle_easy");
}

#[test]
fn middle_medium() {
    test_file("middle_medium");
}

#[test]
fn end_easy() {
    test_file("end_easy");
}
