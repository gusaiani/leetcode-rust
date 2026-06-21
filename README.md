# LeetCode in Rust

Solutions to LeetCode problems in Rust, each with a test suite.

Companion to the JavaScript repo at `../../js/leetcode`. Problems are not duplicated across the two — each new challenge here is one not yet solved in the JS repo (see [Avoiding duplicates](#avoiding-duplicates)).

## Solved Problems

| #   | Problem                                                       | Difficulty | Approach                       | Time     | Space |
| --- | ------------------------------------------------------------- | ---------- | ------------------------------ | -------- | ----- |
| 27  | [Remove Element](https://leetcode.com/problems/remove-element/) | Easy     | Two pointers (write head)      | O(n)     | O(1)  |
| 67  | [Add Binary](https://leetcode.com/problems/add-binary/)       | Easy       | Reverse two-pointer + carry    | O(n)     | O(n)  |
| 387 | [First Unique Character in a String](https://leetcode.com/problems/first-unique-character-in-a-string/) | Easy | Two-pass byte frequency (`[i32; 26]`) | O(n) | O(1) |
| 412 | [Fizz Buzz](https://leetcode.com/problems/fizz-buzz/)         | Easy       | Iterator + tuple `match`       | O(n)     | O(n)  |
| 703 | [Kth Largest Element in a Stream](https://leetcode.com/problems/kth-largest-element-in-a-stream/) | Easy | Min-heap of size k (`Reverse`) | O(log k) | O(k) |
| 704 | [Binary Search](https://leetcode.com/problems/binary-search/) | Easy       | Half-open interval + `cmp`     | O(log n) | O(1)  |
| 771 | [Jewels and Stones](https://leetcode.com/problems/jewels-and-stones/) | Easy | `HashSet` membership (bytes) | O(n + m) | O(1) |
| 1480 | [Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/) | Easy | In-place prefix sum (owned `Vec`) | O(n) | O(1) |
| 344 | [Reverse String](https://leetcode.com/problems/reverse-string/) | Easy | Indexed swap (`0..n/2`, `<[T]>::swap`) | O(n) | O(1) |
| 1672 | [Richest Customer Wealth](https://leetcode.com/problems/richest-customer-wealth/) | Easy | Iterator chain (row `sum` + `max`) | O(m × n) | O(1) |
| 643 | [Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i/) | Easy | Fixed-size sliding window (rolling sum) | O(n) | O(1) |
| 191 | [Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/) | Easy | Brian Kernighan (`n &= n - 1`) | O(1) | O(1) |
| 1768 | [Merge Strings Alternately](https://leetcode.com/problems/merge-strings-alternately/) | Easy | Tuple-match on `chars().next()` | O(n + m) | O(n + m) |
| 1929 | [Concatenation of Array](https://leetcode.com/problems/concatenation-of-array/) | Easy | Push loop over borrowed `&Vec` | O(n) | O(n) |
| 1365 | [How Many Numbers Are Smaller Than the Current Number](https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/) | Easy | Counting sort + prefix sum (`[i32; 101]`) | O(n) | O(1) |
| 1047 | [Remove All Adjacent Duplicates In String](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/) | Easy | Stack (`Vec<u8>` push/pop) | O(n) | O(n) |
| 1342 | [Number of Steps to Reduce a Number to Zero](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/) | Easy | Parity loop (`% 2`, halve/decrement) | O(log n) | O(1) |
| 228 | [Summary Ranges](https://leetcode.com/problems/summary-ranges/) | Easy | Linear scan (consecutive-run grouping) | O(n) | O(n) |
| 118 | [Pascal's Triangle](https://leetcode.com/problems/pascals-triangle/) | Easy | Additive DP (each entry = sum of two above) | O(n²) | O(1) |
| 1281 | [Subtract the Product and Sum of Digits of an Integer](https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/) | Easy | Digit peeling (`% 10`, `/= 10`) | O(log n) | O(1) |
| 1920 | [Build Array from Permutation](https://leetcode.com/problems/build-array-from-permutation/) | Easy | Iterator `map` with index cast (`nums[v as usize]`) | O(n) | O(n) |
| 977 | [Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/) | Easy | Converging two pointers, fill result back-to-front | O(n) | O(n) |
| 1822 | [Sign of the Product of an Array](https://leetcode.com/problems/sign-of-the-product-of-an-array/) | Easy | Fold a 3-state sign (`cmp` + `match` on `Ordering`), overflow-proof | O(n) | O(1) |
| 1431 | [Kids With the Greatest Number of Candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/) | Easy | Max-scan then map (`iter().max()` + `map().collect()`) | O(n) | O(n) |
| 1313 | [Decompress Run-Length Encoded List](https://leetcode.com/problems/decompress-run-length-encoded-list/) | Easy | Pairwise scan (`chunks_exact(2)` + `extend(repeat_n(val, freq))`) | O(n) | O(n) |
| 171 | [Excel Sheet Column Number](https://leetcode.com/problems/excel-sheet-column-number/) | Easy | Horner's method, base-26 over `bytes()` (`result * 26 + (b - b'A' + 1)`) | O(n) | O(1) |
| 867 | [Transpose Matrix](https://leetcode.com/problems/transpose-matrix/) | Easy | Pre-allocate `n × m` grid, copy `matrix[i][j]` → `result[j][i]` | O(m × n) | O(m × n) |
| 1232 | [Check If It Is a Straight Line](https://leetcode.com/problems/check-if-it-is-a-straight-line/) | Easy | Cross product vs. fixed reference point 0 (`i64` widening, no division) | O(n) | O(1) |
| 1389 | [Create Target Array in the Given Order](https://leetcode.com/problems/create-target-array-in-the-given-order/) | Easy | Insertion simulation (`zip` + `Vec::insert` with `usize` cast) | O(n²) | O(n) |
| 1979 | [Find Greatest Common Divisor of Array](https://leetcode.com/problems/find-greatest-common-divisor-of-array/) | Easy | Min/max scan + recursive Euclidean GCD (`gcd(b, a % b)`) | O(n) | O(log min) |
| 1295 | [Find Numbers with Even Number of Digits](https://leetcode.com/problems/find-numbers-with-even-number-of-digits/) | Easy | Linear scan + integer digit count (`while n >= 10`, `/= 10`) | O(n × d) | O(1) |
| 2114 | [Maximum Number of Words Found in Sentences](https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/) | Easy | Map each sentence to `split_whitespace().count()`, then `max()` | O(n) | O(1) |
| 1512 | [Number of Good Pairs](https://leetcode.com/problems/number-of-good-pairs/) | Easy | Brute-force pair scan (`i < j` nested loops) | O(n²) | O(1) |
| 1470 | [Shuffle the Array](https://leetcode.com/problems/shuffle-the-array/) | Easy | Pre-allocate then push interleaved pairs (`nums[i]`, `nums[i + n]`) | O(n) | O(n) |
| 1832 | [Check if the Sentence Is a Pangram](https://leetcode.com/problems/check-if-the-sentence-is-a-pangram/) | Easy | 26-bit bitmask set (`seen \|= 1 << (b - b'a')`), compare to `(1 << 26) - 1` | O(n) | O(1) |
| 1207 | [Unique Number of Occurrences](https://leetcode.com/problems/unique-number-of-occurrences/) | Easy | `HashMap` frequency count, then compare value-`HashSet` size to map size | O(n) | O(n) |
| 605 | [Can Place Flowers](https://leetcode.com/problems/can-place-flowers/) | Easy | Greedy in-place scan, plant when plot + both neighbors empty (off-the-end = empty) | O(n) | O(1) |
| 496 | [Next Greater Element I](https://leetcode.com/problems/next-greater-element-i/) | Easy | Monotonic stack builds value→next-greater `HashMap` over `nums2`, then map `nums1` (`unwrap_or(&-1)`) | O(n + m) | O(n) |

## Project structure

- Implementation: `src/<problem_name>.rs`
- Tests: `tests/<problem_name>.rs` (Rust integration tests)
- Module list: `src/lib.rs` declares each problem as `pub mod <name>;`
- File and module names are `snake_case`; branches are `kebab-case`.

## Setup

Requires a Rust toolchain (stable). Install via [rustup](https://rustup.rs/) if needed.

```sh
cargo build --tests
```

### Pre-push hook

A repo-tracked git hook at `.githooks/pre-push` runs the same checks CI runs (`cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`) before each `git push`. Enable it once per clone:

```sh
git config core.hooksPath .githooks
```

Bypass for a single push with `git push --no-verify` if you really need to.

## Scripts

| Command                                            | Description                          |
| -------------------------------------------------- | ------------------------------------ |
| `cargo test`                                       | Run all tests                        |
| `cargo test --test <problem_name>`                 | Run a single problem's tests         |
| `cargo test --test <problem_name> -- --nocapture`  | Show stdout from a single test file  |
| `cargo build --tests`                              | Type-check everything without running |
| `cargo fmt`                                        | Format with rustfmt                  |
| `cargo clippy --all-targets`                       | Lint                                 |

## Workflow

1. Ask Claude to "Stub challenge `<problem>`" or "Stub job challenge".
2. Implement the function in `src/<problem_name>.rs` — replace `todo!()` with your solution.
3. Run `cargo test --test <problem_name>` until green.
4. Update the row in [Solved Problems](#solved-problems): fill in Approach, Time, Space; change Status to `Solved`.
5. Commit on the problem branch and merge.

## Avoiding duplicates

Before picking a new challenge, read the "Solved Problems" table in **all four sibling repos** — they share one problem space:

- this repo's [Solved Problems](#solved-problems) table
- `/Users/gustavosaiani/code/estudo/js/leetcode/README.md`
- `/Users/gustavosaiani/code/estudo/python/leetcode/README.md`
- `/Users/gustavosaiani/code/estudo/java/leetcode/README.md`

Never pick a problem already in **any** of those tables. "Already in" includes close variants and same-family problems, not just exact `#` matches — e.g. if Java has #350 *Intersection of Two Arrays II*, then #349 *Intersection of Two Arrays* is also off-limits. When a candidate shares a name stem, a problem family (the "II"/"III" follow-ups), or essentially the same core task as something already solved, skip it and pick something genuinely new.

## Code Quality

- **Testing** — every solution has a dedicated integration test file under `tests/` exercising LeetCode examples plus edge cases.
- **Stubs use `todo!()`** — stubbed solutions panic at runtime so unfinished work fails loudly instead of silently passing.
- **Formatting & linting** — `cargo fmt` and `cargo clippy --all-targets`.
