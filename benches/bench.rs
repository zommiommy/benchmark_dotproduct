#![feature(test)]
extern crate test;
use dotproduct::*;

pub const SIZE: usize = 12 * 0x100_000;


#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_native(b: &mut Bencher) {
        // Optionally include some setup
        let x = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();
        let y = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();

        b.bytes = (SIZE * core::mem::size_of::<f32>()) as u64;

        b.iter(|| {
            native(black_box(&x), black_box(&y))
        });
    }

    #[bench]
    fn bench_native_par(b: &mut Bencher) {
        // Optionally include some setup
        let x = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();
        let y = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();

        b.bytes = (SIZE * core::mem::size_of::<f32>()) as u64;

        b.iter(|| {
            native_par(black_box(&x), black_box(&y))
        });
    }

    #[bench]
    fn bench_native_with_size_hint(b: &mut Bencher) {
        // Optionally include some setup
        let x = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();
        let y = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();

        b.bytes = (SIZE * core::mem::size_of::<f32>()) as u64;

        b.iter(|| {
            native_with_size_hint(black_box(&x), black_box(&y))
        });
    }

    #[bench]
    fn bench_simd(b: &mut Bencher) {
        // Optionally include some setup
        let x = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();
        let y = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();

        b.bytes = (SIZE * core::mem::size_of::<f32>()) as u64;

        b.iter(|| {
            simd(black_box(&x), black_box(&y))
        });
    }

    #[bench]
    fn bench_simd_unrolled4(b: &mut Bencher) {
        // Optionally include some setup
        let x = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();
        let y = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();

        b.bytes = (SIZE * core::mem::size_of::<f32>()) as u64;

        b.iter(|| {
            simd_unrolled4(black_box(&x), black_box(&y))
        });
    }


    #[bench]
    fn bench_simd_par(b: &mut Bencher) {
        // Optionally include some setup
        let x = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();
        let y = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();

        b.bytes = (SIZE * core::mem::size_of::<f32>()) as u64;

        b.iter(|| {
            simd_par(black_box(&x), black_box(&y))
        });
    }

    #[bench]
    fn bench_simd_par_better(b: &mut Bencher) {
        // Optionally include some setup
        let x = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();
        let y = (0..SIZE).map(|i| i as f32).collect::<Vec<f32>>();

        b.bytes = (SIZE * core::mem::size_of::<f32>()) as u64;

        b.iter(|| {
            simd_par_better(black_box(&x), black_box(&y))
        });
    }
}
