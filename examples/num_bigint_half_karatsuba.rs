extern crate rust_timeit;
use num_bigint_org::{BigUint, RandBigInt};
use num_bigint_dev::{BigUint as DevBigUint};
use rand::rngs::OsRng;
use rust_timeit::cmp;
use std::time::Duration;
use std::hint::black_box;

fn f1(a: &BigUint, b: &BigUint) {
    let _ = black_box(a * b);
}

fn f2(a: &DevBigUint, b: &DevBigUint) {
    let _ = black_box(a * b);
}

fn main() {
    let mut rng = OsRng;

    println!("b_big_digits,c_big_digits,time_b,time_c,executions,performance_ratio");

    // Array of residuals to measure with significant_figures = 2
    let residuals = [(432, 688), (16, 16), (32, 1616), (176, 160), (32, 96)];

    // Measure specific residuals
    for _ in 0..3 { // Repeat the measurement three times
        for (b_digits, c_digits) in residuals {
            measure(&mut rng, b_digits, c_digits, 2, Duration::from_secs(10));
        }
    }

    // Normal measurement loop with significant_figures = 1
    for _ in 0..3 { // Repeat the measurement three times
        for n in 1..=128 {
            for m in 1..=128 {
                measure(&mut rng, n * 16, m * 16, 1, Duration::from_secs(1));
            }
        }
    }
}

fn measure(rng: &mut OsRng, b_big_digits: u16, c_big_digits: u16, significant_figures: usize, timeout: Duration) {
    const BITS: u64 = 64;
    let b: BigUint = RandBigInt::gen_biguint(rng, b_big_digits as u64 * BITS);
    let c: BigUint = RandBigInt::gen_biguint(rng, c_big_digits as u64 * BITS);

    let b_dev = DevBigUint::parse_bytes(b.to_str_radix(16).as_bytes(), 16).unwrap();
    let c_dev = DevBigUint::parse_bytes(c.to_str_radix(16).as_bytes(), 16).unwrap();

    let mut executions = 1;
    let min_time = 10000;

    let (time_b, time_c, executions) = cmp(
        || f1(&b, &c),
        || f2(&b_dev, &c_dev),
        &mut executions,
        timeout,
        min_time,
        significant_figures,
    );
    let performance_ratio = time_b as f64 / time_c as f64;

    println!("{},{},{},{},{},{}", b_big_digits, c_big_digits, time_b, time_c, executions, performance_ratio);
}
