use leetcode::find_numbers_with_even_number_of_digits::find_numbers;

#[test]
fn example_1() {
    // 12 -> 2 digits, 345 -> 3, 2 -> 1, 6 -> 1, 7896 -> 4 => 2 numbers
    assert_eq!(find_numbers(vec![12, 345, 2, 6, 7896]), 2);
}

#[test]
fn example_2() {
    // 555 -> 3, 901 -> 3, 482 -> 3, 1771 -> 4 => 1 number
    assert_eq!(find_numbers(vec![555, 901, 482, 1771]), 1);
}

#[test]
fn single_one_digit() {
    assert_eq!(find_numbers(vec![5]), 0);
}

#[test]
fn single_two_digit() {
    assert_eq!(find_numbers(vec![10]), 1);
}

#[test]
fn boundary_powers_of_ten() {
    // 9 -> 1, 10 -> 2, 99 -> 2, 100 -> 3, 1000 -> 4 => 3 numbers
    assert_eq!(find_numbers(vec![9, 10, 99, 100, 1000]), 3);
}

#[test]
fn max_constraint_value() {
    // 100000 has 6 digits (even)
    assert_eq!(find_numbers(vec![100000]), 1);
}

#[test]
fn all_even_digit_counts() {
    assert_eq!(find_numbers(vec![11, 22, 3333, 4444]), 4);
}
