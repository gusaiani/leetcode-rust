use leetcode::create_target_array_in_the_given_order::create_target_array;

#[test]
fn example_1() {
    assert_eq!(
        create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
        vec![0, 4, 1, 3, 2]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
        vec![0, 1, 2, 3, 4]
    );
}

#[test]
fn example_3() {
    assert_eq!(create_target_array(vec![1], vec![0]), vec![1]);
}

#[test]
fn single_element() {
    assert_eq!(create_target_array(vec![42], vec![0]), vec![42]);
}

#[test]
fn all_inserts_at_front() {
    assert_eq!(
        create_target_array(vec![1, 2, 3], vec![0, 0, 0]),
        vec![3, 2, 1]
    );
}

#[test]
fn always_appends_at_end() {
    assert_eq!(
        create_target_array(vec![5, 6, 7], vec![0, 1, 2]),
        vec![5, 6, 7]
    );
}

#[test]
fn repeated_middle_insertion() {
    assert_eq!(
        create_target_array(vec![4, 2, 4, 3, 2], vec![0, 0, 1, 3, 1]),
        vec![2, 2, 4, 4, 3]
    );
}
