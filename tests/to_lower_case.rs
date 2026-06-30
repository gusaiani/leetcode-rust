use leetcode::to_lower_case::to_lower_case;

#[test]
fn example_1() {
    assert_eq!(to_lower_case("Hello".to_string()), "hello");
}

#[test]
fn example_2() {
    assert_eq!(to_lower_case("here".to_string()), "here");
}

#[test]
fn example_3() {
    assert_eq!(to_lower_case("LOVELY".to_string()), "lovely");
}

#[test]
fn empty_string() {
    assert_eq!(to_lower_case(String::new()), "");
}

#[test]
fn already_lowercase() {
    assert_eq!(to_lower_case("abcxyz".to_string()), "abcxyz");
}

#[test]
fn digits_and_symbols_unchanged() {
    assert_eq!(to_lower_case("Al5g6Ri7Tm!".to_string()), "al5g6ri7tm!");
}

#[test]
fn boundary_letters() {
    assert_eq!(to_lower_case("AZaz".to_string()), "azaz");
}
