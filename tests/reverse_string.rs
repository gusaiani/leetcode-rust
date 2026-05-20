use leetcode::reverse_string::reverse_string;

#[test]
fn example_1() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
}

#[test]
fn example_2() {
    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}

#[test]
fn single_character() {
    let mut s = vec!['a'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['a']);
}

#[test]
fn two_characters() {
    let mut s = vec!['a', 'b'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['b', 'a']);
}

#[test]
fn empty() {
    let mut s: Vec<char> = vec![];
    reverse_string(&mut s);
    assert_eq!(s, Vec::<char>::new());
}

#[test]
fn palindrome_unchanged() {
    let mut s = vec!['r', 'a', 'c', 'e', 'c', 'a', 'r'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['r', 'a', 'c', 'e', 'c', 'a', 'r']);
}

#[test]
fn non_ascii() {
    let mut s = vec!['á', 'é', 'í', 'ó', 'ú'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['ú', 'ó', 'í', 'é', 'á']);
}
