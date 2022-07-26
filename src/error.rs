use std::fmt;

/// A specialized [`Result`][std::result::Result] for [`kiss_srt::Error`][Error]
pub type Result<T> = std::result::Result<T, Error>;

/// Contains context on why parsing failed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error {
    /// The line number that failed parsing
    pub line: usize,
    /// The kind of failure
    pub kind: ErrorKind,
}

impl Error {
    pub(crate) fn invalid_id(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::InvalidId,
        }
    }

    pub(crate) fn invalid_ts_line(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::InvalidTimestampLine,
        }
    }

    pub(crate) fn invalid_ts_start(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::InvalidTimestampStart,
        }
    }

    pub(crate) fn invalid_ts_divider(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::InvalidTimestampDivider,
        }
    }

    pub(crate) fn invalid_ts_end(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::InvalidTimestampEnd,
        }
    }

    pub(crate) fn ts_end_before_start(line: usize) -> Self {
        Self {
            line,
            kind: ErrorKind::TimestampEndBeforeStart,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on line {}", self.kind, self.line)
    }
}

impl std::error::Error for Error {}

/// Describes the kind of failure
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// The id contained something other than an ASCII digit
    InvalidId,
    /// The timestamp line was either missing or had trailing bytes
    InvalidTimestampLine,
    /// The starting timestamp doesn't match the format of '01:23:45,678'
    InvalidTimestampStart,
    /// The timestamp divider doesn't match ' --> '
    InvalidTimestampDivider,
    /// The ending timestamp doesn't match the format of '01:23:45,678'
    InvalidTimestampEnd,
    /// The ending timestamp is before the start
    TimestampEndBeforeStart,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidId => "Invalid ID-marker",
            Self::InvalidTimestampLine => "Invalid timestamp line",
            Self::InvalidTimestampStart => "Invalid starting timestamp",
            Self::InvalidTimestampDivider => "Invalid timestamp divider",
            Self::InvalidTimestampEnd => "Invalid ending timestamp",
            Self::TimestampEndBeforeStart => "End timestamp is before start",
        })
    }
}
