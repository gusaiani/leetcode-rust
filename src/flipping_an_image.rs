//! LeetCode 832. Flipping an Image
//! https://leetcode.com/problems/flipping-an-image/
//!
//! Given an `n x n` binary matrix `image`, flip the image horizontally, then
//! invert it, and return the resulting image.
//!
//! To flip an image horizontally means that each row of the image is reversed.
//! - For example, flipping `[1,1,0]` horizontally results in `[0,1,1]`.
//!
//! To invert an image means that each `0` is replaced by `1`, and each `1` is
//! replaced by `0`.
//! - For example, inverting `[0,1,1]` results in `[1,0,0]`.

pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = image.clone();
    let row_length = image[0].len();

    for i in 0..image.len() {
        for j in 0..row_length {
            let mut new_digit = 0;

            if image[i][j] == 0 {
                new_digit = 1;
            }

            result[i][row_length - j - 1] = new_digit;
        }
    }

    result
}
