use leetcode::squares_of_a_sorted_array::sorted_squares;

#[test]
fn example_1() {
    assert_eq!(
        sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}

#[test]
fn single_element() {
    assert_eq!(sorted_squares(vec![5]), vec![25]);
}

#[test]
fn single_negative() {
    assert_eq!(sorted_squares(vec![-5]), vec![25]);
}

#[test]
fn all_negative() {
    assert_eq!(sorted_squares(vec![-5, -3, -1]), vec![1, 9, 25]);
}

#[test]
fn all_non_negative() {
    assert_eq!(sorted_squares(vec![0, 2, 4]), vec![0, 4, 16]);
}

#[test]
fn contains_zero() {
    assert_eq!(sorted_squares(vec![-2, 0, 1]), vec![0, 1, 4]);
}

#[test]
fn duplicates() {
    assert_eq!(sorted_squares(vec![-2, -2, 2, 2]), vec![4, 4, 4, 4]);
}
