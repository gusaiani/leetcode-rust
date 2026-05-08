## Project structure

- Implementation files: `src/<problem_name>.rs` (e.g., `src/binary_search.rs`)
- Test files: `tests/<problem_name>.rs` (e.g., `tests/binary_search.rs`)
- Module registration: every implementation file is declared in `src/lib.rs` as `pub mod <problem_name>;`
- Branch name: kebab-case problem name (e.g., `binary-search`)
- File and module names use `snake_case` (Rust convention), even though branches and LeetCode titles are kebab-case.

Implementation file template:
```rust
//! LeetCode <number>. <Title>
//! <url>
//!
//! <problem description>

pub fn <function_name>(<params>) -> <ReturnType> {
    let _ = (<params>);
    todo!()
}
```

Test file template (integration test under `tests/`):
```rust
use leetcode::<problem_name>::<function_name>;

#[test]
fn example_1() {
    assert_eq!(<function_name>(<input>), <expected>);
}
```

Run tests:
- `cargo test` — run all tests
- `cargo test --test <problem_name>` — run a single problem's test file
- `cargo test --test <problem_name> -- --nocapture` — show stdout

## Solved problems

Read `README.md` — the "Solved Problems" table is the source of truth for which problems are already solved here and what patterns/approaches they cover. Use the Approach column to infer covered patterns when doing gap analysis.

## Avoiding duplicates with the JS repo

This repo is a companion to `/Users/gustavosaiani/code/estudo/js/leetcode`. When picking a new challenge, also read that repo's `README.md` "Solved Problems" table and **skip any problem already solved there**. The goal is breadth across both repos, not redundant coverage.

---

Use the term "Stub challenge" as a command to:

- create a branch named after the kebab-case problem
- create a test file at `tests/<problem_name>.rs` with multiple test cases (cover examples + edge cases)
- create an implementation file at `src/<problem_name>.rs` with a stubbed function body using `todo!()`
- register the module in `src/lib.rs` with `pub mod <problem_name>;`
- update `README.md`'s "Solved Problems" table with a new row marking the problem as stubbed (leave Approach/Time/Space blank or as `—` until solved)
- open the editor with both files: `cursor tests/<problem_name>.rs src/<problem_name>.rs`

Do not give hints about how to solve the problem.

Use the term "Stub job challenge" as a command to:

1. Pick a LeetCode challenge that maximizes odds of passing a technical interview for a senior engineer role paying US$150–200k/year, remote from Brazil.
2. Consider which patterns and topics are missing from this repo **and** from the JS repo at `/Users/gustavosaiani/code/estudo/js/leetcode/README.md`. Skip any problem already solved in either repo. Fill the most impactful gap (e.g., BFS, DFS, dynamic programming, sliding window, stack, trie, graph traversal, matrix problems, bit manipulation).
3. Difficulty mix: 50% easy, 40% medium, 10% hard.
4. "Stub challenge" from the choice of challenge above.
5. Explain why the chosen problem is relevant for senior interviews at that salary range, and call out the Rust-specific learning value (ownership, borrowing, lifetimes, iterators, traits) when applicable.
