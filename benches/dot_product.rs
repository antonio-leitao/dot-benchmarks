use criterion::{black_box, criterion_group, criterion_main, Criterion};
use faer::prelude::*;
use nalgebra::DVector;
use rand::Rng;

mod implementations {
    use super::*;

    // Native Rust implementation
    pub fn native_dot(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b).map(|(x, y)| x * y).sum()
    }

    // nalgebra with OpenBLAS
    pub fn nalgebra_openblas_dot(a: &DVector<f64>, b: &DVector<f64>) -> f64 {
        a.dot(b) // Will use BLAS via nalgebra-lapack
    }

    // faer-rs implementation
    pub fn faer_dot(a: &Mat<f64>, b: &Mat<f64>) -> f64 {
        a.transpose().as_2d().dot(b.as_2d())
    }
}

fn bench_dot_product(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let size = 1_000_000; // Large enough to measure meaningful differences

    // Generate random vectors once
    let native_a: Vec<f64> = (0..size).map(|_| rng.gen()).collect();
    let native_b: Vec<f64> = (0..size).map(|_| rng.gen()).collect();

    let nalgebra_a = DVector::from_vec(native_a.clone());
    let nalgebra_b = DVector::from_vec(native_b.clone());

    let faer_a = Mat::from_fn(size, 1, |i, _| native_a[i]);
    let faer_b = Mat::from_fn(size, 1, |i, _| native_b[i]);

    let mut group = c.benchmark_group("Dot Product");
    group.sample_size(1000);
    group.confidence_level(0.99);
    group.measurement_time(std::time::Duration::from_secs(10));

    // Benchmark each implementation
    group.bench_function("Native Rust", |b| {
        b.iter(|| implementations::native_dot(black_box(&native_a), black_box(&native_b)))
    });

    group.bench_function("nalgebra (OpenBLAS)", |b| {
        b.iter(|| {
            implementations::nalgebra_openblas_dot(black_box(&nalgebra_a), black_box(&nalgebra_b))
        })
    });

    group.bench_function("faer-rs", |b| {
        b.iter(|| implementations::faer_dot(black_box(&faer_a), black_box(&faer_b)))
    });

    group.finish();
}

criterion_group!(benches, bench_dot_product);
criterion_main!(benches);
