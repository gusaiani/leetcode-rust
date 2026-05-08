Pick and stub a LeetCode challenge optimized for senior-engineer interview prep.

Steps:

1. Read the "Solved Problems" tables in both:
   - this repo's `README.md`
   - `/Users/gustavosaiani/code/estudo/js/leetcode/README.md`
   Build a combined set of problems and patterns already covered. **Never pick a problem that appears in either table.**
2. Identify the weakest pattern coverage across the combined set: BFS, DFS, dynamic programming, sliding window, two pointers, stack, monotonic stack, trie, graph traversal, topological sort, union-find, matrix, bit manipulation, heap, binary search, backtracking, greedy, prefix sum, intervals, linked list, hash map.
3. Pick a LeetCode problem that:
   - fills the most impactful gap from step 2,
   - is a known interview problem at companies paying US$150–200k/year remote from Brazil,
   - matches the requested difficulty mix over time: 50% easy / 40% medium / 10% hard. Look at the recent additions in both repos and choose the difficulty that keeps the running mix on target.
4. Run `/stub-challenge` (or follow its steps) for the chosen problem.
5. After stubbing, explain:
   - why the chosen problem is relevant for senior interviews at that salary band,
   - which pattern gap it fills,
   - the Rust-specific learning value (ownership, borrowing, lifetimes, iterators, traits, error handling) — call it out only when actually relevant.

Do not give hints about how to solve the problem.

Argument: $ARGUMENTS — optional difficulty override (`easy`, `medium`, `hard`). If absent, follow the running mix in step 3.
