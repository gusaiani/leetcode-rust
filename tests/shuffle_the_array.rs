use leetcode::shuffle_the_array::shuffle;

#[test]
fn example_1() {
    assert_eq!(shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
}

#[test]
fn example_2() {
    assert_eq!(
        shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
        vec![1, 4, 2, 3, 3, 2, 4, 1]
    );
}

#[test]
fn example_3() {
    assert_eq!(shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
}

#[test]
fn single_pair() {
    assert_eq!(shuffle(vec![1, 2], 1), vec![1, 2]);
}

#[test]
fn negatives_and_zero() {
    assert_eq!(shuffle(vec![-1, 0, 5, -3], 2), vec![-1, 5, 0, -3]);
}

#[test]
fn equal_halves() {
    assert_eq!(shuffle(vec![7, 7, 7, 7], 2), vec![7, 7, 7, 7]);
}
