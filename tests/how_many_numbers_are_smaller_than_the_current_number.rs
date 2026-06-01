use leetcode::how_many_numbers_are_smaller_than_the_current_number::smaller_numbers_than_current;

#[test]
fn example_1() {
    assert_eq!(
        smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        smaller_numbers_than_current(vec![6, 5, 4, 8]),
        vec![2, 1, 0, 3]
    );
}

#[test]
fn example_3() {
    assert_eq!(
        smaller_numbers_than_current(vec![7, 7, 7, 7]),
        vec![0, 0, 0, 0]
    );
}

#[test]
fn single_element() {
    assert_eq!(smaller_numbers_than_current(vec![5]), vec![0]);
}

#[test]
fn already_sorted() {
    assert_eq!(
        smaller_numbers_than_current(vec![1, 2, 3, 4]),
        vec![0, 1, 2, 3]
    );
}

#[test]
fn reverse_sorted() {
    assert_eq!(
        smaller_numbers_than_current(vec![4, 3, 2, 1]),
        vec![3, 2, 1, 0]
    );
}

#[test]
fn includes_zero() {
    assert_eq!(
        smaller_numbers_than_current(vec![0, 1, 0, 2]),
        vec![0, 2, 0, 3]
    );
}

#[test]
fn boundary_values() {
    // Constraint range is 0..=100.
    assert_eq!(
        smaller_numbers_than_current(vec![100, 0, 100, 0]),
        vec![2, 0, 2, 0]
    );
}
