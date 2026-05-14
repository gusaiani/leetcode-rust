use leetcode::first_unique_character_in_a_string::first_uniq_char;

#[test]
fn example_1_leetcode() {
    assert_eq!(first_uniq_char("leetcode".to_string()), 0);
}

#[test]
fn example_2_loveleetcode() {
    assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
}

#[test]
fn example_3_no_unique() {
    assert_eq!(first_uniq_char("aabb".to_string()), -1);
}

#[test]
fn single_character() {
    assert_eq!(first_uniq_char("z".to_string()), 0);
}

#[test]
fn all_same_character() {
    assert_eq!(first_uniq_char("aaaa".to_string()), -1);
}

#[test]
fn unique_is_last() {
    assert_eq!(first_uniq_char("aabbc".to_string()), 4);
}

#[test]
fn unique_in_middle() {
    assert_eq!(first_uniq_char("aabcba".to_string()), 3);
}

#[test]
fn two_chars_first_unique() {
    assert_eq!(first_uniq_char("ab".to_string()), 0);
}

#[test]
fn long_with_late_unique() {
    assert_eq!(first_uniq_char("dddccdbba".to_string()), 8);
}
