//! LeetCode 1295. Find Numbers with Even Number of Digits
//! https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
//!
//! Given an array `nums` of integers, return how many of them contain an even
//! number of digits.

pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut numbers_with_even_number_of_digits = 0;

    for num in nums {
        if find_number_of_digits(num) % 2 == 0 {
            numbers_with_even_number_of_digits += 1;
        }
    }

    numbers_with_even_number_of_digits
}

fn find_number_of_digits(num: i32) -> i32 {
    let mut number = num;
    let mut digits = 1;

    while number >= 10 {
        digits += 1;
        number /= 10;
    }

    digits
}
