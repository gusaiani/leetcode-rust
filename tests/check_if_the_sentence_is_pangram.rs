use leetcode::check_if_the_sentence_is_pangram::check_if_pangram;

#[test]
fn example_1() {
    assert!(check_if_pangram(
        "thequickbrownfoxjumpsoverthelazydog".to_string()
    ));
}

#[test]
fn example_2() {
    assert!(!check_if_pangram("leetcode".to_string()));
}

#[test]
fn missing_single_letter() {
    // full alphabet minus 'z'
    assert!(!check_if_pangram("abcdefghijklmnopqrstuvwxy".to_string()));
}

#[test]
fn exactly_the_alphabet() {
    assert!(check_if_pangram("abcdefghijklmnopqrstuvwxyz".to_string()));
}

#[test]
fn single_char() {
    assert!(!check_if_pangram("a".to_string()));
}

#[test]
fn repeated_letters_not_pangram() {
    assert!(!check_if_pangram("aaaaaaaaaa".to_string()));
}
