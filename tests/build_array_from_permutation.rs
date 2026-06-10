use leetcode::build_array_from_permutation::build_array;

#[test]
fn example_1() {
    assert_eq!(build_array(vec![0, 2, 1, 5, 3, 4]), vec![0, 1, 2, 4, 5, 3]);
}

#[test]
fn example_2() {
    assert_eq!(build_array(vec![5, 0, 1, 2, 3, 4]), vec![4, 5, 0, 1, 2, 3]);
}

#[test]
fn single_element() {
    assert_eq!(build_array(vec![0]), vec![0]);
}

#[test]
fn identity_permutation() {
    assert_eq!(build_array(vec![0, 1, 2, 3]), vec![0, 1, 2, 3]);
}

#[test]
fn reversed_permutation() {
    assert_eq!(build_array(vec![3, 2, 1, 0]), vec![0, 1, 2, 3]);
}

#[test]
fn two_elements() {
    assert_eq!(build_array(vec![1, 0]), vec![0, 1]);
}
