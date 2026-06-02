use leetcode::number_of_steps_to_reduce_a_number_to_zero::number_of_steps;

#[test]
fn example_1() {
    // 14 -> 7 -> 6 -> 3 -> 2 -> 1 -> 0
    assert_eq!(number_of_steps(14), 6);
}

#[test]
fn example_2() {
    // 8 -> 4 -> 2 -> 1 -> 0
    assert_eq!(number_of_steps(8), 4);
}

#[test]
fn example_3() {
    assert_eq!(number_of_steps(123), 12);
}

#[test]
fn zero() {
    assert_eq!(number_of_steps(0), 0);
}

#[test]
fn one() {
    assert_eq!(number_of_steps(1), 1);
}

#[test]
fn two() {
    // 2 -> 1 -> 0
    assert_eq!(number_of_steps(2), 2);
}

#[test]
fn power_of_two() {
    // 16 -> 8 -> 4 -> 2 -> 1 -> 0
    assert_eq!(number_of_steps(16), 5);
}

#[test]
fn odd_seven() {
    // 7 -> 6 -> 3 -> 2 -> 1 -> 0
    assert_eq!(number_of_steps(7), 5);
}

#[test]
fn large_value() {
    // constraint upper bound 10^6
    assert_eq!(number_of_steps(1_000_000), 26);
}
