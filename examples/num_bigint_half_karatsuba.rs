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

    println!("b_exp,c_exp,time_b,time_c,executions");

    for _ in 0..3 {
        for b_exp in 1..=17 {
            for c_exp in 1..=17 {
                let b: BigUint = RandBigInt::gen_biguint(&mut rng, 1 << b_exp);
                let c: BigUint = RandBigInt::gen_biguint(&mut rng, 1 << c_exp);

                let b_dev = DevBigUint::parse_bytes(b.to_str_radix(16).as_bytes(), 16).unwrap();
                let c_dev = DevBigUint::parse_bytes(c.to_str_radix(16).as_bytes(), 16).unwrap();

                let mut executions = 1;
                let timeout = Duration::from_secs(1);
                let min_time = 10000;
                let significant_figures = 1;

                let (time_b, time_c, executions) = cmp(
                    || f1(&b, &c),
                    || f2(&b_dev, &c_dev),
                    &mut executions,
                    timeout,
                    min_time,
                    significant_figures,
                );

                println!("{},{},{},{},{}", b_exp, c_exp, time_b, time_c, executions);
            }
        }
    }
}