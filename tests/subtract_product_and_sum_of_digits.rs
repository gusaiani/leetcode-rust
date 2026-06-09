use leetcode::subtract_product_and_sum_of_digits::subtract_product_and_sum;

#[test]
fn example_1() {
    // digits: 2, 3, 4 -> product 24, sum 9 -> 24 - 9 = 15
    assert_eq!(subtract_product_and_sum(234), 15);
}

#[test]
fn example_2() {
    // digits: 4, 4, 2, 1 -> product 32, sum 11 -> 32 - 11 = 21
    assert_eq!(subtract_product_and_sum(4421), 21);
}

#[test]
fn single_digit() {
    // digit: 5 -> product 5, sum 5 -> 0
    assert_eq!(subtract_product_and_sum(5), 0);
}

#[test]
fn single_digit_one() {
    assert_eq!(subtract_product_and_sum(1), 0);
}

#[test]
fn contains_zero_digit() {
    // digits: 1, 0, 2 -> product 0, sum 3 -> 0 - 3 = -3
    assert_eq!(subtract_product_and_sum(102), -3);
}

#[test]
fn all_nines() {
    // digits: 9, 9 -> product 81, sum 18 -> 63
    assert_eq!(subtract_product_and_sum(99), 63);
}
