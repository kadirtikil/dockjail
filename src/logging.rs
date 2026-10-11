//! Timestamped log lines for the login agent, whose stderr ends up in the log file.

use std::fmt::Arguments;

/// Writes one line to stderr, prefixed with the current UTC time.
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logging::write(format_args!($($arg)*))
    };
}
pub(crate) use log;

pub fn write(message: Arguments) {
    eprintln!("{} {message}", timestamp(unsafe { libc::time(std::ptr::null_mut()) }));
}

/// Formats Unix seconds as ISO 8601 in UTC, e.g. `2026-10-11T14:03:22Z`.
fn timestamp(seconds: libc::time_t) -> String {
    let mut parts: libc::tm = unsafe { std::mem::zeroed() };
    unsafe { libc::gmtime_r(&seconds, &mut parts) };
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        parts.tm_year + 1900,
        parts.tm_mon + 1,
        parts.tm_mday,
        parts.tm_hour,
        parts.tm_min,
        parts.tm_sec
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_unix_seconds_as_utc() {
        assert_eq!(timestamp(0), "1970-01-01T00:00:00Z");
        assert_eq!(timestamp(1_791_727_402), "2026-10-11T14:03:22Z");
    }
}
