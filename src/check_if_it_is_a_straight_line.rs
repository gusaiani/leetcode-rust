//! LeetCode 1232. Check If It Is a Straight Line
//! https://leetcode.com/problems/check-if-it-is-a-straight-line/
//!
//! You are given an array `coordinates`, where `coordinates[i] = [x, y]` are the
//! coordinates of the i-th point on a 2D plane. Return `true` if these points
//! make a straight line in the XY plane, and `false` otherwise.

pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    let (x0, y0) = (coordinates[0][0], coordinates[0][1]);
    let x_delta = coordinates[1][0] - x0;
    let y_delta = coordinates[1][1] - y0;

    for point in coordinates.iter().skip(2) {
        let dx = point[0] - x0;
        let dy = point[1] - y0;

        if (x_delta as i64) * (dy as i64) != (y_delta as i64) * (dx as i64) {
            return false;
        }
    }

    true
}
