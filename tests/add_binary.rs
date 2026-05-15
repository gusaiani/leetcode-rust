use leetcode::add_binary::add_binary;

#[test]
fn example_1() {
    assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
}

#[test]
fn example_2() {
    assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");
}

#[test]
fn both_zero() {
    assert_eq!(add_binary("0".to_string(), "0".to_string()), "0");
}

#[test]
fn zero_plus_nonzero() {
    assert_eq!(add_binary("0".to_string(), "1".to_string()), "1");
    assert_eq!(add_binary("1101".to_string(), "0".to_string()), "1101");
}

#[test]
fn unequal_lengths() {
    assert_eq!(add_binary("1".to_string(), "111".to_string()), "1000");
    assert_eq!(
        add_binary("100".to_string(), "110010".to_string()),
        "110110"
    );
}

#[test]
fn long_carry_chain() {
    assert_eq!(add_binary("111".to_string(), "1".to_string()), "1000");
    assert_eq!(
        add_binary("1111111".to_string(), "1".to_string()),
        "10000000"
    );
}

#[test]
fn equal_length_no_carry() {
    assert_eq!(add_binary("1010".to_string(), "0101".to_string()), "1111");
}

#[test]
fn large_inputs() {
    let a = "1".repeat(100);
    let b = "1".to_string();
    let mut expected = String::from("1");
    expected.push_str(&"0".repeat(100));
    assert_eq!(add_binary(a, b), expected);
}

#[test]
fn carry_propagates_beyond_longer_operand() {
    assert_eq!(add_binary("11".to_string(), "11".to_string()), "110");
    assert_eq!(
        add_binary("10011".to_string(), "11101".to_string()),
        "110000"
    );
}
