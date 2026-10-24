use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemMemorySnapshot {
    total_bytes: u64,
    available_bytes: u64,
}

impl SystemMemorySnapshot {
    pub const fn total_bytes(self) -> u64 {
        self.total_bytes
    }

    pub const fn available_bytes(self) -> u64 {
        self.available_bytes
    }

    pub const fn used_bytes(self) -> u64 {
        self.total_bytes - self.available_bytes
    }
}

pub fn read_system_memory() -> io::Result<SystemMemorySnapshot> {
    #[cfg(target_os = "linux")]
    {
        let contents = std::fs::read_to_string("/proc/meminfo")?;
        parse_meminfo(&contents).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
    }

    #[cfg(not(target_os = "linux"))]
    {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "system memory monitoring is supported on Linux only",
        ))
    }
}

#[cfg(any(target_os = "linux", test))]
fn parse_meminfo(contents: &str) -> Result<SystemMemorySnapshot, MemInfoParseError> {
    let total_bytes = parse_meminfo_kib(contents, "MemTotal")?;
    let available_bytes = parse_meminfo_kib(contents, "MemAvailable")?;

    if available_bytes > total_bytes {
        return Err(MemInfoParseError::AvailableExceedsTotal);
    }

    Ok(SystemMemorySnapshot {
        total_bytes,
        available_bytes,
    })
}

#[cfg(any(target_os = "linux", test))]
fn parse_meminfo_kib(contents: &str, key: &'static str) -> Result<u64, MemInfoParseError> {
    let value = contents
        .lines()
        .find_map(|line| {
            let (field, value) = line.split_once(':')?;
            (field.trim() == key).then_some(value)
        })
        .ok_or(MemInfoParseError::MissingField(key))?;

    let mut parts = value.split_whitespace();
    let amount = parts
        .next()
        .ok_or(MemInfoParseError::InvalidField(key))?
        .parse::<u64>()
        .map_err(|_| MemInfoParseError::InvalidField(key))?;
    let unit = parts.next().ok_or(MemInfoParseError::InvalidField(key))?;

    if unit != "kB" {
        return Err(MemInfoParseError::InvalidField(key));
    }

    amount
        .checked_mul(1024)
        .ok_or(MemInfoParseError::InvalidField(key))
}

#[derive(Debug, PartialEq, Eq)]
#[cfg(any(target_os = "linux", test))]
enum MemInfoParseError {
    MissingField(&'static str),
    InvalidField(&'static str),
    AvailableExceedsTotal,
}

#[cfg(any(target_os = "linux", test))]
impl std::fmt::Display for MemInfoParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(field) => write!(formatter, "missing {field} in /proc/meminfo"),
            Self::InvalidField(field) => write!(formatter, "invalid {field} in /proc/meminfo"),
            Self::AvailableExceedsTotal => {
                formatter.write_str("MemAvailable exceeds MemTotal in /proc/meminfo")
            }
        }
    }
}

#[cfg(any(target_os = "linux", test))]
impl std::error::Error for MemInfoParseError {}

#[cfg(test)]
mod tests {
    use super::{MemInfoParseError, SystemMemorySnapshot, parse_meminfo, read_system_memory};

    const MEMINFO_FIXTURE: &str =
        "MemFree: 200 kB\nMemAvailable: 256 kB\nBuffers: 12 kB\nMemTotal: 1024 kB\n";

    #[test]
    fn parses_total_and_available_bytes_and_derives_used_memory() {
        let snapshot = parse_meminfo(MEMINFO_FIXTURE).unwrap();

        assert_eq!(
            snapshot,
            SystemMemorySnapshot {
                total_bytes: 1_048_576,
                available_bytes: 262_144,
            }
        );
        assert_eq!(snapshot.used_bytes(), 786_432);
    }

    #[test]
    fn reports_missing_available_memory_instead_of_assuming_zero() {
        let error = parse_meminfo("MemTotal: 1024 kB\n").unwrap_err();

        assert_eq!(error, MemInfoParseError::MissingField("MemAvailable"));
    }

    #[test]
    fn rejects_invalid_units_and_values() {
        let wrong_unit = parse_meminfo("MemTotal: 1 MB\nMemAvailable: 1 kB\n");
        let invalid_value = parse_meminfo("MemTotal: many kB\nMemAvailable: 1 kB\n");

        assert_eq!(wrong_unit, Err(MemInfoParseError::InvalidField("MemTotal")));
        assert_eq!(
            invalid_value,
            Err(MemInfoParseError::InvalidField("MemTotal"))
        );
    }

    #[test]
    fn rejects_available_memory_above_total_memory() {
        let error = parse_meminfo("MemTotal: 100 kB\nMemAvailable: 101 kB\n").unwrap_err();

        assert_eq!(error, MemInfoParseError::AvailableExceedsTotal);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn reads_a_real_linux_memory_snapshot() {
        let snapshot = read_system_memory().unwrap();

        assert!(snapshot.total_bytes > 0);
        assert!(snapshot.available_bytes <= snapshot.total_bytes);
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn reports_linux_only_reader_as_unsupported_elsewhere() {
        let error = read_system_memory().unwrap_err();

        assert_eq!(error.kind(), std::io::ErrorKind::Unsupported);
    }
}
