//! LeetCode 804. Unique Morse Code Words
//! https://leetcode.com/problems/unique-morse-code-words/
//!
//! International Morse Code defines a standard encoding where each letter is
//! mapped to a string of dots and dashes (`'a'` maps to `".-"`, `'b'` maps to
//! `"-..."`, ..., `'z'` maps to `"--.."`).
//!
//! Given an array of strings `words` where each word can be written as a
//! concatenation of the Morse code of each letter, return the number of
//! different transformations among all the words.

use std::collections::HashSet;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let morse = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];

    let mut seen: HashSet<String> = HashSet::new();

    for word in words {
        let mut transformation = String::new();

        for byte in word.into_bytes() {
            let index = (byte - b'a') as usize;
            transformation.push_str(morse[index]);
        }
        seen.insert(transformation);
    }

    seen.len() as i32
}

// cab
// c -.-.
// a .-
// b -...
