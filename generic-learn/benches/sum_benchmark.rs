#[path = "../src/sum.rs"]
mod sum;

use sum::{sum1, sum2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sum1_bench(c: &mut Criterion){
    let input = black_box([10, 20, 30, 40]);
    
    c.bench_function("sum_1", |b| b.iter(|| sum1(&input)));

    c.bench_function("sum_2", |b| b.iter(|| sum2(&input)));
}

criterion_group!(benches, sum1_bench);
criterion_main!(benches);