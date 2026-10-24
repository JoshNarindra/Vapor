# Proposed Architecture and Engineering Constraints

## Working stack

- Tauri for desktop application shell.
- React + TypeScript for UI.
- Rust for system-facing operations and core logic.
- Git repository and one monorepo.
- macOS development in VS Code with Codex.
- Linux build/test path plus real SteamOS validation.

The owner approved this stack as the implementation direction. It has not been benchmarked against a native Rust UI. The first system-memory reading is connected through Linux `/proc/meminfo`; the other overview metrics are not yet connected. Steam Machine Desktop Mode is the first target; full-screen SteamOS/Gaming Mode is a later possibility.

## Distribution boundary

Defer distribution and installation choices, including Flatpak, until a minimum viable product exists. Develop and validate the application first; do not treat packaging as a prerequisite for the first read-only overview.

## Boundaries

Keep a clear separation between:
1. UI and presentation state.
2. Domain models and application logic.
3. Linux/SteamOS-specific inspection and system adapters.
4. Tests, fixtures and integration boundaries.

A monorepo could later use pnpm workspaces for frontend packages and a Cargo workspace for Rust crates. Do not create empty packages just to match a proposed diagram. Introduce boundaries when code and tests justify them.

## Process monitoring

- Prefer a suitable maintained Rust library or documented Linux interface over ad-hoc shell parsing.
- Investigate available process metadata and permissions on the target SteamOS version.
- Treat processes as ephemeral: a process may exit between listing and detail reads.
- Handle inaccessible process details without failing the entire list.
- Define memory metrics and units explicitly.
- Define how CPU percentages are computed, sampled and normalised.
- Keep system sampling out of the UI thread.
- Return structured data and typed errors across the frontend/backend boundary.
- Avoid expensive full-system scans at unnecessarily high frequency.

## Tauri and command execution

- Use narrow Tauri capabilities and commands.
- Grant only the frontend permissions currently needed. The first RAM reading uses one narrow custom Tauri command to read `/proc/meminfo`; it does not need plugin permissions.
- Do not expose arbitrary shell execution to the frontend.
- If a future operation needs an external command, use a narrowly scoped backend operation with fixed executable/validated arguments and a documented reason.
- Validate data crossing the frontend/backend boundary.

## SteamOS/Linux constraints

SteamOS is Arch-based and may have an immutable/read-only base design. Do not alter the base OS or require installing development tools onto the target device as a default workflow. Investigate appropriate build, packaging and runtime dependencies. A Mac build does not prove Linux compatibility. Test on a suitable Linux environment and then the actual target device.

## Security

- Run as the normal logged-in user.
- Never require root for routine monitoring.
- Avoid privileged helpers unless a specific later requirement justifies a threat model and separate design.
- Keep file-system and process-control features narrowly scoped.
- For future destructive operations: inspect, preview, explain, confirm, act, and verify the result.
- Do not let a UI confirmation be treated as blanket permission for future destructive actions.
- Do not add remote endpoints or a remote agent in V1.

## Testing

- Unit tests for parsing, calculations, model transformations and error handling.
- Fixtures/mocks for process data.
- Integration tests where useful.
- Never let test code terminate arbitrary host processes or delete real user data.
- Validate Linux-specific behaviour and permissions.
- Record what has actually been tested; do not claim device compatibility without evidence.
