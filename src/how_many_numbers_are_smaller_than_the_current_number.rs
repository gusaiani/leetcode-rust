//! LeetCode 1365. How Many Numbers Are Smaller Than the Current Number
//! https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
//!
//! Given the array `nums`, for each `nums[i]` find out how many numbers in the
//! array are smaller than it. That is, for each `nums[i]` you have to count the
//! number of valid `j`'s such that `j != i` and `nums[j] < nums[i]`.
//!
//! Return the answer in an array.

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut counts = [0i32; 101];
    for &n in &nums {
        counts[n as usize] += 1;
    }

    let mut smaller = 0;
    for count in counts.iter_mut() {
        let occurrences = *count;
        *count = smaller;
        smaller += occurrences;
    }

    nums.iter().map(|&n| counts[n as usize]).collect()
}
