use leetcode::add_digits::{add_digits, add_digits_digital_root};

#[test]
fn example_1() {
    // 38 -> 3 + 8 = 11 -> 1 + 1 = 2
    assert_eq!(add_digits(38), 2);
}

#[test]
fn example_2() {
    assert_eq!(add_digits(0), 0);
}

#[test]
fn single_digit() {
    assert_eq!(add_digits(5), 5);
    assert_eq!(add_digits(9), 9);
}

#[test]
fn multiple_of_nine() {
    // Digital root of any positive multiple of 9 is 9.
    assert_eq!(add_digits(9), 9);
    assert_eq!(add_digits(18), 9);
    assert_eq!(add_digits(99), 9);
    assert_eq!(add_digits(123456789), 9);
}

#[test]
fn ten() {
    assert_eq!(add_digits(10), 1);
}

#[test]
fn large_value() {
    // 2147483647 -> 2+1+4+7+4+8+3+6+4+7 = 46 -> 10 -> 1
    assert_eq!(add_digits(2147483647), 1);
}

#[test]
fn digital_root_matches_examples() {
    assert_eq!(add_digits_digital_root(38), 2);
    assert_eq!(add_digits_digital_root(0), 0);
    assert_eq!(add_digits_digital_root(10), 1);
    assert_eq!(add_digits_digital_root(2147483647), 1);
}

#[test]
fn both_implementations_agree() {
    for n in 0..=10_000 {
        assert_eq!(add_digits(n), add_digits_digital_root(n), "mismatch at {n}");
    }
}
