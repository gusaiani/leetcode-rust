use leetcode::check_if_it_is_a_straight_line::check_straight_line;

#[test]
fn diagonal_line() {
    let points = vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![6, 7],
    ];
    assert!(check_straight_line(points));
}

#[test]
fn not_a_line() {
    let points = vec![
        vec![1, 1],
        vec![2, 2],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![7, 7],
    ];
    assert!(!check_straight_line(points));
}

#[test]
fn two_points_always_a_line() {
    assert!(check_straight_line(vec![vec![0, 0], vec![1, 1]]));
}

#[test]
fn horizontal_line() {
    assert!(check_straight_line(vec![
        vec![-3, 5],
        vec![0, 5],
        vec![4, 5]
    ]));
}

#[test]
fn vertical_line() {
    assert!(check_straight_line(vec![
        vec![7, -2],
        vec![7, 0],
        vec![7, 100]
    ]));
}

#[test]
fn negative_coordinates_collinear() {
    assert!(check_straight_line(vec![
        vec![-4, -3],
        vec![-2, -1],
        vec![0, 1],
        vec![2, 3]
    ]));
}

#[test]
fn last_point_off_line() {
    assert!(!check_straight_line(vec![
        vec![0, 0],
        vec![1, 1],
        vec![2, 3]
    ]));
}
