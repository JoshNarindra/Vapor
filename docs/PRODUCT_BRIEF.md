# SteamOS Hub — Product Brief

**Status:** Working brief; product discovery is ongoing
**Scope distinction:** Long-term vision is broad; V1 is intentionally focused.

## 1. Vision

SteamOS Hub is a unified control centre for SteamOS. It should make the system's existing capabilities easier to find, understand and use, instead of forcing users to jump between separate apps, settings pages, and terminal commands.

The product should feel like a coherent application rather than a collection of links to utilities. Over time, it should provide native, well-designed workflows where technically reliable, while respecting SteamOS's existing mechanisms and KDE Plasma.

It is not a new operating system and is not intended to replace Gaming Mode or KDE Plasma.

## 2. Problem to solve

The user sees value in Linux and SteamOS's lightweight, straightforward computing approach, but experiences fragmentation: useful capabilities are spread across separate tools, settings, and commands; discoverability and search can be awkward; and workflows can feel inconsistent.

SteamOS Hub should reduce that fragmentation while staying transparent about what it knows, what it can safely do, and what remains under the control of SteamOS or another tool.

## 3. Product principles

1. **Unify before reinventing.** Make existing SteamOS capabilities easier to reach and understand. Build new functionality only where it adds real value.
2. **Broad vision, staged delivery.** The eventual product spans several areas, but each milestone should be small and verifiable.
3. **Friendly by default, technical on demand.** Use clear language and progressive disclosure, without removing advanced information.
4. **Truthful system state.** Show observed facts, explain unavailable information, and never fabricate measurements or claim actions succeeded without evidence.
5. **Safety and user control.** Explain consequences, preview meaningful changes, ask for confirmation for consequential actions, and avoid blanket destructive operations.
6. **Respect the platform.** Do not replace supported SteamOS update behaviour or assume all Linux distributions and SteamOS versions behave identically.
7. **Consistent design.** The same navigation, terminology, spacing and interaction patterns should carry across sections.
8. **Local-first initially.** No remote-management server, cloud account, or remote agent is required for the initial release.
9. **Expandable architecture, not premature architecture.** Keep real boundaries clear, but do not create packages or abstractions without a demonstrated need.
10. **Learn through building.** AI-generated code must be reviewable, explained, tested and improved in small increments.

## 4. Intended audience and devices

The intended audience includes SteamOS users who want a simpler way to understand and manage their device, including users who are not comfortable with Linux internals and more technical users who need deeper details.

The product should eventually support multiple SteamOS device configurations rather than assume one fixed hardware setup. It should detect actual hardware, OS version, storage and configuration at runtime where appropriate. Validate first on a known device, then expand coverage through testing.

Exact supported device list and minimum OS versions remain open.

## 5. Long-term feature areas

Names and boundaries are provisional.

### Home
A useful system overview dashboard, not a generic welcome screen. It should show a small set of meaningful current facts and provide direct routes into the main sections. The top navigation remains visible so users can switch sections quickly.

### System
System information and eventually commonly used system controls. Show the installed OS version and relevant hardware information where reliable. Avoid presenting unsupported or unavailable readings as zeros.

### Files and storage
Both ordinary file management and storage analysis/cleanup are important to the long-term product. Potential capabilities include browsing folders and drives, copy/move/rename/delete, search and sorting, explanations of Linux folder purposes, storage analysis, and identifying potentially large game data, shader caches or Proton-related files. Advanced permissions are later scope.

Cleanup should be guided and transparent, not a generic “junk” score. Show actual measured size and estimated reclaimable space separately; explain what an item is, what uses it, what is known or unknown, possible side effects, reversibility and the exact items proposed for action. Do not delete based only on age or size. Never imply exact contents of a directory that has not actually been inspected.

Example of desired explanatory style:
> Large compatibility-data folder — Purpose needs verification. Associated game and exact path should be identified before suggesting removal. Inspect contents →

For caches, explain measured size, likely use, whether contents can be regenerated, and possible side effects. Distinguish caches from saves, settings, and valuable user data. Prefer review and confirmation before deletion, and reversible actions where feasible.

### Applications
Long-term goal: a unified inventory of installed applications and games, rather than separate inventories by default. Possible information includes source, installed version, update date where reliable, size and associated files where feasible. Support discovery and launching. Respect different installation and update mechanisms; do not pretend all software can be managed identically. Per-app Update actions may be appropriate when genuinely supported; do not show an Update button when no update is available. An “Update All” feature is not desired. Installation/store functionality is not part of the current plan. Uninstalling may be considered later with safeguards and source-specific mechanisms.

### Task Manager
Show running applications and background processes, CPU/memory usage and technical details on demand. Safe process termination may be added after read-only monitoring is reliable. Protect sensitive system processes, use normal-user permissions and do not run the Hub as root by default. Advanced performance information such as GPU, temperature, fan and power may be investigated later.

### Updates
Respect SteamOS's supported update mechanisms and automation; do not replace them. Show installed OS version, reliable update status and meaningful events/status where supported. For applications, show per-app status and only provide actions that are supported by the relevant installation source.

### Settings and Customisation
Long-term ambition is a unified settings destination, eventually replacing many fragmented settings journeys with native Hub controls where reliable. Begin by mapping existing settings and prioritising common or awkward workflows; do not remain only a directory of links forever, but do not rebuild everything prematurely.

Desktop customisation specifically means unifying what SteamOS/KDE Plasma already provides first, not inventing a new desktop environment. Longer-term priorities discussed include wallpapers/multiple monitors, taskbar/panels, desktop layout/widgets, window behaviour, keyboard shortcuts and quick actions. New capabilities may be considered after the existing experience is unified. Customisation is a lower priority than the initial core functionality.

The Hub's own appearance should initially be consistent. A Hub light/dark/system theme selector may be added later; it is not an initial requirement.

### Notifications
Keep active notifications relevant and restrained. Avoid nagging and permanent clutter. Resolved items should disappear from the active view; dismissing a notification is not the same as resolving the underlying condition. Device state must remain accurate. A separate notification history is undecided and likely not needed initially.

## 6. Remote management — future only

A future companion experience may start with a phone dashboard for checking storage, OS version, notifications and running apps, then expand toward more control. This is not V1 and should not trigger early remote infrastructure work.

If explored later, security expectations include explicit pairing, strong authentication, encryption, least privilege, a visible and disableable agent, and no public endpoint by default. A VPN/established remote-access approach may be safer than inventing an exposed service; no specific solution has been selected.

## 7. Commercial and distribution thoughts — unvalidated

Ideas discussed include a possible low one-time purchase, distribution through Steam, Flatpak/Flathub or a direct website, and avoiding a subscription for core local utilities. These are hypotheses, not decisions. Revisit only after product value, technical feasibility and current distribution rules are understood.

## 8. What success looks like

- Users can find common system capabilities without knowing which separate utility owns them.
- Information is understandable at a glance and deeper details are available when wanted.
- Actions are explained and safe; no misleading claims about what the system contains or what an operation did.
- The app feels consistent and polished rather than stitched together.
- Features work on the actual target device, not just in a mock-up or on a developer Mac.
- The product can grow without committing V1 to every long-term idea.
