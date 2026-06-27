use leetcode::minimum_sum_of_four_digit_number_after_splitting_digits::minimum_sum;

#[test]
fn example_1() {
    // num = 2932 -> [29, 23] -> 52
    assert_eq!(minimum_sum(2932), 52);
}

#[test]
fn example_2() {
    // num = 4009 -> digits 0,0,4,9 -> [04, 09] = [4, 9] -> 13
    assert_eq!(minimum_sum(4009), 13);
}

#[test]
fn all_same_digits() {
    // 1111 -> [11, 11] -> 22
    assert_eq!(minimum_sum(1111), 22);
}

#[test]
fn smallest_four_digit() {
    // 1000 -> digits 0,0,0,1 -> [00, 01] -> 1
    assert_eq!(minimum_sum(1000), 1);
}

#[test]
fn largest_four_digit() {
    // 9999 -> [99, 99] -> 198
    assert_eq!(minimum_sum(9999), 198);
}

#[test]
fn mixed_digits() {
    // 2304 -> digits 0,2,3,4 -> [03, 24] -> 27
    assert_eq!(minimum_sum(2304), 27);
}
