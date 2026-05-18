use leetcode::jewels_and_stones::num_jewels_in_stones;

#[test]
fn example_1() {
    assert_eq!(
        num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
}

#[test]
fn example_2() {
    assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}

#[test]
fn no_stones() {
    assert_eq!(num_jewels_in_stones("abc".to_string(), "".to_string()), 0);
}

#[test]
fn no_jewels() {
    assert_eq!(num_jewels_in_stones("".to_string(), "abc".to_string()), 0);
}

#[test]
fn all_stones_are_jewels() {
    assert_eq!(
        num_jewels_in_stones("abc".to_string(), "aabbcc".to_string()),
        6
    );
}

#[test]
fn case_sensitive() {
    assert_eq!(
        num_jewels_in_stones("aA".to_string(), "aaAA".to_string()),
        4
    );
}
