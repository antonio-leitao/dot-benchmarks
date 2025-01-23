use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dotzilla;
use nalgebra::DVector;
use rand::Rng;

mod implementations {
    use super::*;

    // f64 implementations
    pub fn native_dot_f64(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b).map(|(x, y)| x * y).sum()
    }

    pub fn nalgebra_dot_f64(a: &DVector<f64>, b: &DVector<f64>) -> f64 {
        a.dot(b)
    }

    pub fn dotzilla_dot_f64(a: &[f64], b: &[f64]) -> f64 {
        dotzilla::dot_product(a, b)
    }

    // f32 implementations
    pub fn native_dot_f32(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(x, y)| x * y).sum()
    }

    pub fn nalgebra_dot_f32(a: &DVector<f32>, b: &DVector<f32>) -> f32 {
        a.dot(b)
    }

    pub fn dotzilla_dot_f32(a: &[f32], b: &[f32]) -> f32 {
        dotzilla::dot_product(a, b)
    }
}

fn bench_dot_products(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let size = 1_000_000;

    // Benchmark f64 implementations
    {
        let a_f64: Vec<f64> = (0..size).map(|_| rng.gen()).collect();
        let b_f64: Vec<f64> = (0..size).map(|_| rng.gen()).collect();
        let a_nalg_f64 = DVector::from_vec(a_f64.clone());
        let b_nalg_f64 = DVector::from_vec(b_f64.clone());

        let mut group = c.benchmark_group("f64");
        group.sample_size(1000);
        group.confidence_level(0.99);
        group.measurement_time(std::time::Duration::from_secs(10));

        group.bench_function("native Rust", |b| {
            b.iter(|| implementations::native_dot_f64(black_box(&a_f64), black_box(&b_f64)))
        });

        group.bench_function("nalgebra", |b| {
            b.iter(|| {
                implementations::nalgebra_dot_f64(black_box(&a_nalg_f64), black_box(&b_nalg_f64))
            })
        });

        group.bench_function("dotzilla", |b| {
            b.iter(|| implementations::dotzilla_dot_f64(black_box(&a_f64), black_box(&b_f64)))
        });
    }

    // Benchmark f32 implementations
    {
        let a_f32: Vec<f32> = (0..size).map(|_| rng.gen()).collect();
        let b_f32: Vec<f32> = (0..size).map(|_| rng.gen()).collect();
        let a_nalg_f32 = DVector::from_vec(a_f32.clone());
        let b_nalg_f32 = DVector::from_vec(b_f32.clone());

        let mut group = c.benchmark_group("f32");
        group.sample_size(1000);
        group.confidence_level(0.99);
        group.measurement_time(std::time::Duration::from_secs(10));

        group.bench_function("native", |b| {
            b.iter(|| implementations::native_dot_f32(black_box(&a_f32), black_box(&b_f32)))
        });

        group.bench_function("nalgebra", |b| {
            b.iter(|| {
                implementations::nalgebra_dot_f32(black_box(&a_nalg_f32), black_box(&b_nalg_f32))
            })
        });

        group.bench_function("dotzilla", |b| {
            b.iter(|| implementations::dotzilla_dot_f32(black_box(&a_f32), black_box(&b_f32)))
        });
    }
}

criterion_group!(benches, bench_dot_products);
criterion_main!(benches);
