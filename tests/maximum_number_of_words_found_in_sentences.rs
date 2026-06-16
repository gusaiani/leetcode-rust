use leetcode::maximum_number_of_words_found_in_sentences::most_words_found;

fn owned(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

#[test]
fn example_1() {
    assert_eq!(
        most_words_found(owned(&[
            "alice and bob love leetcode",
            "i think so too",
            "this is great thanks very much",
        ])),
        6
    );
}

#[test]
fn example_2() {
    assert_eq!(
        most_words_found(owned(&[
            "please wait",
            "continue to fight",
            "continue to win",
        ])),
        3
    );
}

#[test]
fn single_sentence() {
    assert_eq!(most_words_found(owned(&["hello world foo"])), 3);
}

#[test]
fn single_word_sentences() {
    assert_eq!(most_words_found(owned(&["hi", "yo", "hey"])), 1);
}

#[test]
fn first_is_longest() {
    assert_eq!(most_words_found(owned(&["a b c d e", "x", "y z"])), 5);
}
