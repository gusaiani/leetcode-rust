use leetcode::can_place_flowers::can_place_flowers;

#[test]
fn example_1() {
    assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 1));
}

#[test]
fn example_2() {
    assert!(!can_place_flowers(vec![1, 0, 0, 0, 1], 2));
}

#[test]
fn n_zero_always_true() {
    assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 0));
}

#[test]
fn single_empty_plot() {
    assert!(can_place_flowers(vec![0], 1));
}

#[test]
fn single_planted_plot() {
    assert!(!can_place_flowers(vec![1], 1));
}

#[test]
fn two_empty_plots() {
    assert!(can_place_flowers(vec![0, 0], 1));
    assert!(!can_place_flowers(vec![0, 0], 2));
}

#[test]
fn all_empty_even_spacing() {
    assert!(can_place_flowers(vec![0, 0, 0, 0], 2));
    assert!(!can_place_flowers(vec![0, 0, 0, 0], 3));
}

#[test]
fn boundary_plants() {
    // Only one usable middle slot.
    assert!(can_place_flowers(vec![1, 0, 0, 0, 0, 1], 1));
    assert!(!can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2));
}

#[test]
fn empty_at_edges() {
    assert!(can_place_flowers(vec![0, 0, 1, 0, 1], 1));
    assert!(!can_place_flowers(vec![0, 0, 1, 0, 1], 2));
}
