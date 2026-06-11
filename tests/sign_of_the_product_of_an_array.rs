use leetcode::sign_of_the_product_of_an_array::array_sign;

#[test]
fn example_1() {
    assert_eq!(array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
}

#[test]
fn example_2() {
    assert_eq!(array_sign(vec![1, 5, 0, 2, -3]), 0);
}

#[test]
fn example_3() {
    assert_eq!(array_sign(vec![-1, 1, -1, 1, -1]), -1);
}

#[test]
fn single_positive() {
    assert_eq!(array_sign(vec![7]), 1);
}

#[test]
fn single_negative() {
    assert_eq!(array_sign(vec![-7]), -1);
}

#[test]
fn single_zero() {
    assert_eq!(array_sign(vec![0]), 0);
}

#[test]
fn even_negatives() {
    assert_eq!(array_sign(vec![-2, -8]), 1);
}

#[test]
fn odd_negatives() {
    assert_eq!(array_sign(vec![-2, -8, -3]), -1);
}

#[test]
fn zero_dominates_negatives() {
    assert_eq!(array_sign(vec![-1, -1, -1, 0]), 0);
}

#[test]
fn large_values_no_overflow() {
    // Multiplying these out would overflow i32; the sign must still be -1.
    assert_eq!(array_sign(vec![100000, 100000, -100000]), -1);
}
