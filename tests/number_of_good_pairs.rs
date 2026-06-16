use leetcode::number_of_good_pairs::num_identical_pairs;

#[test]
fn example_1() {
    // (0,3), (0,4), (3,4), (2,5) => 4
    assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
}

#[test]
fn example_2() {
    // all identical: C(4,2) = 6
    assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
}

#[test]
fn example_3() {
    // all distinct => 0
    assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0);
}

#[test]
fn single_element() {
    assert_eq!(num_identical_pairs(vec![7]), 0);
}

#[test]
fn one_pair() {
    assert_eq!(num_identical_pairs(vec![5, 5]), 1);
}

#[test]
fn mixed_frequencies() {
    // 2 appears 3x -> C(3,2)=3; 4 appears 2x -> C(2,2)=1; total 4
    assert_eq!(num_identical_pairs(vec![2, 4, 2, 4, 2]), 4);
}
