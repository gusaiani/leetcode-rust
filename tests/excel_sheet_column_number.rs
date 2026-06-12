use leetcode::excel_sheet_column_number::title_to_number;

#[test]
fn single_letter_a() {
    assert_eq!(title_to_number("A".to_string()), 1);
}

#[test]
fn single_letter_z() {
    assert_eq!(title_to_number("Z".to_string()), 26);
}

#[test]
fn two_letters_aa() {
    assert_eq!(title_to_number("AA".to_string()), 27);
}

#[test]
fn two_letters_ab() {
    assert_eq!(title_to_number("AB".to_string()), 28);
}

#[test]
fn two_letters_zy() {
    assert_eq!(title_to_number("ZY".to_string()), 701);
}

#[test]
fn three_letters_aaa() {
    assert_eq!(title_to_number("AAA".to_string()), 703);
}

#[test]
fn max_column_fxshrxw() {
    assert_eq!(title_to_number("FXSHRXW".to_string()), 2_147_483_647);
}
