use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error {
    pub line: usize,
    pub kind: ErrorKind,
}

impl Error {
    fn new_from_0_indexed_line(line: usize, kind: ErrorKind) -> Self {
        Self {
            // Convert from 0-indexed to 1-indexed
            line: line + 1,
            kind,
        }
    }

    pub(crate) fn invalid_id(line: usize) -> Self {
        Self::new_from_0_indexed_line(line, ErrorKind::InvalidId)
    }

    pub(crate) fn missing_ts_line(line: usize) -> Self {
        Self::new_from_0_indexed_line(line, ErrorKind::MissingTimestampLine)
    }

    pub(crate) fn invalid_ts_start(line: usize) -> Self {
        Self::new_from_0_indexed_line(line, ErrorKind::InvalidTimestampStart)
    }

    pub(crate) fn invalid_ts_divider(line: usize) -> Self {
        Self::new_from_0_indexed_line(line, ErrorKind::InvalidTimestampDivider)
    }

    pub(crate) fn invalid_ts_end(line: usize) -> Self {
        Self::new_from_0_indexed_line(line, ErrorKind::InvalidTimestampEnd)
    }

    pub(crate) fn ts_end_before_start(line: usize) -> Self {
        Self::new_from_0_indexed_line(line, ErrorKind::TimestampEndBeforeStart)
    }

    pub(crate) fn missing_text(line: usize) -> Self {
        Self::new_from_0_indexed_line(line, ErrorKind::MissingText)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on line {}", self.kind, self.line)
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidId,
    MissingTimestampLine,
    InvalidTimestampStart,
    InvalidTimestampDivider,
    InvalidTimestampEnd,
    TimestampEndBeforeStart,
    MissingText,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidId => "Invalid ID-marker",
            Self::MissingTimestampLine => "Missing timestamp line",
            Self::InvalidTimestampStart => "Invalid start timestamp",
            Self::InvalidTimestampDivider => "Invalid timestamp divider",
            Self::InvalidTimestampEnd => "Invalid end timestamp",
            Self::TimestampEndBeforeStart => "End timestamp is before start",
            Self::MissingText => "Missing text section",
        })
    }
}
