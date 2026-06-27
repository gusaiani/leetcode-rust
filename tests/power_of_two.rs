use leetcode::power_of_two::is_power_of_two;

#[test]
fn example_1() {
    // 2^0 = 1
    assert!(is_power_of_two(1));
}

#[test]
fn example_2() {
    // 2^4 = 16
    assert!(is_power_of_two(16));
}

#[test]
fn example_3() {
    assert!(!is_power_of_two(3));
}

#[test]
fn two_is_a_power_of_two() {
    assert!(is_power_of_two(2));
}

#[test]
fn zero_is_not_a_power_of_two() {
    assert!(!is_power_of_two(0));
}

#[test]
fn negatives_are_not_powers_of_two() {
    assert!(!is_power_of_two(-1));
    assert!(!is_power_of_two(-16));
}

#[test]
fn largest_power_of_two_in_i32() {
    // 2^30 = 1_073_741_824 fits in i32; 2^31 does not.
    assert!(is_power_of_two(1_073_741_824));
}

#[test]
fn non_powers_near_powers() {
    assert!(!is_power_of_two(15));
    assert!(!is_power_of_two(17));
    assert!(!is_power_of_two(6));
}

#[test]
fn i32_max_is_not_a_power_of_two() {
    assert!(!is_power_of_two(i32::MAX));
}
