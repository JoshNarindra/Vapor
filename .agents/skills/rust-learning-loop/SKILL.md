---
name: rust-learning-loop
description: Use when implementing or reviewing Rust code for SteamOS Hub, especially Task Manager monitoring, to keep changes focused and explain the Rust concepts involved.
---

Use this workflow for Rust implementation and review tasks in SteamOS Hub.

- Work from the current request, relevant code and tests. Read product documents only when they affect the task.
- Keep the change within the agreed scope and follow existing Rust patterns. Raise architectural choices before committing the project to one.
- Explain the Rust concepts that actually appear in the change, such as ownership, borrowing, `Option`, `Result` or data flow. Use the code as the lesson; avoid a general Rust lecture.
- In the handoff, describe the behavior change, key design choices, checks performed and remaining uncertainties in plain language.

For Task Manager work, preserve the read-only scope: run as the normal user, represent unavailable readings honestly, and do not add process-control actions or arbitrary shell execution.
