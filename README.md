# Vapor

Working title for SteamOS Hub.

**Project status:** Early implementation
**Current focus:** Build the read-only, whole-machine system overview for Steam Machine; the first Linux memory reading is connected, while other metrics remain unavailable.

SteamOS Hub is intended to make SteamOS easier to understand and manage by bringing existing system capabilities into one coherent, approachable control centre. It is not a replacement operating system, Gaming Mode, or KDE Plasma.

## Start here

1. Read `docs/PRODUCT_BRIEF.md` for the full product vision.
2. Read `docs/DECISIONS.md` to distinguish agreed direction from open questions.
3. Read `docs/ROADMAP.md` for the staged scope.
4. Read `docs/UX_AND_DESIGN.md` for the working visual direction.
5. Read `docs/ARCHITECTURE.md` for the proposed technical approach.
6. Read `AGENTS.md` for working rules for AI coding agents.
7. Read `docs/CURRENT_TASK.md` before implementing anything.

## Important status note

This is a curated handover based on the conversation context available at the time of writing. It is not a verbatim transcript of every message from any earlier chat. Where details are uncertain, they are marked provisional or open rather than presented as settled facts.

## Current milestone

Build a **read-only system overview** for the target SteamOS device. The Tauri + React screen now reads physical RAM from Linux `/proc/meminfo`; CPU, GPU, disk and network are still deliberately unavailable until their readers are implemented and validated. Per-process monitoring comes later; do not implement process termination in this milestone.

For actual-device validation, follow `.agents/skills/steam-machine-validation/SKILL.md`. GitHub Issues are the lightweight place to hand off individual test runs and record bugs/results.

Development starts in Desktop Mode. Full-screen SteamOS/Gaming Mode support and distribution (including Flatpak) are deferred until there is a minimum viable product to test.

## Working style

The project owner is learning Rust by reviewing AI-generated code. Prefer small, understandable, testable increments; explain decisions in practical language; ask before expanding scope or introducing major architectural changes. Do not generate the entire application in one go.
