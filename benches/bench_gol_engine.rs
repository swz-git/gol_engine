use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gol_engine::*;
use rand::Rng;

fn gen_random_board(size: i32, amount: usize) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut board: Vec<Point> = Vec::new();
    for _ in 0..amount {
        board.push(Point {
            x: rng.gen_range(0..size),
            y: rng.gen_range(0..size),
        });
    }
    return board;
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("tick 5000", |b| {
    //     b.iter(|| tick(black_box(&mut gen_random_board(100, 5000))))
    // });

    c.bench_function("tick 500", |b| {
        b.iter(|| tick(black_box(&mut gen_random_board(100, 500))))
    });

    c.bench_function("get_cells_to_compute 500", |b| {
        b.iter(|| get_cells_to_compute(black_box(&mut gen_random_board(100, 500))))
    });

    c.bench_function("get_living_cell_neighbors 500", |b| {
        b.iter(|| {
            get_living_cell_neighbors(
                black_box(Point { x: 10, y: 10 }),
                black_box(&mut gen_random_board(100, 500)),
            )
        })
    });

    // c.bench_function("tick 100", |b| {
    //     b.iter(|| tick(black_box(&mut gen_random_board(100, 100))))
    // });

    // c.bench_function("tick 20", |b| {
    //     b.iter(|| tick(black_box(&mut gen_random_board(100, 20))))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
