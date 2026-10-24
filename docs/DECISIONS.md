# Decisions, Preferences and Open Questions

This file separates stronger user preferences from provisional ideas. Do not treat every brainstormed option as a binding requirement.

## Strong direction / agreed preferences

- Product name: **SteamOS Hub** (working name; retain unless the owner changes it).
- Target product: software specifically for Valve Steam Machine / SteamOS hardware, not a generic Linux system utility.
- Initial interaction mode: Desktop Mode. Full-screen SteamOS/Gaming Mode is a future possibility, not a V1 requirement.
- Product is a unified control centre for SteamOS, not a replacement OS, Gaming Mode or KDE Plasma.
- Initial release should be focused rather than ambitious.
- First substantial feature: Task Manager.
- First Task Manager milestone: read-only monitoring with real process data.
- Implementation order agreed 2026-10-24: begin with a holistic, system-wide resource overview; add per-process attribution later.
- Working stack agreed 2026-10-24: Tauri desktop shell, React + TypeScript UI, and Rust for system-facing monitoring. This is the chosen direction for implementation, not a claim that other stacks were benchmarked.
- CPU utilization for system and process views means share of total online logical-CPU time capacity, normalized to 0–100% over a sampling interval; it is not a per-logical-CPU percentage.
- Task Manager UX: balanced — friendly overview by default, technical detail on demand.
- Process termination is excluded from the first milestone and requires separate safeguards later.
- Visual direction: dark, SteamOS-inspired, polished and cohesive.
- The user liked a reference screenshot's horizontal top navigation and active underline.
- Intended layout: persistent horizontal top navigation; contextual sidebar as useful; information-rich but uncluttered main content; notifications near top navigation.
- Visual design is a starting point and may become more visually distinctive during development.
- Hubs own appearance should remain consistent initially; a theme dropdown may be considered later.
- The Home screen should be a useful system overview dashboard, not a hybrid dashboard that resumes the last-used screen.
- Main areas discussed: Home, System, Files, Applications, Updates, Customisation; Task Manager is a major early feature and may be placed within System or top-level navigation after UX refinement.
- User wants a coherent first release that can be built upon, not an isolated disk cleaner.
- User is learning Rust by reviewing AI-generated code rather than starting with a Rust-first curriculum.
- Use Mac + VS Code + Codex + Git for development; test on Linux and actual SteamOS hardware.
- Prefer small, reviewable implementation increments and explanations.
- User prefers unified application inventory rather than splitting applications and games by default.
- No “Update All” feature desired; per-app update action only when supported and applicable.
- No app store/installation feature in current plan.
- Settings/customisation ambition is to unify existing SteamOS/KDE functionality first, then improve awkward workflows and potentially add new capabilities.
- Guided, transparent storage cleanup; no blanket “junk” labels or deletion based solely on size/age.
- Do not run as root by default or expose arbitrary shell execution to the frontend.
- Do not replace SteamOS update mechanisms.
- Remote management is future scope, not V1.

## Agreed working technical direction

- Tauri desktop shell with React + TypeScript UI.
- Rust system-monitoring backend, kept separate from presentation logic.
- Monorepo and Git.
- Proposed pnpm workspaces for TypeScript and Cargo workspace for Rust are options, not yet committed.
- Proposed layering: UI, application/domain logic, OS-specific adapters.
- Linux process data can be read through a suitable maintained Rust library or documented Linux interface; select after investigation.
- Build/test on Linux and verify on the actual SteamOS target. Mac-only success is insufficient.
- Keep architecture modular but avoid speculative abstraction and creating empty packages.
- Distribution work, including Flatpak, is deferred until the minimum viable product exists. Do not let packaging decisions block the first working slice.

## Open questions (do not block the first read-only milestone unless essential)

- Exact SteamOS device and OS version used for first validation.
- Exact Linux/SteamOS build and validation setup.
- Exact target Linux build environment and dependency installation approach.
- Exact Tauri/Rust/Node/pnpm versions after compatibility check.
- Whether Task Manager is a top-level tab or nested under System.
- Process grouping rules and friendly-name mapping.
- CPU sampling interval and exact idle/I/O-wait accounting convention.
- Which memory metrics should be shown by default.
- Exact system-wide GPU activity and memory counters available on target SteamOS hardware.
- Whether a separate notification history is needed.
- Final app name/branding, icons, typography, accent shades and polished visual language.
- Hub theme selector.
- Exact V1 distribution route and commercial model (deferred until after the MVP).
- Scope and implementation of later feature areas.

## Decision discipline

When a new choice is made:
1. Record the decision and date/context.
2. Label it as agreed, provisional, or open.
3. Explain meaningful trade-offs in plain language.
4. Do not silently turn future ideas into V1 scope.
