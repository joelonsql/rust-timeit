# rust-timeit 🚧

`rust-timeit` is designed to facilitate precise time measurements and comparisons between different implementations of the same functionality. **Please note this is an unreleased work-in-progress project, and breaking changes may occur at any time.** ⚠️

## Features

- **Measure Time** ⏱️: Functionality to measure the execution time of a Rust function over a specified number of executions.

- **Compare Functions** 🔍: Ability to compare the execution time of two different functions to determine which is faster under similar conditions.

## Installation

Add `rust-timeit` to your `Cargo.toml` dependencies:

```toml
[dependencies]
rust-timeit = "0.1.0"
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

## License

Distributed under the terms of the MIT license, `rust-timeit` is free and open source software.
