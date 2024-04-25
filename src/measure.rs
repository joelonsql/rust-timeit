use minstant::Instant;
use std::hint::black_box;

/// Measures the execution time of a function over a specified number of executions.
/// 
/// # Parameters
/// - `func`: The function to measure.
/// - `executions`: The number of times to execute the function.
/// 
/// # Returns
/// Returns the total execution time in microseconds as u128.
pub fn measure<F>(func: F, executions: usize) -> u128
where
    F: Fn() -> (),
{
    let start = Instant::now();
    for _ in 0..executions {
        black_box(func());
    }
    let duration = start.elapsed();
    duration.as_micros()  // Return the execution time in microseconds
}
