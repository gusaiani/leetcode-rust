use leetcode::unique_morse_code_words::unique_morse_representations;

fn words(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn example_1() {
    // "gin" -> "--...-.", "zen" -> "--...-.", "gig" -> "--...--.", "msg" -> "--...--."
    assert_eq!(
        unique_morse_representations(words(&["gin", "zen", "gig", "msg"])),
        2
    );
}

#[test]
fn example_2() {
    assert_eq!(unique_morse_representations(words(&["a"])), 1);
}

#[test]
fn all_distinct_letters() {
    assert_eq!(unique_morse_representations(words(&["a", "b", "c"])), 3);
}

#[test]
fn all_identical_words() {
    assert_eq!(
        unique_morse_representations(words(&["abc", "abc", "abc"])),
        1
    );
}

#[test]
fn single_long_word() {
    assert_eq!(unique_morse_representations(words(&["zzzz"])), 1);
}

#[test]
fn empty_input() {
    assert_eq!(unique_morse_representations(Vec::new()), 0);
}
