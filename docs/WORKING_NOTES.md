# Working Notes and Conversation Context

This is a curated summary, not a verbatim transcript.

## How the idea evolved
- The core problem is fragmentation across existing SteamOS/Linux tools and settings.
- The ambition became a unified control centre, not merely a cleaner utility.
- The owner wants a smaller, coherent first release that can grow.
- Task Manager was selected as the first feature to prove.
- Task Manager style selected: balanced — friendly by default, technical detail on demand.
- First milestone selected: read-only monitoring. Process termination comes later with safeguards.
- Visual reference: user supplied a screenshot of a banking interface and liked the horizontal top banner/navigation and understated active underline. The chosen adaptation is darker and SteamOS-inspired. The HTML mock-up was illustrative; the initial React overview now mirrors its direction but still has no live readings.
- The owner said the visual could be made more appealing later; keep the design open to iteration.

## Specific product principles from discussion
- Storage cleanup must explain findings specifically: exact path/items, measured size, what is known/unknown, purpose, potential consequences and reversibility. Do not make unsupported claims about directory contents.
- Do not label everything “junk”, and do not delete solely based on age or size.
- The app inventory should be unified by default; app install/store is not in current scope; no Update All button.
- Respect SteamOS's update mechanism.
- Unify existing KDE/SteamOS desktop customisation before inventing new capabilities.
- Remote companion management is a future possibility, not a current implementation requirement.
- User prefers a polished, consistent interface, with relevant-only notifications and no nagging.
- A useful Home system dashboard is preferred over a screen that changes based on the last-used section.
- Ordinary UI preferences may eventually be remembered, but a past confirmation must never be treated as blanket approval for destructive actions.

## Development approach
- The owner knows VS Code and Git at a basic level, is new to Linux development and ChatGPT coding workflows, and wants to learn Rust through reviewing useful code.
- Use Codex as a coding agent, but keep the owner in the loop.
- Superpowers was discussed as a possible planning/TDD workflow framework. It is optional; do not add process overhead just for its own sake.
- Potential monorepo outline was discussed, but it is a target idea, not a requirement to create every directory at the beginning:
  - apps/desktop/
  - packages/ (only as needed)
  - crates/ (only as needed)
  - tests/
  - docs/
  - scripts/
  - Cargo workspace and JS workspace if justified
- The first task should be a working vertical slice, not an oversized scaffold.
