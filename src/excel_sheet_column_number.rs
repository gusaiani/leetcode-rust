//! LeetCode 171. Excel Sheet Column Number
//! https://leetcode.com/problems/excel-sheet-column-number/
//!
//! Given a string `column_title` that represents the column title as appears in
//! an Excel sheet, return its corresponding column number.
//!
//! For example:
//!   A  -> 1
//!   B  -> 2
//!   Z  -> 26
//!   AA -> 27
//!   AB -> 28

pub fn title_to_number(column_title: String) -> i32 {
    let mut result = 0;

    for byte in column_title.bytes() {
        let value = (byte - b'A' + 1) as i32;
        result = result * 26 + value;
    }

    result
}
