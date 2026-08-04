---
name: milestone-planning
description: Break a redis-store milestone into verifiable steps. Use when starting a milestone from tasks/plan.md, when Jacob asks what to build next, or when a step proves too coarse.
---

# Milestone planning

## Build it before you plan it

1. Build the finished milestone in the scratchpad. Never in the repo.
2. Run it against `redis-cli -p <scratch port>`.
3. Compare its bytes to a freshly started Redis.
4. Derive the steps from the working code.

Never write steps toward an end state that has not run.

## Write each step

Anchor every step to a line that already exists in `src/main.rs`. Work outside-in: imports,
then declarations, then the loop, then the body.

Give each step four parts:

1. **What changes** — one sentence naming the line or function.
2. **Why** — the mechanical constraint that forces it. One sentence.
3. **The mechanism** — one `std` method or syntax form, with a doc link that resolves. State
   what it returns and what it mutates.
4. **Verify** — a command and its literal expected output.

A step with no verify command is not a step.

## Rules

Change one thing per step. Every step must compile.
Never add a new function, file, or test harness while a transition is in progress.
Quote the exact compiler output the step produces, including the next warning to appear.
Say whether the compiler's suggested fix is correct or wrong.
Split the previous step when he says "I built it from an example online."

## Deliver one step at a time

Give the milestone's step list by name only. Then detail step one and stop.

## Verify the milestone

Run `cargo clippy` and report the output.
Test every command against `redis-cli -p 6380`.
Byte-compare against port 6379 with `nc` and `xxd`.
Update the status column in `tasks/plan.md`.
