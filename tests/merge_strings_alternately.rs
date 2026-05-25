use leetcode::merge_strings_alternately::merge_alternately;

#[test]
fn example_1_equal_length() {
    assert_eq!(
        merge_alternately("abc".to_string(), "pqr".to_string()),
        "apbqcr"
    );
}

#[test]
fn example_2_word2_longer() {
    assert_eq!(
        merge_alternately("ab".to_string(), "pqrs".to_string()),
        "apbqrs"
    );
}

#[test]
fn example_3_word1_longer() {
    assert_eq!(
        merge_alternately("abcd".to_string(), "pq".to_string()),
        "apbqcd"
    );
}

#[test]
fn single_character_each() {
    assert_eq!(merge_alternately("a".to_string(), "b".to_string()), "ab");
}

#[test]
fn word1_single_word2_multi() {
    assert_eq!(
        merge_alternately("x".to_string(), "12345".to_string()),
        "x12345"
    );
}

#[test]
fn word2_single_word1_multi() {
    assert_eq!(
        merge_alternately("hello".to_string(), "!".to_string()),
        "h!ello"
    );
}

#[test]
fn max_length_inputs() {
    let word1 = "a".repeat(100);
    let word2 = "b".repeat(100);
    let expected: String = word1
        .chars()
        .zip(word2.chars())
        .flat_map(|(a, b)| [a, b])
        .collect();
    assert_eq!(merge_alternately(word1, word2), expected);
}
