//! LeetCode 1773. Count Items Matching a Rule
//! https://leetcode.com/problems/count-items-matching-a-rule/
//!
//! You are given an array `items`, where each `items[i] = [type_i, color_i,
//! name_i]` describes the type, color, and name of the i-th item. You are also
//! given a rule represented by two strings, `rule_key` and `rule_value`.
//!
//! The i-th item is said to match the rule if one of the following is true:
//! - `rule_key == "type"`  and `rule_value == type_i`
//! - `rule_key == "color"` and `rule_value == color_i`
//! - `rule_key == "name"`  and `rule_value == name_i`
//!
//! Return the number of items that match the given rule.

pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let key = match rule_key.as_str() {
        "type" => 0,
        "color" => 1,
        _ => 2,
    };

    items.iter().fold(0, |acc, item| {
        if item[key] == rule_value {
            acc + 1
        } else {
            acc
        }
    })
}
