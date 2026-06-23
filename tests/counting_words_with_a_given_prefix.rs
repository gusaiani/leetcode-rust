use leetcode::counting_words_with_a_given_prefix::prefix_count;

fn words(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn example_1() {
    // "pay", "attention", "practice", "attend" — prefix "at" -> "attention", "attend"
    assert_eq!(
        prefix_count(
            words(&["pay", "attention", "practice", "attend"]),
            "at".to_string()
        ),
        2
    );
}

#[test]
fn example_2() {
    // "leetcode", "win", "loops", "success" — prefix "code" -> none
    assert_eq!(
        prefix_count(
            words(&["leetcode", "win", "loops", "success"]),
            "code".to_string()
        ),
        0
    );
}

#[test]
fn all_match() {
    assert_eq!(
        prefix_count(words(&["go", "good", "gone"]), "go".to_string()),
        3
    );
}

#[test]
fn single_word_match() {
    assert_eq!(prefix_count(words(&["hello"]), "hell".to_string()), 1);
}

#[test]
fn single_word_no_match() {
    assert_eq!(prefix_count(words(&["hello"]), "world".to_string()), 0);
}

#[test]
fn prefix_longer_than_word() {
    // "ab" cannot have "abc" as a prefix
    assert_eq!(
        prefix_count(words(&["ab", "abc", "abcd"]), "abc".to_string()),
        2
    );
}

#[test]
fn whole_word_equals_prefix() {
    assert_eq!(
        prefix_count(words(&["xyz", "xy", "x"]), "xyz".to_string()),
        1
    );
}

#[test]
fn single_char_prefix() {
    assert_eq!(
        prefix_count(words(&["apple", "banana", "avocado"]), "a".to_string()),
        2
    );
}
