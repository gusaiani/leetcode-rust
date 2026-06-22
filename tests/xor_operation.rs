use leetcode::xor_operation::xor_operation;

#[test]
fn example_1() {
    // nums = [5, 7, 9, 11], XOR = 8
    assert_eq!(xor_operation(5, 0), 8);
}

#[test]
fn example_2() {
    // nums = [1, 3, 5, 7, 9], XOR = 8
    assert_eq!(xor_operation(4, 3), 8);
}

#[test]
fn single_element() {
    // nums = [3], XOR = 3
    assert_eq!(xor_operation(1, 3), 3);
}

#[test]
fn single_element_zero() {
    // nums = [0], XOR = 0
    assert_eq!(xor_operation(1, 0), 0);
}

#[test]
fn two_elements() {
    // nums = [7, 9], XOR = 14
    assert_eq!(xor_operation(2, 7), 14);
}

#[test]
fn larger_range() {
    // nums = [10, 12, 14, 16, 18, 20], XOR = 30
    assert_eq!(xor_operation(6, 10), 30);
}
