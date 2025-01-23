# 🚀 Rust Dot Product Performance Benchmark

## 📊 Overview

This project provides a comprehensive benchmark of dot product implementations in Rust, comparing different computational approaches across multiple architectures.

## 🔬 Benchmark Configurations

### Implementations

- **Native Rust**: Pure Rust implementation using iterator methods
- **Nalgebra (OpenBLAS)**: Optimized linear algebra implementation leveraging BLAS

### Benchmark Parameters

- **Vector Size**: 1,000,000 elements
- **Sample Size**: 1,000 iterations
- **Confidence Level**: 99%
- **Measurement Time**: 10 seconds per benchmark

## 🖥️ Architectures Tested

- x86_64 (Intel/AMD 64-bit)
- ARM64 (aarch64)

## 📈 Performance Visualization

### x86_64 Performance

![x86_64 Architecture Benchmark](assets/x86_64-unknown-linux-gnu_violins.svg)

### ARM64 Performance

![ARM64 Architecture Benchmark](assets/aarch64-unknown-linux-gnu_violins.svg)
