use leetcode::maximum_average_subarray_i::find_max_average;

/// Floating-point answers within 1e-5 of the expected value are accepted.
fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-5,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn example_1() {
    assert_close(find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75);
}

#[test]
fn example_2() {
    assert_close(find_max_average(vec![5], 1), 5.0);
}

#[test]
fn window_is_whole_array() {
    assert_close(find_max_average(vec![1, 2, 3, 4], 4), 2.5);
}

#[test]
fn k_equals_one_picks_max_element() {
    assert_close(find_max_average(vec![4, 0, 9, 2], 1), 9.0);
}

#[test]
fn all_negative() {
    assert_close(find_max_average(vec![-1, -2, -3, -4], 2), -1.5);
}

#[test]
fn best_window_is_at_the_end() {
    assert_close(find_max_average(vec![0, 0, 0, 7, 8], 2), 7.5);
}

#[test]
fn handles_extreme_values_without_overflow() {
    assert_close(find_max_average(vec![10000, 10000, 10000], 3), 10000.0);
    assert_close(find_max_average(vec![-10000, -10000, -10000], 3), -10000.0);
}
