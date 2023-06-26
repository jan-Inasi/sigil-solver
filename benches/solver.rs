use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sigils::board::{Board, TetrisShape};
use sigils::{solve, TetrisSet};

fn solve_8_6(c: &mut Criterion) {
    let mut board = Board::new(8, 6);
    let mut blocks = TetrisSet::new();
    blocks.add(TetrisShape::O, 3);
    blocks.add(TetrisShape::Z, 3);
    blocks.add(TetrisShape::L, 4);
    blocks.add(TetrisShape::J, 2);

    c.bench_function("solve 8 6", |b| {
        b.iter(|| solve(black_box(&mut board), black_box(&mut blocks)))
    });
}

fn solve_8_8(c: &mut Criterion) {
    let mut board = Board::new(8, 8);
    let mut blocks = TetrisSet::new();
    blocks.add(TetrisShape::O, 8);
    blocks.add(TetrisShape::Z, 4);
    blocks.add(TetrisShape::L, 4);

    c.bench_function("solve 8 8", |b| {
        b.iter(|| solve(black_box(&mut board), black_box(&mut blocks)))
    });
}

criterion_group!(benches, solve_8_6, solve_8_8);
criterion_main!(benches);
