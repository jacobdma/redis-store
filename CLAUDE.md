# redis-store

A Redis server in Rust. It speaks RESP over TCP, so unmodified Redis clients work against it.

@tasks/lessons.md

Plan, status, and milestones: `tasks/plan.md`
Benchmark method and baselines: `tasks/benchmarks.md`

## Goals

1. **Write good Rust.** Idiomatic code, not only working code.
2. **Beat real Redis on a benchmark.** Performance work is the deliverable here, not premature optimization.

## Code ownership

**Jacob writes all server logic**: parsing, dispatch, storage, and I/O structure.

**Give Rust language mechanics directly and completely.** State syntax, what a construct evaluates to, and any `std` method by name with a doc link. Show complete example functions on data unrelated to Redis.

Test each answer: if he could find it in the Rust Book knowing the search term, state it outright. If it decides how his server behaves, he writes it.

Write real Rust for the repo only when he says "write this."

## Response language

Write prose replies in **AST-STE100 Simplified Technical English**: active voice, short sentences, one instruction per sentence, approved words only, no synonyms, slang, or idioms. Prose only — leave code, code comments, commit messages, file contents, and command output alone.

## Self-improvement loop

After ANY correction from the user: update `tasks/lessons.md` with the pattern. Write rules for yourself that prevent the same mistake. **Ruthlessly iterate on these lessons until the mistake rate drops.** Keep every rule imperative and short. Do not record justifications, dates, or anecdotes. Review lessons at session start.

## Verification before done

Never mark a task complete without proving it works. Run the code, compare its bytes against a freshly started Redis, and report the actual output. Then ask: "Would a staff engineer approve this?"
