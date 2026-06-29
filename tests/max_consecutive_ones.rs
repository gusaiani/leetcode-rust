use leetcode::max_consecutive_ones::find_max_consecutive_ones;

#[test]
fn example_1() {
    // [1,1,0,1,1,1] -> longest run is the trailing three 1s
    assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
}

#[test]
fn example_2() {
    assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
}

#[test]
fn all_ones() {
    assert_eq!(find_max_consecutive_ones(vec![1, 1, 1, 1]), 4);
}

#[test]
fn all_zeros() {
    assert_eq!(find_max_consecutive_ones(vec![0, 0, 0]), 0);
}

#[test]
fn single_one() {
    assert_eq!(find_max_consecutive_ones(vec![1]), 1);
}

#[test]
fn single_zero() {
    assert_eq!(find_max_consecutive_ones(vec![0]), 0);
}

#[test]
fn run_at_start() {
    assert_eq!(find_max_consecutive_ones(vec![1, 1, 1, 0, 1]), 3);
}

#[test]
fn alternating() {
    assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 0, 1]), 1);
}
