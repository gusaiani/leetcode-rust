use leetcode::flipping_an_image::flip_and_invert_image;

#[test]
fn example_1() {
    assert_eq!(
        flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
        vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        flip_and_invert_image(vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0]
        ]),
        vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 1, 0]
        ]
    );
}

#[test]
fn single_cell_zero() {
    assert_eq!(flip_and_invert_image(vec![vec![0]]), vec![vec![1]]);
}

#[test]
fn single_cell_one() {
    assert_eq!(flip_and_invert_image(vec![vec![1]]), vec![vec![0]]);
}

#[test]
fn palindrome_row() {
    // Reversing leaves the row unchanged; only inversion takes effect.
    assert_eq!(
        flip_and_invert_image(vec![vec![1, 0, 1]]),
        vec![vec![0, 1, 0]]
    );
}

#[test]
fn all_ones() {
    assert_eq!(
        flip_and_invert_image(vec![vec![1, 1], vec![1, 1]]),
        vec![vec![0, 0], vec![0, 0]]
    );
}
