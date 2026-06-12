use leetcode::decompress_run_length_encoded_list::decompress_rl_elist;

#[test]
fn example_1() {
    assert_eq!(decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
}

#[test]
fn example_2() {
    assert_eq!(decompress_rl_elist(vec![1, 1, 2, 3]), vec![1, 3, 3]);
}

#[test]
fn single_pair() {
    assert_eq!(decompress_rl_elist(vec![3, 7]), vec![7, 7, 7]);
}

#[test]
fn zero_frequency_pair_contributes_nothing() {
    assert_eq!(decompress_rl_elist(vec![0, 9, 2, 5]), vec![5, 5]);
}

#[test]
fn all_pairs_zero_frequency() {
    assert_eq!(decompress_rl_elist(vec![0, 1, 0, 2]), Vec::<i32>::new());
}

#[test]
fn repeated_values_across_pairs() {
    assert_eq!(decompress_rl_elist(vec![2, 5, 3, 5]), vec![5, 5, 5, 5, 5]);
}
