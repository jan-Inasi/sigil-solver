use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sigils::board::{Board, Tetris};
use sigils::solve;

fn solve_8_6(c: &mut Criterion) {
    let mut board = Board::new(8, 6);
    let mut blocks = Vec::new();
    blocks.extend(vec![Tetris::o_default(); 3].into_iter());
    blocks.extend(vec![Tetris::z_default(); 3].into_iter());
    blocks.extend(vec![Tetris::l_default(); 4].into_iter());
    blocks.extend(vec![Tetris::j_default(); 2].into_iter());

    c.bench_function("solve 8 6", |b| {
        b.iter(|| solve(black_box(&mut board), black_box(&mut blocks)))
    });
}

fn solve_8_9(c: &mut Criterion) {
    let mut board = Board::new(8, 8);
    let mut blocks = Vec::new();
    blocks.extend(vec![Tetris::o_default(); 8].into_iter());
    blocks.extend(vec![Tetris::z_default(); 4].into_iter());
    blocks.extend(vec![Tetris::l_default(); 4].into_iter());

    c.bench_function("solve 8 9", |b| {
        b.iter(|| solve(black_box(&mut board), black_box(&mut blocks)))
    });
}

criterion_group!(benches, solve_8_6, solve_8_9);
criterion_main!(benches);
