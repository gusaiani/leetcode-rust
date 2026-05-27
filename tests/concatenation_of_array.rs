use leetcode::concatenation_of_array::get_concatenation;

#[test]
fn example_1() {
    assert_eq!(get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
}

#[test]
fn example_2() {
    assert_eq!(
        get_concatenation(vec![1, 3, 2, 1]),
        vec![1, 3, 2, 1, 1, 3, 2, 1]
    );
}

#[test]
fn single_element() {
    assert_eq!(get_concatenation(vec![7]), vec![7, 7]);
}

#[test]
fn two_elements() {
    assert_eq!(get_concatenation(vec![5, 9]), vec![5, 9, 5, 9]);
}

#[test]
fn zeros() {
    assert_eq!(get_concatenation(vec![0, 0, 0]), vec![0, 0, 0, 0, 0, 0]);
}

#[test]
fn negative_and_extremes() {
    assert_eq!(
        get_concatenation(vec![-1000, 0, 1000]),
        vec![-1000, 0, 1000, -1000, 0, 1000]
    );
}

#[test]
fn repeated_values() {
    assert_eq!(
        get_concatenation(vec![4, 4, 4, 4]),
        vec![4, 4, 4, 4, 4, 4, 4, 4]
    );
}

#[test]
fn preserves_order() {
    let input: Vec<i32> = (1..=50).collect();
    let mut expected = input.clone();
    expected.extend(input.iter());
    assert_eq!(get_concatenation(input), expected);
}
