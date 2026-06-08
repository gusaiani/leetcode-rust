use leetcode::pascals_triangle::generate;

#[test]
fn example_1() {
    assert_eq!(
        generate(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ]
    );
}

#[test]
fn example_2() {
    assert_eq!(generate(1), vec![vec![1]]);
}

#[test]
fn two_rows() {
    assert_eq!(generate(2), vec![vec![1], vec![1, 1]]);
}

#[test]
fn three_rows() {
    assert_eq!(generate(3), vec![vec![1], vec![1, 1], vec![1, 2, 1]]);
}

#[test]
fn row_sums_are_powers_of_two() {
    // Row i sums to 2^i — a quick structural sanity check.
    let triangle = generate(10);
    for (i, row) in triangle.iter().enumerate() {
        let sum: i32 = row.iter().sum();
        assert_eq!(sum, 1 << i, "row {i} should sum to 2^{i}");
    }
}

#[test]
fn edges_are_one() {
    let triangle = generate(8);
    for row in &triangle {
        assert_eq!(*row.first().unwrap(), 1);
        assert_eq!(*row.last().unwrap(), 1);
    }
}

#[test]
fn row_length_equals_index_plus_one() {
    let triangle = generate(6);
    for (i, row) in triangle.iter().enumerate() {
        assert_eq!(row.len(), i + 1);
    }
}
