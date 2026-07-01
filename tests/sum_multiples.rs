use leetcode::sum_multiples::sum_of_multiples;

#[test]
fn example_1() {
    // 3, 5, 6, 7 are the multiples of 3, 5, or 7 in [1, 7].
    assert_eq!(sum_of_multiples(7), 21);
}

#[test]
fn example_2() {
    // 3, 5, 6, 7, 9, 10 are the multiples in [1, 10] → 40.
    assert_eq!(sum_of_multiples(10), 40);
}

#[test]
fn example_3() {
    // 3, 5, 6, 7, 9 qualify in [1, 9] → 30.
    assert_eq!(sum_of_multiples(9), 30);
}

#[test]
fn smallest_input() {
    // n = 1: nothing in [1, 1] is divisible by 3, 5, or 7.
    assert_eq!(sum_of_multiples(1), 0);
}

#[test]
fn first_multiple() {
    // n = 3: only 3 qualifies.
    assert_eq!(sum_of_multiples(3), 3);
}

#[test]
fn overlap_not_double_counted() {
    // 15 is divisible by both 3 and 5 but must be counted once.
    // Multiples in [1, 15]: 3,5,6,7,9,10,12,14,15 = 81.
    assert_eq!(sum_of_multiples(15), 81);
}

#[test]
fn upper_bound() {
    // n = 1000 (problem constraint max).
    assert_eq!(sum_of_multiples(1000), 272066);
}
