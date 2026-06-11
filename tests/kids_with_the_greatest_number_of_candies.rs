use leetcode::kids_with_the_greatest_number_of_candies::kids_with_candies;

#[test]
fn example_1() {
    assert_eq!(
        kids_with_candies(vec![2, 3, 5, 1, 3], 3),
        vec![true, true, true, false, true]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        kids_with_candies(vec![4, 2, 1, 1, 2], 1),
        vec![true, false, false, false, false]
    );
}

#[test]
fn example_3() {
    assert_eq!(
        kids_with_candies(vec![12, 1, 12], 10),
        vec![true, false, true]
    );
}

#[test]
fn single_kid() {
    assert_eq!(kids_with_candies(vec![5], 0), vec![true]);
}

#[test]
fn all_equal() {
    assert_eq!(kids_with_candies(vec![2, 2, 2], 0), vec![true, true, true]);
}

#[test]
fn extra_zero_keeps_only_max() {
    assert_eq!(
        kids_with_candies(vec![1, 4, 4, 3], 0),
        vec![false, true, true, false]
    );
}

#[test]
fn large_extra_lifts_everyone() {
    assert_eq!(
        kids_with_candies(vec![3, 5, 1], 100),
        vec![true, true, true]
    );
}
