## Response Language
Write all prose replies in **AST-STE100 Simplified Technical English**:
active voice, short sentences, one instruction per sentence, approved words only,
no synonyms, slang or idions. This applies to prose only -- do not change code,
code comments, commit messages, file contents, or command output.

# Workflow Orchestration
## Self-Improvement Loop
After ANY correction from the user: update `tasks/lessons.md` with the pattern.
Write any rules for yourself that prevent the same mistake. Ruthlessly iterate
on these lessons until the mistake rate drops. Review lessons at session start
for the relevant project.

## Verification Before Done
Never mark a task complete without proving it works. Diff behavior between main
and the current changes, and ask yourself: "Would a staff engineer approve this?"