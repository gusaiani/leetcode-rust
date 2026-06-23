use leetcode::reverse_words_in_a_string_iii::reverse_words;

#[test]
fn example_1() {
    assert_eq!(
        reverse_words("Let's take LeetCode contest".to_string()),
        "s'teL ekat edoCteeL tsetnoc".to_string()
    );
}

#[test]
fn example_2() {
    assert_eq!(reverse_words("Mr Ding".to_string()), "rM gniD".to_string());
}

#[test]
fn single_word() {
    assert_eq!(reverse_words("hello".to_string()), "olleh".to_string());
}

#[test]
fn single_char_word() {
    assert_eq!(reverse_words("a".to_string()), "a".to_string());
}

#[test]
fn two_single_chars() {
    assert_eq!(reverse_words("a b".to_string()), "a b".to_string());
}

#[test]
fn palindrome_word_unchanged() {
    assert_eq!(
        reverse_words("racecar level".to_string()),
        "racecar level".to_string()
    );
}

#[test]
fn with_digits() {
    assert_eq!(
        reverse_words("ab12 cd34".to_string()),
        "21ba 43dc".to_string()
    );
}
