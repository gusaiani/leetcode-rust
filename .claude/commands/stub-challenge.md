Stub a LeetCode challenge in this Rust repo.

Steps:

1. Confirm the problem is not already solved in **any** of the four sibling repos — read each "Solved Problems" table:
   - this repo's `README.md`
   - `/Users/gustavosaiani/code/estudo/js/leetcode/README.md`
   - `/Users/gustavosaiani/code/estudo/python/leetcode/README.md`
   - `/Users/gustavosaiani/code/estudo/java/leetcode/README.md`
   "Already solved" includes close variants and same-family problems, not just exact `#` matches (e.g. if any repo has #350 *Intersection of Two Arrays II*, then #349 *Intersection of Two Arrays* is also off-limits). If the problem — or a same-family variant — appears in any table, stop and report instead of stubbing.
2. Create and switch to a branch named after the kebab-case problem title (e.g., `binary-search`).
3. Create `tests/<problem_name>.rs` with multiple test cases — cover the LeetCode examples plus edge cases (empty input, single element, extremes, large input where reasonable). Use `snake_case` for the file.
4. Create `src/<problem_name>.rs` with a doc comment header (number, title, URL, problem description) and a stub function whose body is `todo!()`. Suppress unused-variable warnings on the parameters with `let _ = (...);` so the stub compiles cleanly.
5. Register the module in `src/lib.rs` by adding `pub mod <problem_name>;`.
6. Add a new row to the README "Solved Problems" table. Leave Approach/Time/Space as `—` until the user solves it.
7. Run `cargo build --tests` to confirm everything compiles.
8. Open both files: `cursor tests/<problem_name>.rs src/<problem_name>.rs`.

Do not give hints about how to solve the problem. Do not write the implementation. Only stub.

Argument: $ARGUMENTS — the problem identifier (number, title, or LeetCode URL). If empty, ask the user which problem to stub.
