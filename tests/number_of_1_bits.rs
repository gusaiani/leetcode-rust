use leetcode::number_of_1_bits::hamming_weight;

#[test]
fn example_1_eleven() {
    // 11 == 0b1011 -> 3 set bits
    assert_eq!(hamming_weight(11), 3);
}

#[test]
fn example_2_one_hundred_twenty_eight() {
    // 128 == 0b1000_0000 -> 1 set bit
    assert_eq!(hamming_weight(128), 1);
}

#[test]
fn example_3_max_i32() {
    // 2_147_483_647 == 0x7FFF_FFFF -> 31 set bits
    assert_eq!(hamming_weight(2_147_483_647), 31);
}

#[test]
fn zero_has_no_set_bits() {
    assert_eq!(hamming_weight(0), 0);
}

#[test]
fn one_has_one_set_bit() {
    assert_eq!(hamming_weight(1), 1);
}

#[test]
fn power_of_two_has_one_set_bit() {
    assert_eq!(hamming_weight(1024), 1);
}

#[test]
fn alternating_bits() {
    // 0b0101_0101_0101_0101_0101_0101_0101_0101 == 1_431_655_765 -> 16 set bits
    assert_eq!(hamming_weight(1_431_655_765), 16);
}
