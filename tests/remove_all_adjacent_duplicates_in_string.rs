use leetcode::remove_all_adjacent_duplicates_in_string::remove_duplicates;

#[test]
fn example_1() {
    assert_eq!(remove_duplicates("abbaca".to_string()), "ca".to_string());
}

#[test]
fn example_2() {
    assert_eq!(remove_duplicates("azxxzy".to_string()), "ay".to_string());
}

#[test]
fn single_char() {
    assert_eq!(remove_duplicates("a".to_string()), "a".to_string());
}

#[test]
fn collapses_entirely() {
    assert_eq!(remove_duplicates("aa".to_string()), "".to_string());
}

#[test]
fn nested_collapse() {
    // abccba -> abba -> aa -> ""
    assert_eq!(remove_duplicates("abccba".to_string()), "".to_string());
}

#[test]
fn no_adjacent_duplicates() {
    assert_eq!(remove_duplicates("abcde".to_string()), "abcde".to_string());
}

#[test]
fn all_same() {
    // even length fully cancels
    assert_eq!(remove_duplicates("aaaa".to_string()), "".to_string());
}

#[test]
fn all_same_odd() {
    assert_eq!(remove_duplicates("aaaaa".to_string()), "a".to_string());
}

#[test]
fn cascading_from_middle() {
    // m i s s(pop) i(pop) s s(pop) i p p(pop) -> "mi"; checks repeated reductions
    assert_eq!(
        remove_duplicates("mississipp".to_string()),
        "mi".to_string()
    );
}
