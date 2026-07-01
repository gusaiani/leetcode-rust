use leetcode::check_if_two_string_arrays_are_equivalent::array_strings_are_equal;

fn v(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| s.to_string()).collect()
}

#[test]
fn example_1() {
    // "ab" + "c" == "a" + "bc" == "abc"
    assert!(array_strings_are_equal(v(&["ab", "c"]), v(&["a", "bc"])));
}

#[test]
fn example_2() {
    // "a" + "cb" == "acb" != "ab" + "c" == "abc"
    assert!(!array_strings_are_equal(v(&["a", "cb"]), v(&["ab", "c"])));
}

#[test]
fn example_3() {
    // "abc" == "a" + "b" + "c"
    assert!(array_strings_are_equal(v(&["abc"]), v(&["a", "b", "c"])));
}

#[test]
fn different_total_length() {
    // "abcd" != "abc"
    assert!(!array_strings_are_equal(v(&["abcd"]), v(&["abc"])));
}

#[test]
fn same_length_different_content() {
    assert!(!array_strings_are_equal(v(&["a", "b"]), v(&["a", "a"])));
}

#[test]
fn single_char_each() {
    assert!(array_strings_are_equal(v(&["x"]), v(&["x"])));
}

#[test]
fn many_pieces_vs_one() {
    assert!(array_strings_are_equal(
        v(&["h", "e", "l", "l", "o"]),
        v(&["hello"])
    ));
}
