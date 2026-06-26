//! LeetCode 2011. Final Value of Variable After Performing Operations
//! https://leetcode.com/problems/final-value-of-variable-after-performing-operations/
//!
//! There is a programming language with only four operations and one variable X:
//!   - ++X and X++ increment the value of X by 1.
//!   - --X and X-- decrement the value of X by 1.
//!
//! Initially, the value of X is 0.
//!
//! Given an array of strings `operations` containing a list of operations,
//! return the final value of X after performing all the operations.

pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut result = 0;

    for operation in operations {
        if operation.contains("--") {
            result -= 1;
        } else {
            result += 1;
        }
    }

    result
}
