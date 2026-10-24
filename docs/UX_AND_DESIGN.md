# UX and Design Direction

## Overall direction

The user likes the horizontal top-navigation layout from a reference screenshot, adapted to a darker SteamOS-inspired interface. This is a visual direction, not a final design system.

Desired qualities:
- Dark, cohesive and polished.
- Functional and information-rich without becoming a wall of data.
- Clear, simple and familiar, with advanced detail available when wanted.
- Visually appealing without being flashy.
- Consistent across all sections.
- Suitable for users who are new to Linux as well as technical users.

## Main shell

- Persistent horizontal navigation across the top.
- Understated blue underline to indicate the active section.
- Main navigation concepts discussed: Home, System, Files, Applications, Updates, Customisation. Names and placement remain provisional.
- Contextual sidebar changes with the selected section when it helps navigation.
- Main content area provides clear headings, useful information and straightforward actions.
- Notifications sit near the main navigation and show only relevant current items.
- Avoid bulky buttons or heavy selected-tab boxes when a subtle active indicator suffices.

## Home dashboard

- A system overview dashboard, not a welcome page.
- Show meaningful, accurate at-a-glance information.
- Do not build generic cards merely to fill space.
- Keep sections one click away through persistent navigation.

## Task Manager interaction model

- First implementation slice: a holistic, system-wide resource overview. Add the per-process list in a later increment, so the whole-machine status is visible before we explain individual contributors.
- Overview metrics: CPU capacity utilization, memory used/total, GPU activity/memory, disk read/write activity, and network receive/send activity. Use units suited to each metric rather than forcing every reading into a percentage.
- Search and sorting are easy to find.
- When the later process view is added, show application/process name, CPU and memory readings.
- Technical details such as PID and executable/command information appear on demand.
- Distinguish application concepts from individual Linux processes when the data allows it; do not imply one process always equals one app.
- Use clear labels and units; explain unavailable values.
- CPU percentages represent share of total online logical-CPU time capacity over a sample interval, normalized to 0–100%; do not show the per-logical-CPU convention as the primary user-facing number.
- Keep unavailable values distinct from zero.
- Refresh should be smooth and not cause avoidable visual jitter.
- Destructive controls are not part of the first read-only milestone.

## Storage/cleanup interaction principles for later

Explain what was found, what is known or unknown, measured size, confidence, possible side effects, reversibility and exact items proposed for action. Distinguish observed device facts from general platform knowledge. Preview changes and let the user choose. Do not use generic disclaimers in place of a specific explanation.

## Notifications

- Relevant-now only; no nagging.
- Resolved items leave the active list.
- Dismissal does not mean the underlying issue is resolved.
- Underlying device state must remain accurate.
- History is undecided.

## Visual refinement

The dark mock-up discussed previously was illustrative only, not an implemented UI and not live device data. As functionality develops, compare and refine typography, spacing, icons, colour, depth and hierarchy. The user explicitly wants the option to make the design more visually appealing, so do not treat the first mock-up as final.
