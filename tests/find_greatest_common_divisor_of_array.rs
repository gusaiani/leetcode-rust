use leetcode::find_greatest_common_divisor_of_array::find_gcd;

#[test]
fn example_1() {
    // min = 2, max = 6, gcd(2, 6) = 2
    assert_eq!(find_gcd(vec![2, 5, 6, 9, 10]), 2);
}

#[test]
fn example_2() {
    // min = 1, max = 5, gcd(1, 5) = 1
    assert_eq!(find_gcd(vec![7, 5, 6, 8, 3]), 1);
}

#[test]
fn example_3() {
    // min = 3, max = 3, gcd(3, 3) = 3
    assert_eq!(find_gcd(vec![3, 3]), 3);
}

#[test]
fn single_element() {
    // min == max == the only element
    assert_eq!(find_gcd(vec![42]), 42);
}

#[test]
fn one_divides_other() {
    // min = 4, max = 12, gcd = 4
    assert_eq!(find_gcd(vec![12, 4, 8]), 4);
}

#[test]
fn coprime_extremes() {
    // min = 9, max = 14, gcd(9, 14) = 1
    assert_eq!(find_gcd(vec![14, 9, 11]), 1);
}

#[test]
fn contains_one() {
    // min = 1 forces gcd to 1
    assert_eq!(find_gcd(vec![1, 1000]), 1);
}

#[test]
fn equal_large_values() {
    // min = max = 1000, gcd = 1000
    assert_eq!(find_gcd(vec![1000, 1000, 1000]), 1000);
}
