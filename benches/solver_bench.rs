use connect_four_engine::{Game, Solver};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

struct BenchData<'a> {
    name: &'a str,
    moves: &'a [usize],
}

const END_EASY: BenchData = BenchData {
    name: "end_easy",
    moves: &[
        1, 1, 4, 1, 4, 6, 5, 1, 4, 2, 3, 5, 1, 1, 3, 3, 0, 0, 0, 4, 5, 2, 2, 5, 4, 2, 3, 2, 5, 6,
        0, 2, 4, 0, 3, 3, 0,
    ],
};

const MIDDLE_EASY: BenchData = BenchData {
    name: "middle_easy",
    moves: &[
        4, 4, 4, 3, 1, 1, 3, 2, 2, 2, 1, 2, 3, 4, 0, 0, 6, 5, 3, 3, 0, 4, 0, 0, 4,
    ],
};

const _MIDDLE_MEDIUM: BenchData = BenchData {
    name: "middle_medium",
    moves: &[1, 6, 3, 4, 4, 1, 1, 1, 3, 0, 2, 0, 5, 5, 0],
};

const _BEGIN_EASY: BenchData = BenchData {
    name: "begin_easy",
    moves: &[2, 1, 0, 5, 3, 5, 1, 4],
};

const _BEGIN_MEDIUM: BenchData = BenchData {
    name: "begin_medium",
    moves: &[2, 1, 6, 4, 0, 4, 6, 0, 1, 2, 0, 4, 4, 6],
};

const _BEGIN_HARD: BenchData = BenchData {
    name: "begin_hard",
    moves: &[0, 2, 6, 0, 1],
};

const _FULL_SEARCH: BenchData = BenchData {
    name: "full_search",
    moves: &[],
};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("solve");

    for data in [END_EASY, MIDDLE_EASY] {
        let mut game = Game::new();
        if !data.moves.is_empty() {
            game.play_moves(data.moves).expect("Invalid moves");
        }

        group.throughput(Throughput::Elements(data.moves.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(data.name), &game, |b, game| {
            b.iter(|| Solver::solve(game.clone()));
        });
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
