use leetcode::richest_customer_wealth::maximum_wealth;

#[test]
fn example_1() {
    assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
}

#[test]
fn example_2() {
    assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10);
}

#[test]
fn example_3() {
    assert_eq!(
        maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
        17
    );
}

#[test]
fn single_customer_single_bank() {
    assert_eq!(maximum_wealth(vec![vec![10]]), 10);
}

#[test]
fn single_bank_many_customers() {
    assert_eq!(maximum_wealth(vec![vec![4], vec![2], vec![9]]), 9);
}

#[test]
fn richest_is_first_row() {
    assert_eq!(maximum_wealth(vec![vec![6, 6], vec![1, 1], vec![2, 2]]), 12);
}

#[test]
fn all_zero_balances() {
    assert_eq!(maximum_wealth(vec![vec![0, 0], vec![0, 0]]), 0);
}
