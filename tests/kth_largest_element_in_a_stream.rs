use leetcode::kth_largest_element_in_a_stream::KthLargest;

#[test]
fn example_1() {
    let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(kth.add(3), 4);
    assert_eq!(kth.add(5), 5);
    assert_eq!(kth.add(10), 5);
    assert_eq!(kth.add(9), 8);
    assert_eq!(kth.add(4), 8);
}

#[test]
fn k_equals_one_tracks_running_max() {
    let mut kth = KthLargest::new(1, vec![]);
    assert_eq!(kth.add(-3), -3);
    assert_eq!(kth.add(-2), -2);
    assert_eq!(kth.add(-4), -2);
    assert_eq!(kth.add(0), 0);
    assert_eq!(kth.add(4), 4);
}

#[test]
fn handles_duplicates() {
    let mut kth = KthLargest::new(2, vec![0]);
    assert_eq!(kth.add(-1), -1);
    assert_eq!(kth.add(1), 0);
    assert_eq!(kth.add(-2), 0);
    assert_eq!(kth.add(-4), 0);
    assert_eq!(kth.add(3), 1);
}

#[test]
fn initial_stream_already_has_k_elements() {
    let mut kth = KthLargest::new(3, vec![5, 1, 9, 7, 3]);
    assert_eq!(kth.add(2), 5);
    assert_eq!(kth.add(8), 7);
}
