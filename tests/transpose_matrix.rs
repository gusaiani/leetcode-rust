use leetcode::transpose_matrix::transpose;

#[test]
fn square_matrix() {
    assert_eq!(
        transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    );
}

#[test]
fn wide_matrix() {
    assert_eq!(
        transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
}

#[test]
fn tall_matrix() {
    assert_eq!(
        transpose(vec![vec![1, 2], vec![3, 4], vec![5, 6]]),
        vec![vec![1, 3, 5], vec![2, 4, 6]]
    );
}

#[test]
fn single_row() {
    assert_eq!(
        transpose(vec![vec![1, 2, 3]]),
        vec![vec![1], vec![2], vec![3]]
    );
}

#[test]
fn single_column() {
    assert_eq!(
        transpose(vec![vec![1], vec![2], vec![3]]),
        vec![vec![1, 2, 3]]
    );
}

#[test]
fn single_element() {
    assert_eq!(transpose(vec![vec![42]]), vec![vec![42]]);
}

#[test]
fn negatives() {
    assert_eq!(
        transpose(vec![vec![-1, 0], vec![5, -9]]),
        vec![vec![-1, 5], vec![0, -9]]
    );
}
