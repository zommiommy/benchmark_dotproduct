#![feature(portable_simd)]
use std::simd::f32x8;
use std::convert::TryInto;
use std::simd::{StdFloat, SimdFloat};
use rayon::prelude::*;

pub fn native(vec1: &[f32], vec2: &[f32]) -> f32 {
    vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum()
}

pub fn native_par(vec1: &[f32], vec2: &[f32]) -> f32 {
    vec1.par_iter().zip(vec2.par_iter()).map(|(a, b)| a * b).sum()
}


pub fn native_with_size_hint(vec1: &[f32], vec2: &[f32]) -> f32 {
    if vec1.len() != vec2.len() {
        unsafe{
            std::hint::unreachable_unchecked();
        }
    }
    vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum()
}

pub fn simd(vec1: &[f32], vec2: &[f32]) -> f32 {
    if vec1.len() != vec2.len() {
        unsafe{
            std::hint::unreachable_unchecked();
        }
    }

    let mut total = f32x8::splat(0.0);
    for (a, b) in vec1.chunks(8).zip(vec2.chunks(8)) {
        let a = f32x8::from_array(a.try_into().unwrap());
        let b = f32x8::from_array(b.try_into().unwrap());

        total = f32x8::mul_add(a, b, total);
    }

    total.reduce_sum()
}

pub fn simd_unrolled4(vec1: &[f32], vec2: &[f32]) -> f32 {
    if vec1.len() != vec2.len() {
        unsafe{
            std::hint::unreachable_unchecked();
        }
    }

    let mut total0 = f32x8::splat(0.0);
    let mut total1 = f32x8::splat(0.0);
    let mut total2 = f32x8::splat(0.0);
    let mut total3 = f32x8::splat(0.0);
    for (a, b) in vec1.chunks(8 * 4).zip(vec2.chunks(8 * 4)) {
        let a0 = f32x8::from_array(unsafe{a[0..8].try_into().unwrap_unchecked()});
        let b0 = f32x8::from_array(unsafe{b[0..8].try_into().unwrap_unchecked()});
        total0 = f32x8::mul_add(a0, b0, total0);
        let a1 = f32x8::from_array(unsafe{a[8..16].try_into().unwrap_unchecked()});
        let b1 = f32x8::from_array(unsafe{b[8..16].try_into().unwrap_unchecked()});
        total1 = f32x8::mul_add(a1, b1, total0);
        let a2 = f32x8::from_array(unsafe{a[16..24].try_into().unwrap_unchecked()});
        let b2 = f32x8::from_array(unsafe{b[16..24].try_into().unwrap_unchecked()});
        total2 = f32x8::mul_add(a2, b2, total2);
        let a3 = f32x8::from_array(unsafe{a[24..32].try_into().unwrap_unchecked()});
        let b3 = f32x8::from_array(unsafe{b[24..32].try_into().unwrap_unchecked()});
        total3 = f32x8::mul_add(a3, b3, total3);
    }

    (total0 + total1 + total2 + total3).reduce_sum()
}


pub fn simd_par(vec1: &[f32], vec2: &[f32]) -> f32 {
    
    if vec1.len() != vec2.len() {
        unsafe{
            std::hint::unreachable_unchecked();
        }
    }

    vec1.par_chunks(8 * 4).zip(vec2.par_chunks(8 * 4)).map(|(a, b)| {
        let a0 = f32x8::from_array(unsafe{a[0..8].try_into().unwrap_unchecked()});
        let b0 = f32x8::from_array(unsafe{b[0..8].try_into().unwrap_unchecked()});
        let mut total = a0 + b0;
        let a1 = f32x8::from_array(unsafe{a[8..16].try_into().unwrap_unchecked()});
        let b1 = f32x8::from_array(unsafe{b[8..16].try_into().unwrap_unchecked()});
        total = f32x8::mul_add(a1, b1, total);
        let a2 = f32x8::from_array(unsafe{a[16..24].try_into().unwrap_unchecked()});
        let b2 = f32x8::from_array(unsafe{b[16..24].try_into().unwrap_unchecked()});
        total = f32x8::mul_add(a2, b2, total);
        let a3 = f32x8::from_array(unsafe{a[24..32].try_into().unwrap_unchecked()});
        let b3 = f32x8::from_array(unsafe{b[24..32].try_into().unwrap_unchecked()});
        total = f32x8::mul_add(a3, b3, total);
        total
    }).reduce(||f32x8::splat(0.0) , |a, b| a + b).reduce_sum()

}

pub fn simd_par_better(vec1: &[f32], vec2: &[f32]) -> f32 {
    
    if vec1.len() != vec2.len(){
        unsafe{
            std::hint::unreachable_unchecked();
        }
    }

    let n_threads = rayon::current_num_threads();

    assert!(vec1.len() % (n_threads * 32) == 0);
    assert!(vec2.len() % (n_threads * 32) == 0);

    let chunk_size = vec1.len() / n_threads;

    (0..n_threads).map(|i| {
        simd_unrolled4(&vec1[(i * chunk_size)..((i + 1) * chunk_size)], &vec2[(i * chunk_size)..((i + 1) * chunk_size)]) 
    }).sum()
}
