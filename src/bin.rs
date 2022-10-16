#![feature(bench_black_box)]
use dotproduct::*;
use std::hint::black_box;
use std::arch::x86_64::_rdtsc;

pub fn rdtsc() -> u64 {
    unsafe{_rdtsc()}
}

macro_rules! benchmark {
    ($size: expr, $a:expr, $b: expr, $($f:expr),*) => {$(

let n = 1_000;
let mut moment1 = 0;
let mut moment2 = 0;

for _ in 0..n {
    let start = rdtsc();
    let _ = black_box($f(black_box(&$a), black_box(&$b)));
    let end = rdtsc();
    moment1 += end - start;
    moment2 += (end - start).pow(2);
}

let moment1 = moment1 as f64 / n as f64;
let moment2 = moment2 as f64 / n as f64;

println!("{},{},{},{}", stringify!($f), $size, moment1, moment2 - (moment1 * moment1));
    )*};
}

pub fn main() {
    for i in 1..4 {
        let size = 8 * i;
        let a = (0..size).map(|x| x as f32).collect::<Vec<_>>();
        let b = (0..size).map(|x| x as f32).collect::<Vec<_>>();

        benchmark!(
            size, a, b, 
            native,
            native_with_size_hint,
            simd_f32x8,
            native_par
        );
    }

    for i in 1..rayon::current_num_threads() {
        let size = 32 * i;
        let a = (0..size).map(|x| x as f32).collect::<Vec<_>>();
        let b = (0..size).map(|x| x as f32).collect::<Vec<_>>();

        benchmark!(
            size, a, b, 
            native,
            native_with_size_hint,
            simd_f32x8,
            simd_unrolled4_f32x8,
            native_par
        );
    }

    for i in 1..1_00 {
        let size = i * 32 * rayon::current_num_threads();
        let a = (0..size).map(|x| x as f32).collect::<Vec<_>>();
        let b = (0..size).map(|x| x as f32).collect::<Vec<_>>();

        benchmark!(
            size, a, b, 
            native,
            native_with_size_hint,
            simd_f32x8,
            simd_f32x16,
            simd_unrolled4_f32x8,
            simd_unrolled4_f32x16,
            native_par,
            simd_par,
            simd_par_better
        );
    }

    for i in 1..1_00 {
        let size = i * 3200 * rayon::current_num_threads();
        let a = (0..size).map(|x| x as f32).collect::<Vec<_>>();
        let b = (0..size).map(|x| x as f32).collect::<Vec<_>>();

        benchmark!(
            size, a, b, 
            native,
            native_with_size_hint,
            simd_f32x8,
            simd_f32x16,
            simd_unrolled4_f32x8,
            simd_unrolled4_f32x16,
            native_par,
            simd_par,
            simd_par_better
        );
    }

    for i in 1..1_00 {
        let size = i * 320000 * rayon::current_num_threads();
        let a = (0..size).map(|x| x as f32).collect::<Vec<_>>();
        let b = (0..size).map(|x| x as f32).collect::<Vec<_>>();

        benchmark!(
            size, a, b, 
            native,
            native_with_size_hint,
            simd_f32x8,
            simd_f32x16,
            simd_unrolled4_f32x8,
            simd_unrolled4_f32x16,
            native_par,
            simd_par,
            simd_par_better
        );
    }
}