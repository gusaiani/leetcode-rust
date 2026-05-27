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

Before picking a new challenge, check both:

- this repo's [Solved Problems](#solved-problems) table
- the JS repo's table at `/Users/gustavosaiani/code/estudo/js/leetcode/README.md`

Skip anything already in either. The two repos share a problem space.

## Code Quality

- **Testing** — every solution has a dedicated integration test file under `tests/` exercising LeetCode examples plus edge cases.
- **Stubs use `todo!()`** — stubbed solutions panic at runtime so unfinished work fails loudly instead of silently passing.
- **Formatting & linting** — `cargo fmt` and `cargo clippy --all-targets`.
