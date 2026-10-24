# Current Task — Milestone 1: Read-only Task Manager

**Status:** In progress — the overview reads system memory from Linux `/proc/meminfo`; CPU, GPU, disk and network are not connected. SteamOS device validation remains outstanding.

## Objective

Deliver a read-only Task Manager for the target SteamOS device. Begin with a truthful overview of whole-machine resource use. Add per-process attribution later, after the system-wide readings and their presentation are validated.

## Agreed implementation order

1. **System overview first:** show machine-wide CPU, memory, GPU, disk and network readings.
2. **Per-process breakdown later:** show which processes contribute to resource use, using clearly defined units and attribution semantics.

CPU percentage means the process or system's share of total online logical-CPU time capacity over a sampling interval, normalized to 0–100%. It does not mean percentage of one logical CPU. The sample interval and treatment of CPU idle/I/O-wait accounting must be documented and validated.

## System overview requirements

- CPU: total machine utilization as a percentage of total CPU time capacity.
- Memory: used and total physical memory, with available memory shown or explained; define the Linux accounting convention rather than treating every non-free page as unavailable.
- GPU: overall GPU activity and GPU-memory use/capacity where the target driver exposes reliable values. Keep dedicated and shared memory distinct unless their accounting can be combined without misleading users.
- Disk: system-level read/write activity rates, with device aggregation defined so stacked devices or partitions are not double-counted.
- Network: system-level receive/send rates, with interface aggregation defined and no claim that local interface traffic equals internet usage or ISP bandwidth.
- For every metric, provide a timestamp/sample state and distinguish unavailable or unsupported readings from zero.
- Remain read-only, run as the normal user, and keep sampling off the UI thread.

Linux documentation describes candidate system interfaces, but does not prove that a particular reading is exposed or accurate on the target SteamOS kernel, firmware and hardware. Validate on Linux and then on the actual SteamOS device before claiming support.

## Deferred within this milestone

- Running-process list with name, PID, per-process CPU and memory.
- Search, sorting and process details.
- Process-to-application grouping where the data supports it.
- Per-process GPU, disk or network attribution.

Do not implement process termination, priority changes, services, root/privileged execution, remote management, or advanced temperature/fan/power controls in this milestone.

## Acceptance criteria for the first system-overview slice

1. A Linux build can be produced using a documented process, and the app launches on the target SteamOS device.
2. The overview displays real CPU, memory, GPU, disk and network readings where supported, with explicit units, sampling conventions and unavailable states.
3. CPU utilization is normalized to total online logical-CPU time capacity and stays within the intended 0–100% scale.
4. Memory labels and the relationship between total, used and available memory are defined.
5. Sampling does not freeze the UI or introduce avoidable instability.
6. Automated tests cover metric parsing, calculations, counter resets and unavailable/error paths.
7. The owner receives a plain-language explanation of the code, checks performed and remaining hardware-specific uncertainty.

## Existing implementation

The repository contains a Rust library that reads per-process ID, name, resident memory and `sysinfo` CPU usage. It also contains a Tauri + React overview screen with a Linux system-memory reader backed by `/proc/meminfo`. The UI shows used, total and available physical RAM, and lets the user request a fresh snapshot. On non-Linux systems the reader reports unsupported; it does not fabricate a value. CPU, GPU, disk and network remain unavailable. The process CPU metric is not the agreed whole-machine percentage. Keep the process library as later groundwork; do not treat it as the first overview slice or as proof of SteamOS compatibility.

For target-device testing, use `.agents/skills/steam-machine-validation/SKILL.md`. Record each test handoff and result on a GitHub Issue.
