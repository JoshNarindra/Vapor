# Roadmap

This roadmap is a sequence of hypotheses, not a promise that every item ships in a particular release.

## Phase 0 — Project foundation
- Confirm the repository and target device.
- Establish a minimal development workflow and Git baseline.
- Verify framework and Linux build requirements.
- Add project instructions, tests and security expectations.
- Avoid creating a large scaffold before the first feature needs it.

## Phase 1 — Read-only Task Manager (first complete feature)
- Display real running-process information.
- Show overall CPU and memory information using defined conventions.
- Search and sort the process list.
- Refresh periodically and handle process races.
- Provide technical details on demand.
- Add automated tests for parsing, calculations and errors.
- Build and test on Linux; validate on actual SteamOS hardware.

## Phase 2 — Guarded process control
Only after Phase 1 is reliable:
- Design process termination UX and safeguards.
- Explain target process and likely consequences.
- Confirm consequential actions.
- Use normal-user permissions and report the actual outcome.
- Consider force termination separately; it must not be the default.
- Test permission failures and protected-process behaviour.

## Phase 3 — Integrate Task Manager into Hub shell
- Apply the persistent top navigation and dark design system.
- Build the useful Home system overview.
- Standardise loading, unavailable, empty and error states.
- Keep visual polish improving alongside functionality.

## Phase 4 — Expand coherent core coverage
Choose feature slices based on technical feasibility and user value:
- System information and update status.
- Applications inventory and supported per-app update visibility.
- Files/storage inspection and transparent cleanup review.
- Selected settings journeys.

The long-term aim is broad coverage of the core areas, but not every operation must be native in the first release. Be explicit about which capabilities are native, read-only, delegated to an existing tool, or not yet supported.

## Phase 5 — Unify desktop customisation
- Inventory supported existing SteamOS/KDE customisation.
- Prioritise wallpapers/multiple monitors, panels/taskbar, desktop layout/widgets, window behaviour and shortcuts/quick actions as feasible.
- Prefer native controls where reliable.
- Consider new functionality only after the existing experience is unified.

## Later exploration
- Additional performance telemetry.
- Broader system controls.
- Personal companion/remote dashboard, with security design before implementation.
- Distribution and commercial model.

## Roadmap rules
- Each phase has observable acceptance criteria.
- Do not begin a later phase merely because it appears in this file.
- Re-evaluate scope after device testing and user feedback.
