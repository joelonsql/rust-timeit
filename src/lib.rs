use std::time::Duration;
use minstant::Instant;

mod measure;

pub use measure::measure;

pub fn cmp<F, G>(
    func_a: F,
    func_b: G,
    executions: &mut usize,
    timeout: Duration,
    min_time: u128,
    significant_figures: usize,
) -> (u128, u128, usize)
where
    F: Fn() -> (),
    G: Fn() -> (),
{
    let start = Instant::now();
    let mut total_time_a;
    let mut total_time_b;

    loop {
        let time_a_1 = measure(&func_a, *executions);
        let time_b_1 = measure(&func_b, *executions);
        let time_a_2 = measure(&func_a, *executions);
        let time_b_2 = measure(&func_b, *executions);

        total_time_a = (time_a_1 + time_a_2) / 2;
        total_time_b = (time_b_1 + time_b_2) / 2;

        let ratio_1 = time_a_1 as f64 / time_b_1 as f64;
        let ratio_2 = time_a_2 as f64 / time_b_2 as f64;

        if time_a_1 > min_time && time_b_1 > min_time &&
           time_a_2 > min_time && time_b_2 > min_time &&
           ratios_are_equal_to_sig_figs(ratio_1, ratio_2, significant_figures) &&
           times_are_disjoint(time_a_1, time_a_2, time_b_1, time_b_2) ||
           start.elapsed() > timeout {
            break;
        } else {
            *executions *= 2; // Doubling executions for the next iteration
        }
    }

    (total_time_a, total_time_b, *executions)
}

fn times_are_disjoint(a1: u128, a2: u128, b1: u128, b2: u128) -> bool {
    let min_a = a1.min(a2);
    let max_a = a1.max(a2);
    let min_b = b1.min(b2);
    let max_b = b1.max(b2);

    max_a < min_b || min_a > max_b
}

fn ratios_are_equal_to_sig_figs(ratio_1: f64, ratio_2: f64, significant_figures: usize) -> bool {
    round_to_sig_figs(ratio_1, significant_figures) == round_to_sig_figs(ratio_2, significant_figures)
}

fn round_to_sig_figs(value: f64, figs: usize) -> f64 {
    if value == 0.0 {
        return 0.0;
    }
    let log_base_10 = value.abs().log10().floor();
    let scale = 10f64.powi((figs as i32) - 1 - log_base_10 as i32);
    (value * scale).round() / scale
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_times_are_disjoint_no_overlap() {
        assert!(times_are_disjoint(1, 2, 3, 4));
        assert!(times_are_disjoint(3, 4, 1, 2));
    }

    #[test]
    fn test_times_are_disjoint_with_overlap() {
        assert!(!times_are_disjoint(1, 3, 2, 4));
        assert!(!times_are_disjoint(2, 4, 1, 3));
    }

    #[test]
    fn test_ratios_are_equal_to_sig_figs_true() {
        assert!(ratios_are_equal_to_sig_figs(1.12345, 1.12349, 4));
        assert!(ratios_are_equal_to_sig_figs(0.0012345, 0.0012349, 4));
    }

    #[test]
    fn test_ratios_are_equal_to_sig_figs_false() {
        assert!(!ratios_are_equal_to_sig_figs(1.12345, 1.12351, 4));
        assert!(!ratios_are_equal_to_sig_figs(1.1234, 1.1239, 4));
    }

    #[test]
    fn test_round_to_sig_figs() {
        assert_eq!(round_to_sig_figs(1.2345, 3), 1.23);
        assert_eq!(round_to_sig_figs(0.00012345, 3), 0.000123);
        assert_eq!(round_to_sig_figs(98765.4321, 2), 99000 as f64);
    }
}
