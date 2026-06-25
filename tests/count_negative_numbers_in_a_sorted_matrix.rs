use leetcode::count_negative_numbers_in_a_sorted_matrix::count_negatives;

#[test]
fn example_1() {
    let grid = vec![
        vec![4, 3, 2, -1],
        vec![3, 2, 1, -1],
        vec![1, 1, -1, -2],
        vec![-1, -1, -2, -3],
    ];
    assert_eq!(count_negatives(grid), 8);
}

#[test]
fn example_2() {
    let grid = vec![vec![3, 2], vec![1, 0]];
    assert_eq!(count_negatives(grid), 0);
}

#[test]
fn all_negative() {
    let grid = vec![vec![-1, -2], vec![-3, -4]];
    assert_eq!(count_negatives(grid), 4);
}

#[test]
fn no_negatives_single_row() {
    let grid = vec![vec![5, 4, 3, 2, 1]];
    assert_eq!(count_negatives(grid), 0);
}

#[test]
fn single_cell_negative() {
    let grid = vec![vec![-5]];
    assert_eq!(count_negatives(grid), 1);
}

#[test]
fn single_cell_non_negative() {
    let grid = vec![vec![0]];
    assert_eq!(count_negatives(grid), 0);
}

#[test]
fn single_column() {
    let grid = vec![vec![3], vec![2], vec![-1], vec![-4]];
    assert_eq!(count_negatives(grid), 2);
}

#[test]
fn zero_boundary() {
    // Zero is non-negative; only strictly-negative entries count.
    let grid = vec![vec![2, 1, 0], vec![1, 0, -1], vec![0, -1, -2]];
    assert_eq!(count_negatives(grid), 3);
}
