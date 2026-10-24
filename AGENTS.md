# Instructions for AI Coding Agents

## Project context
- Read `docs/CURRENT_TASK.md` for Task Manager work; consult other product documents when they are relevant to the task.
- Treat handoff documents as product intent and provisional technical direction. Verify implementation facts against current code, tests and configuration.
- If a document conflicts with a newer explicit owner decision or observed implementation, surface the conflict and use the newer, more authoritative evidence.

## Project purpose
SteamOS Hub is a unified SteamOS control centre. The long-term vision is broad; the current implementation target is a read-only Task Manager. Do not interpret the long-term feature list as permission to implement everything now.

## Working with the owner
The owner is learning Rust by reviewing AI-generated code. Explain unfamiliar concepts in practical language. Prefer small changes that can be understood, tested and reviewed. Do not dump a complete application in one implementation pass.

## Before coding
1. Inspect the repository and current environment.
2. Summarise what exists and identify blockers.
3. Propose a small plan with likely files, dependencies, tests and risks.
4. Ask before major scope changes, new architectural commitments, significant dependencies or destructive operations.
5. Do not claim a task is complete before running relevant checks.

## Implementation rules
- Follow existing conventions; avoid needless restructuring.
- Prefer maintained libraries and documented OS interfaces.
- Keep UI, domain logic and OS-specific code appropriately separated.
- Avoid speculative abstractions and empty scaffolding.
- Keep the app running as a normal user.
- Do not expose arbitrary shell execution to the frontend.
- Do not add process termination in the first milestone.
- Do not run the app as root or ask to modify the SteamOS base OS as a routine setup step.
- Do not fabricate system readings or treat unavailable values as zero.
- Do not claim SteamOS compatibility until it has been tested on the target device.

## Testing and review
For each meaningful change:
1. Add/update tests.
2. Run formatter, linter and relevant tests.
3. Inspect the diff.
4. Review for security, correctness, complexity and error handling.
5. Explain what changed, why, test results, and limitations.

Use fixtures/mocks for process data. Tests must not kill arbitrary processes or modify real user data.

## Current scope
Only implement `docs/CURRENT_TASK.md` after the owner approves the plan. Later roadmap items remain out of scope unless the owner explicitly changes priorities.

When asked to validate a change on the Steam Machine, read `.agents/skills/steam-machine-validation/SKILL.md`. Use a GitHub Issue as the per-test handoff and result record when GitHub is available; do not add a separate ticketing system unless the owner asks.
