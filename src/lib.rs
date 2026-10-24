use std::{collections::HashSet, ffi::OsStr, time::Instant};

use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

mod memory;

pub use memory::{SystemMemorySnapshot, read_system_memory};

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage_percent: Option<f32>,
    pub memory_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ProcessIdentity {
    pid: u32,
    start_time_seconds: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessMonitorError {
    UnsupportedSystem,
}

impl std::fmt::Display for ProcessMonitorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedSystem => {
                formatter.write_str("process monitoring is unsupported on this system")
            }
        }
    }
}

impl std::error::Error for ProcessMonitorError {}

pub struct ProcessMonitor {
    system: System,
    last_cpu_refresh_at: Option<Instant>,
    cpu_sampled_processes: HashSet<ProcessIdentity>,
}

impl ProcessMonitor {
    pub fn new() -> Result<Self, ProcessMonitorError> {
        if !sysinfo::IS_SUPPORTED_SYSTEM {
            return Err(ProcessMonitorError::UnsupportedSystem);
        }

        Ok(Self {
            system: System::new(),
            last_cpu_refresh_at: None,
            cpu_sampled_processes: HashSet::new(),
        })
    }

    pub fn snapshot(&mut self) -> Vec<ProcessInfo> {
        let should_refresh_cpu = cpu_refresh_is_due(self.last_cpu_refresh_at, Instant::now());
        let refresh_kind = ProcessRefreshKind::nothing().with_memory().without_tasks();
        let refresh_kind = if should_refresh_cpu {
            refresh_kind.with_cpu()
        } else {
            refresh_kind
        };

        self.system
            .refresh_processes_specifics(ProcessesToUpdate::All, true, refresh_kind);

        let mut processes = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let pid = pid.as_u32();
                let identity = process_identity(pid, process.start_time());
                let cpu_usage_percent = self
                    .cpu_sampled_processes
                    .contains(&identity)
                    .then(|| process.cpu_usage());

                process_info(pid, process.name(), cpu_usage_percent, process.memory())
            })
            .collect::<Vec<_>>();

        sort_processes_by_pid(&mut processes);

        if should_refresh_cpu {
            self.cpu_sampled_processes = self
                .system
                .processes()
                .iter()
                .map(|(pid, process)| process_identity(pid.as_u32(), process.start_time()))
                .collect();
            self.last_cpu_refresh_at = Some(Instant::now());
        }

        processes
    }
}

fn process_identity(pid: u32, start_time_seconds: u64) -> ProcessIdentity {
    ProcessIdentity {
        pid,
        start_time_seconds,
    }
}

fn cpu_refresh_is_due(last_cpu_refresh_at: Option<Instant>, now: Instant) -> bool {
    match last_cpu_refresh_at {
        None => true,
        Some(last_cpu_refresh_at) => {
            now.saturating_duration_since(last_cpu_refresh_at)
                >= sysinfo::MINIMUM_CPU_UPDATE_INTERVAL
        }
    }
}

fn process_info(
    pid: u32,
    name: &OsStr,
    cpu_usage_percent: Option<f32>,
    memory_bytes: u64,
) -> ProcessInfo {
    ProcessInfo {
        pid,
        name: name.to_string_lossy().into_owned(),
        cpu_usage_percent,
        memory_bytes,
    }
}

fn sort_processes_by_pid(processes: &mut [ProcessInfo]) {
    processes.sort_by_key(|process| process.pid);
}

#[cfg(test)]
mod tests {
    use std::{
        ffi::OsStr,
        time::{Duration, Instant},
    };

    use super::{cpu_refresh_is_due, process_identity, process_info, sort_processes_by_pid};

    #[test]
    fn first_cpu_refresh_establishes_a_baseline() {
        assert!(cpu_refresh_is_due(None, Instant::now()));
    }

    #[test]
    fn cpu_refresh_waits_for_sysinfo_minimum_interval() {
        let last_refresh = Instant::now();
        let minimum_interval = sysinfo::MINIMUM_CPU_UPDATE_INTERVAL;

        assert!(cpu_refresh_is_due(
            Some(last_refresh),
            last_refresh + minimum_interval
        ));

        if minimum_interval > Duration::ZERO {
            let just_before_interval = minimum_interval - Duration::from_nanos(1);
            assert!(!cpu_refresh_is_due(
                Some(last_refresh),
                last_refresh + just_before_interval
            ));
        }
    }

    #[test]
    fn process_info_preserves_unavailable_cpu_and_memory_bytes() {
        let process = process_info(42, OsStr::new("steam"), None, 1_572_864);

        assert_eq!(process.pid, 42);
        assert_eq!(process.name, "steam");
        assert_eq!(process.cpu_usage_percent, None);
        assert_eq!(process.memory_bytes, 1_572_864);
    }

    #[test]
    fn process_snapshots_sort_by_pid() {
        let mut processes = vec![
            process_info(9, OsStr::new("game"), Some(2.0), 200),
            process_info(3, OsStr::new("steam"), Some(1.0), 100),
        ];

        sort_processes_by_pid(&mut processes);

        assert_eq!(processes[0].pid, 3);
        assert_eq!(processes[1].pid, 9);
    }

    #[test]
    fn reusing_a_pid_with_a_new_start_time_requires_a_new_cpu_sample() {
        let sampled_process = process_identity(42, 100);
        let replacement_process = process_identity(42, 101);
        let sampled_processes = std::collections::HashSet::from([sampled_process]);

        assert!(!sampled_processes.contains(&replacement_process));
    }
}
