use leetcode::truncate_sentence::truncate_sentence;

#[test]
fn example_1() {
    assert_eq!(
        truncate_sentence("Hello how are you Contestant".to_string(), 4),
        "Hello how are you"
    );
}

#[test]
fn example_2() {
    assert_eq!(
        truncate_sentence("What is the solution to this problem".to_string(), 4),
        "What is the solution"
    );
}

#[test]
fn k_equals_word_count() {
    assert_eq!(
        truncate_sentence("chopper is not a tanuki".to_string(), 5),
        "chopper is not a tanuki"
    );
}

#[test]
fn single_word() {
    assert_eq!(truncate_sentence("Hello".to_string(), 1), "Hello");
}

#[test]
fn first_word_only() {
    assert_eq!(
        truncate_sentence("The quick brown fox".to_string(), 1),
        "The"
    );
}
