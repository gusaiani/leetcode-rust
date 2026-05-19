use leetcode::fizz_buzz::fizz_buzz;

fn s(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

#[test]
fn example_1() {
    assert_eq!(fizz_buzz(3), s(&["1", "2", "Fizz"]));
}

#[test]
fn example_2() {
    assert_eq!(fizz_buzz(5), s(&["1", "2", "Fizz", "4", "Buzz"]));
}

#[test]
fn example_3() {
    assert_eq!(
        fizz_buzz(15),
        s(&[
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ])
    );
}

#[test]
fn single_element() {
    assert_eq!(fizz_buzz(1), s(&["1"]));
}

#[test]
fn no_fizzbuzz_before_fifteen() {
    let out = fizz_buzz(14);
    assert_eq!(out.len(), 14);
    assert!(!out.contains(&"FizzBuzz".to_string()));
}

#[test]
fn divisible_by_five_not_three() {
    assert_eq!(fizz_buzz(10).last().unwrap(), "Buzz");
}

#[test]
fn divisible_by_three_not_five() {
    assert_eq!(fizz_buzz(9).last().unwrap(), "Fizz");
}
