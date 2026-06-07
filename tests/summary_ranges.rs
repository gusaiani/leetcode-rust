use leetcode::summary_ranges::summary_ranges;

#[test]
fn example_1() {
    assert_eq!(
        summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec!["0->2", "4->5", "7"]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
        vec!["0", "2->4", "6", "8->9"]
    );
}

#[test]
fn empty() {
    let expected: Vec<String> = vec![];
    assert_eq!(summary_ranges(vec![]), expected);
}

#[test]
fn single_element() {
    assert_eq!(summary_ranges(vec![5]), vec!["5"]);
}

#[test]
fn all_consecutive() {
    assert_eq!(summary_ranges(vec![1, 2, 3, 4]), vec!["1->4"]);
}

#[test]
fn no_consecutive() {
    assert_eq!(summary_ranges(vec![1, 3, 5, 7]), vec!["1", "3", "5", "7"]);
}

#[test]
fn negatives_and_zero() {
    assert_eq!(
        summary_ranges(vec![-3, -2, -1, 1, 2]),
        vec!["-3->-1", "1->2"]
    );
}

#[test]
fn handles_i32_bounds() {
    assert_eq!(
        summary_ranges(vec![i32::MIN, i32::MIN + 1, i32::MAX]),
        vec![
            format!("{}->{}", i32::MIN, i32::MIN + 1),
            i32::MAX.to_string()
        ]
    );
}
