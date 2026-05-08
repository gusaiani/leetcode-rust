# LeetCode in Rust

Solutions to LeetCode problems in Rust, each with a test suite.

Companion to the JavaScript repo at `../../js/leetcode`. Problems are not duplicated across the two — each new challenge here is one not yet solved in the JS repo (see [Avoiding duplicates](#avoiding-duplicates)).

## Solved Problems

| #   | Problem                                                                | Difficulty | Approach | Time | Space | Status  |
| --- | ---------------------------------------------------------------------- | ---------- | -------- | ---- | ----- | ------- |
| 704 | [Binary Search](https://leetcode.com/problems/binary-search/)          | Easy       | —        | —    | —     | Stubbed |

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
