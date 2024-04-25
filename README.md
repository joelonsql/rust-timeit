# rust-timeit 🚧

`rust-timeit` is designed to compare the execution time of two different implementations of the same function to determine which is faster under similar conditions. **Please note this is an unreleased work-in-progress project, and breaking changes may occur at any time.** ⚠️

## Installation

To install `rust-timeit`, you need to include it in your Rust project's `Cargo.toml` file. Additionally, before building the project, you must execute a pre-build script to set up necessary dependencies.

1. Add to `Cargo.toml`:

Add `rust-timeit` to your dependencies in `Cargo.toml`:

```toml
[dependencies]
rust-timeit = "0.1.0"
```

2. Run Pre-Build Script:

Execute the `pre-build.sh` script to prepare the project environment:

```sh
./pre-build.sh
```

3. Build the Project:

Use Cargo to build your project:

```sh
cargo build
```

## Usage

### Basic Usage

Here is a basic example of how to use `rust-timeit` to measure the execution time of a function:

```rust
extern crate rust_timeit;
use rust_timeit::measure;

fn sample_function() {
    // Your code here
}

fn main() {
    let executions = 10;
    let time = measure(sample_function, executions);
    println!("Total execution time: {} microseconds", time);
}
```

### Comparing Two Functions

To compare the performance of two functions:

```rust
extern crate rust_timeit;
use rust_timeit::cmp;
use std::time::Duration;

fn function_a() {
    // Function A code
}

fn function_b() {
    // Function B code
}

fn main() {
    let mut executions = 1;
    let timeout = Duration::from_secs(1);
    let min_time = 10000; // microseconds
    let significant_figures = 2;

    let (time_a, time_b, final_executions) = cmp(
        function_a,
        function_b,
        &mut executions,
        timeout,
        min_time,
        significant_figures,
    );

    println!("Function A time: {}μs, Function B time: {}μs, Executions: {}", time_a, time_b, final_executions);
}
```

## Real-Life Examples

### Benchmarking Optimizations in `num-bigint`

`rust-timeit` was initially developed to evaluate an optimisation in the [https://github.com/rust-num/num-bigint](https://github.com/rust-num/num-bigint) project.

To replicate the evaluation and visualize the performance gains:

```sh
cd examples
cargo run --release --example num_bigint_half_karatsuba > half_karatsuba.csv
./plot_half_karatsuba.py
```

![Half Karatsuba patch benchmark plot](examples/half_karatsuba.png?raw=true)

## License

Distributed under the terms of the MIT license, `rust-timeit` is free and open source software.
