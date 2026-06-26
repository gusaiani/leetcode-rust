use leetcode::final_value_of_variable_after_performing_operations::final_value_after_operations;

fn ops(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

#[test]
fn example_1() {
    assert_eq!(final_value_after_operations(ops(&["--X", "X++", "X++"])), 1);
}

#[test]
fn example_2() {
    assert_eq!(final_value_after_operations(ops(&["++X", "++X", "X++"])), 3);
}

#[test]
fn example_3() {
    assert_eq!(
        final_value_after_operations(ops(&["X++", "++X", "--X", "X--"])),
        0
    );
}

#[test]
fn single_increment_prefix() {
    assert_eq!(final_value_after_operations(ops(&["++X"])), 1);
}

#[test]
fn single_decrement_postfix() {
    assert_eq!(final_value_after_operations(ops(&["X--"])), -1);
}

#[test]
fn all_decrements_go_negative() {
    assert_eq!(
        final_value_after_operations(ops(&["--X", "X--", "--X"])),
        -3
    );
}

#[test]
fn mixed_cancels_out() {
    assert_eq!(
        final_value_after_operations(ops(&["X++", "X--", "++X", "--X"])),
        0
    );
}
